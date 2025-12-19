import * as vscode from 'vscode';
import * as net from 'net';

// ============================================================================
// DX Forge Extension - Binary Dawn Edition
// ============================================================================

interface DxNotification {
    jsonrpc: string;
    method: string;
    params: Record<string, unknown>;
}

interface DxToolStatus {
    tool: string;
    status: 'running' | 'success' | 'failed';
    duration_ms?: number;
}

// ============================================================================
// FORGE CLIENT
// ============================================================================

class ForgeClient {
    private socket: net.Socket | null = null;
    private connected = false;
    private outputChannel: vscode.OutputChannel;
    private statusBar: vscode.StatusBarItem;
    private buffer = '';

    constructor(
        private port: number,
        private onNotification: (notification: DxNotification) => void
    ) {
        this.outputChannel = vscode.window.createOutputChannel('DX Forge');
        this.statusBar = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Left,
            100
        );
        this.statusBar.text = '$(debug-disconnect) DX Forge';
        this.statusBar.tooltip = 'DX Forge Daemon - Disconnected';
        this.statusBar.show();
    }

    async connect(): Promise<boolean> {
        return new Promise((resolve) => {
            this.socket = new net.Socket();

            this.socket.on('connect', () => {
                this.connected = true;
                this.statusBar.text = '$(check) DX Forge';
                this.statusBar.tooltip = 'DX Forge Daemon - Connected';
                this.statusBar.backgroundColor = undefined;
                this.outputChannel.appendLine(`Connected to Forge Daemon on port ${this.port}`);
                resolve(true);
            });

            this.socket.on('data', (data) => {
                this.buffer += data.toString();
                this.processBuffer();
            });

            this.socket.on('error', (err) => {
                this.outputChannel.appendLine(`Connection error: ${err.message}`);
                this.connected = false;
                this.updateDisconnectedStatus();
                resolve(false);
            });

            this.socket.on('close', () => {
                this.connected = false;
                this.updateDisconnectedStatus();
                this.outputChannel.appendLine('Disconnected from Forge Daemon');
            });

            this.socket.connect(this.port, '127.0.0.1');
        });
    }

    private updateDisconnectedStatus(): void {
        this.statusBar.text = '$(debug-disconnect) DX Forge';
        this.statusBar.tooltip = 'DX Forge Daemon - Disconnected';
        this.statusBar.backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
    }

    private processBuffer(): void {
        const lines = this.buffer.split('\n');
        this.buffer = lines.pop() || '';

        for (const line of lines) {
            if (line.trim()) {
                try {
                    const notification = JSON.parse(line) as DxNotification;
                    this.onNotification(notification);
                } catch {
                    this.outputChannel.appendLine(`Invalid JSON: ${line}`);
                }
            }
        }
    }

    send(notification: DxNotification): void {
        if (this.socket && this.connected) {
            this.socket.write(JSON.stringify(notification) + '\n');
        }
    }

    sendFileChange(uri: string, content: string): void {
        this.send({
            jsonrpc: '2.0',
            method: 'textDocument/didChange',
            params: {
                textDocument: { uri, version: 1 },
                contentChanges: [{ text: content }]
            }
        });
    }

    disconnect(): void {
        this.socket?.destroy();
        this.socket = null;
        this.connected = false;
    }

    isConnected(): boolean {
        return this.connected;
    }

    dispose(): void {
        this.disconnect();
        this.outputChannel.dispose();
        this.statusBar.dispose();
    }
}

// ============================================================================
// EXTENSION
// ============================================================================

let client: ForgeClient | null = null;
let toolStatusBar: vscode.StatusBarItem;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
    console.log('DX Forge extension activating...');

    // Create tool status bar
    toolStatusBar = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Left,
        99
    );
    context.subscriptions.push(toolStatusBar);

    // Get configuration
    const config = vscode.workspace.getConfiguration('dx');
    const port = config.get<number>('daemonPort', 9527);
    const autoStart = config.get<boolean>('autoStart', true);

    // Create client
    client = new ForgeClient(port, handleNotification);
    context.subscriptions.push({ dispose: () => client?.dispose() });

    // Register commands
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.startDaemon', startDaemon),
        vscode.commands.registerCommand('dx.stopDaemon', stopDaemon),
        vscode.commands.registerCommand('dx.runTool', runTool),
        vscode.commands.registerCommand('dx.warmCache', warmCache),
        vscode.commands.registerCommand('dx.clearCache', clearCache),
        vscode.commands.registerCommand('dx.showStats', showStats)
    );

    // Watch for document changes
    if (config.get<boolean>('enableLspWatcher', true)) {
        context.subscriptions.push(
            vscode.workspace.onDidChangeTextDocument((event) => {
                if (client?.isConnected()) {
                    const uri = event.document.uri.toString();
                    const content = event.document.getText();
                    client.sendFileChange(uri, content);
                }
            })
        );

        context.subscriptions.push(
            vscode.workspace.onDidSaveTextDocument((document) => {
                if (client?.isConnected()) {
                    client.send({
                        jsonrpc: '2.0',
                        method: 'textDocument/didSave',
                        params: {
                            textDocument: { uri: document.uri.toString() }
                        }
                    });
                }
            })
        );
    }

    // Auto-connect
    if (autoStart) {
        await client.connect();
    }

    console.log('DX Forge extension activated');
}

function handleNotification(notification: DxNotification): void {
    switch (notification.method) {
        case 'dx/toolStarted': {
            const params = notification.params as { tool: string };
            toolStatusBar.text = `$(sync~spin) ${params.tool}`;
            toolStatusBar.show();
            break;
        }

        case 'dx/toolCompleted': {
            const params = notification.params as DxToolStatus;
            const icon = params.status === 'success' ? '$(check)' : '$(error)';
            const duration = params.duration_ms ? ` (${params.duration_ms}ms)` : '';
            toolStatusBar.text = `${icon} ${params.tool}${duration}`;
            setTimeout(() => toolStatusBar.hide(), 3000);
            break;
        }

        case 'dx/patternDetected': {
            // Handle pattern detection
            break;
        }

        case 'dx/cacheStatus': {
            // Handle cache status
            break;
        }

        default:
            console.log('Unknown notification:', notification.method);
    }
}

async function startDaemon(): Promise<void> {
    if (client?.isConnected()) {
        vscode.window.showInformationMessage('Forge Daemon is already connected');
        return;
    }

    const connected = await client?.connect();
    if (connected) {
        vscode.window.showInformationMessage('Connected to Forge Daemon');
    } else {
        vscode.window.showErrorMessage(
            'Failed to connect to Forge Daemon. Make sure it is running.'
        );
    }
}

function stopDaemon(): void {
    client?.disconnect();
    vscode.window.showInformationMessage('Disconnected from Forge Daemon');
}

async function runTool(): Promise<void> {
    const tools = [
        { label: 'bundler', description: 'JavaScript/TypeScript bundler' },
        { label: 'style', description: 'Binary CSS compiler' },
        { label: 'test', description: 'Test runner' },
        { label: 'www', description: 'HTIP framework' },
        { label: 'package-manager', description: 'npm package manager' }
    ];

    const selected = await vscode.window.showQuickPick(tools, {
        placeHolder: 'Select a tool to run'
    });

    if (selected && client?.isConnected()) {
        client.send({
            jsonrpc: '2.0',
            method: 'dx/runTool',
            params: { tool: selected.label }
        });
    }
}

function warmCache(): void {
    if (client?.isConnected()) {
        client.send({
            jsonrpc: '2.0',
            method: 'dx/warmCache',
            params: {}
        });
        vscode.window.showInformationMessage('Warming cache...');
    }
}

function clearCache(): void {
    if (client?.isConnected()) {
        client.send({
            jsonrpc: '2.0',
            method: 'dx/clearCache',
            params: {}
        });
        vscode.window.showInformationMessage('Cache cleared');
    }
}

function showStats(): void {
    if (client?.isConnected()) {
        client.send({
            jsonrpc: '2.0',
            method: 'dx/getStats',
            params: {}
        });
    }
}

export function deactivate(): void {
    client?.dispose();
}
