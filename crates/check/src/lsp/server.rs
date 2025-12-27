//! LSP Server Implementation
//!
//! Main language server using tower-lsp for async handling.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use parking_lot::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::config::CheckerConfig;
use crate::diagnostics::{Diagnostic as DxDiagnostic, DiagnosticSeverity as DxSeverity, LineIndex};
use crate::engine::Checker;
use crate::rules::dxs_parser::load_dxs_directory;
use crate::rules::schema::DxRule;

use super::LspConfig;

/// dx-check Language Server
pub struct DxCheckLanguageServer {
    /// LSP client for sending notifications
    client: Client,
    /// Checker engine instance
    checker: Arc<RwLock<Option<Checker>>>,
    /// Loaded rules (indexed by name)
    rules: Arc<RwLock<HashMap<String, DxRule>>>,
    /// Document content cache
    documents: Arc<RwLock<HashMap<Url, String>>>,
    /// Server configuration
    config: Arc<RwLock<LspConfig>>,
    /// Workspace root
    workspace_root: Arc<RwLock<Option<PathBuf>>>,
}

impl DxCheckLanguageServer {
    /// Create a new language server instance
    pub fn new(client: Client) -> Self {
        Self {
            client,
            checker: Arc::new(RwLock::new(None)),
            rules: Arc::new(RwLock::new(HashMap::new())),
            documents: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(LspConfig::default())),
            workspace_root: Arc::new(RwLock::new(None)),
        }
    }

    /// Load rules from .dxs files
    async fn load_rules(&self, rules_dir: PathBuf) {
        let load_result = load_dxs_directory(&rules_dir);
        
        match load_result {
            Ok(database) => {
                let rule_count = {
                    let mut rules_map = self.rules.write();
                    rules_map.clear();
                    for rule in database.rules {
                        rules_map.insert(rule.name.clone(), rule);
                    }
                    rules_map.len()
                };
                self.client
                    .log_message(
                        MessageType::INFO,
                        format!("Loaded {} rules from {:?}", rule_count, rules_dir),
                    )
                    .await;
            }
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Failed to load rules: {}", e))
                    .await;
            }
        }
    }

    /// Publish diagnostics for a document
    async fn publish_diagnostics(&self, uri: Url) {
        let content = {
            let docs = self.documents.read();
            docs.get(&uri).cloned()
        };

        let Some(content) = content else { return };

        // Get file extension to determine language
        let extension = uri
            .path()
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();

        // Only lint supported file types
        if !matches!(
            extension.as_str(),
            "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs"
        ) {
            return;
        }

        // Run checker
        let diagnostics = self.lint_content(&content, &uri);

        // Convert to LSP diagnostics
        let lsp_diagnostics: Vec<Diagnostic> = diagnostics
            .into_iter()
            .map(|d| self.to_lsp_diagnostic(&d, &content))
            .collect();

        self.client
            .publish_diagnostics(uri, lsp_diagnostics, None)
            .await;
    }

    /// Lint content and return diagnostics
    fn lint_content(&self, content: &str, uri: &Url) -> Vec<DxDiagnostic> {
        let checker = self.checker.read();
        if let Some(checker) = checker.as_ref() {
            // Convert URI to path
            let path = uri.to_file_path().unwrap_or_else(|_| PathBuf::from("temp.ts"));
            checker.check_source(&path, content).unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    /// Convert dx-check diagnostic to LSP diagnostic
    fn to_lsp_diagnostic(&self, diag: &DxDiagnostic, source: &str) -> Diagnostic {
        let severity = match diag.severity {
            DxSeverity::Error => DiagnosticSeverity::ERROR,
            DxSeverity::Warning => DiagnosticSeverity::WARNING,
            DxSeverity::Info => DiagnosticSeverity::INFORMATION,
            DxSeverity::Hint => DiagnosticSeverity::HINT,
        };

        // Convert byte offsets to line/column
        let line_index = LineIndex::new(source);
        let (start_lc, end_lc) = diag.span.to_line_col(&line_index);

        let range = Range {
            start: Position {
                line: start_lc.line.saturating_sub(1), // LSP is 0-indexed
                character: start_lc.col.saturating_sub(1),
            },
            end: Position {
                line: end_lc.line.saturating_sub(1),
                character: end_lc.col.saturating_sub(1),
            },
        };

        Diagnostic {
            range,
            severity: Some(severity),
            code: Some(NumberOrString::String(diag.rule_id.clone())),
            code_description: None,
            source: Some("dx-check".to_string()),
            message: diag.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        }
    }

    /// Get rule by ID for hover documentation
    #[allow(dead_code)]
    fn get_rule_info(&self, rule_id: &str) -> Option<String> {
        let rules = self.rules.read();
        rules.get(rule_id).map(|rule| {
            let mut info = format!("## {}\n\n{}", rule.name, rule.description);
            if let Some(ref url) = rule.docs_url {
                info.push_str(&format!("\n\n[Documentation]({})", url));
            }
            if rule.fixable {
                info.push_str("\n\n✅ **Auto-fixable**");
            }
            info
        })
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for DxCheckLanguageServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Store workspace root
        if let Some(root) = params.root_uri {
            if let Ok(path) = root.to_file_path() {
                *self.workspace_root.write() = Some(path.clone());

                // Try to load rules from workspace/rules or workspace/.dx-check/rules
                let rules_dir = path.join("rules");
                if rules_dir.exists() {
                    self.load_rules(rules_dir).await;
                } else {
                    let dx_rules_dir = path.join(".dx-check").join("rules");
                    if dx_rules_dir.exists() {
                        self.load_rules(dx_rules_dir).await;
                    }
                }
            }
        }

        // Initialize checker engine
        {
            let config = CheckerConfig::default();
            let checker = Checker::new(config);
            *self.checker.write() = Some(checker);
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Options(
                    CodeActionOptions {
                        code_action_kinds: Some(vec![CodeActionKind::QUICKFIX]),
                        resolve_provider: Some(false),
                        ..Default::default()
                    },
                )),
                document_formatting_provider: Some(OneOf::Left(true)),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("dx-check".to_string()),
                        inter_file_dependencies: false,
                        workspace_diagnostics: false,
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "dx-check".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "dx-check LSP server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        {
            let mut docs = self.documents.write();
            docs.insert(uri.clone(), params.text_document.text);
        }
        self.publish_diagnostics(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Some(change) = params.content_changes.into_iter().next() {
            {
                let mut docs = self.documents.write();
                docs.insert(uri.clone(), change.text);
            }
            self.publish_diagnostics(uri).await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        if let Some(text) = params.text {
            let uri = params.text_document.uri.clone();
            {
                let mut docs = self.documents.write();
                docs.insert(uri.clone(), text);
            }
            self.publish_diagnostics(uri).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        {
            let mut docs = self.documents.write();
            docs.remove(&uri);
        }
        // Clear diagnostics
        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        // Get diagnostics at position and show rule documentation
        let _uri = params.text_document_position_params.text_document.uri;
        let _position = params.text_document_position_params.position;

        // Find diagnostic at position
        // For now, return generic hover info
        // TODO: Implement proper hover based on diagnostics at position

        Ok(None)
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let _uri = params.text_document.uri;
        let _range = params.range;

        // Find diagnostics in range that have fixes
        let mut actions = Vec::new();

        for diag in &params.context.diagnostics {
            if let Some(NumberOrString::String(rule_id)) = &diag.code {
                let rules = self.rules.read();
                if let Some(rule) = rules.get(rule_id) {
                    if rule.fixable {
                        actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                            title: format!("Fix: {} ({})", rule.name, rule_id),
                            kind: Some(CodeActionKind::QUICKFIX),
                            diagnostics: Some(vec![diag.clone()]),
                            edit: None, // TODO: Compute actual fix
                            command: None,
                            is_preferred: Some(true),
                            disabled: None,
                            data: None,
                        }));
                    }
                }
            }
        }

        if actions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(actions))
        }
    }

    async fn formatting(&self, _params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        // TODO: Implement formatting using dx-check formatter
        Ok(None)
    }
}

/// Start the LSP server on stdin/stdout
pub async fn start_lsp_server() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(DxCheckLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}
