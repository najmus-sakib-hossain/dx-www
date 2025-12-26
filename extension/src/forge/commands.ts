/**
 * Forge Commands - VS Code command handlers for Forge integration
 */

import * as vscode from 'vscode';
import { ForgeClient, getForgeClient } from './client';
import { exec } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

/**
 * Register all Forge-related commands
 */
export function registerForgeCommands(context: vscode.ExtensionContext): void {
    const client = getForgeClient();

    // Start Forge Daemon
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.forge.start', async () => {
            await startForgeDaemon();
        })
    );

    // Stop Forge Daemon
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.forge.stop', async () => {
            await stopForgeDaemon();
        })
    );

    // Restart Forge Daemon
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.forge.restart', async () => {
            await restartForgeDaemon();
        })
    );

    // Show Forge Status
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.forge.status', async () => {
            await showForgeStatus(client);
        })
    );

    // Show Tool List
    context.subscriptions.push(
        vscode.commands.registerCommand('dx.tools.list', async () => {
            await showToolList(client);
        })
    );
}

/**
 * Get the configured forge-cli executable path
 * Auto-detects from workspace if not explicitly configured
 */
function getForgeCliPath(): string {
    const config = vscode.workspace.getConfiguration('dx.forge');
    const configuredPath = config.get<string>('executablePath', '');

    // If explicitly configured, use that
    if (configuredPath) {
        console.log(`DX Forge: Using configured executable path: ${configuredPath}`);
        return configuredPath;
    }

    // Try to auto-detect from workspace
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders && workspaceFolders.length > 0) {
        const workspaceRoot = workspaceFolders[0].uri.fsPath;
        console.log(`DX Forge: Searching for forge-cli executable in workspace: ${workspaceRoot}`);

        // Check common locations in order of preference
        // Windows executables first, then Unix
        const isWindows = process.platform === 'win32';
        const candidates = isWindows ? [
            path.join(workspaceRoot, 'target', 'release', 'forge-cli.exe'),
            path.join(workspaceRoot, 'target', 'debug', 'forge-cli.exe'),
            path.join(workspaceRoot, 'target', 'release', 'forge-cli'),
            path.join(workspaceRoot, 'target', 'debug', 'forge-cli'),
        ] : [
            path.join(workspaceRoot, 'target', 'release', 'forge-cli'),
            path.join(workspaceRoot, 'target', 'debug', 'forge-cli'),
            path.join(workspaceRoot, 'target', 'release', 'forge-cli.exe'),
            path.join(workspaceRoot, 'target', 'debug', 'forge-cli.exe'),
        ];

        for (const candidate of candidates) {
            console.log(`DX Forge: Checking candidate: ${candidate}`);
            if (fs.existsSync(candidate)) {
                console.log(`DX Forge: Found executable at: ${candidate}`);
                return candidate;
            }
        }

        console.log('DX Forge: No executable found in workspace, falling back to PATH');
    } else {
        console.log('DX Forge: No workspace folders found');
    }

    // Fallback to PATH lookup
    return 'forge-cli';
}

/**
 * Start the Forge daemon
 */
async function startForgeDaemon(): Promise<void> {
    const client = getForgeClient();

    if (client.isConnected()) {
        vscode.window.showInformationMessage('Forge daemon is already running');
        return;
    }

    const forgePath = getForgeCliPath();
    const config = vscode.workspace.getConfiguration('dx.forge');
    const port = config.get('port', 9876);

    await vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title: 'Starting Forge daemon...',
            cancellable: false
        },
        async () => {
            return new Promise<void>((resolve) => {
                // Use daemon start command with port
                // Set a timeout to prevent hanging if the process doesn't exit cleanly
                const child = exec(`"${forgePath}" daemon start --port ${port}`, { timeout: 10000 }, (error, stdout, stderr) => {
                    if (error) {
                        const errorMsg = stderr || error.message;
                        // Provide helpful message if forge-cli executable not found
                        if (errorMsg.includes('not recognized') || errorMsg.includes('not found') || errorMsg.includes('ENOENT')) {
                            vscode.window.showErrorMessage(
                                `Failed to start Forge daemon: forge-cli executable not found at "${forgePath}". ` +
                                `Please build the project with 'cargo build --release' or configure 'dx.forge.executablePath' in settings.`,
                                'Open Settings'
                            ).then(action => {
                                if (action === 'Open Settings') {
                                    vscode.commands.executeCommand('workbench.action.openSettings', 'dx.forge.executablePath');
                                }
                            });
                        } else if (!errorMsg.includes('SIGTERM') && !errorMsg.includes('killed')) {
                            // Ignore timeout kills - daemon likely started successfully
                            vscode.window.showErrorMessage(
                                `Failed to start Forge daemon: ${errorMsg}`
                            );
                        }
                    }
                    resolve();
                });

                // Also resolve after a short delay if the process seems to have started
                // This handles the case where the daemon daemonizes but exec doesn't return
                setTimeout(() => {
                    resolve();
                }, 3000);
            });
        }
    );

    // Try to connect after the progress closes
    setTimeout(async () => {
        const connected = await client.connect();
        if (connected) {
            vscode.window.showInformationMessage('Forge daemon started');
        } else {
            vscode.window.showWarningMessage(
                'Forge daemon may have started but connection failed. Check if it\'s running.'
            );
        }
    }, 1000);
}

/**
 * Stop the Forge daemon
 */
async function stopForgeDaemon(): Promise<void> {
    const client = getForgeClient();
    const forgePath = getForgeCliPath();

    vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title: 'Stopping Forge daemon...',
            cancellable: false
        },
        async () => {
            return new Promise<void>((resolve) => {
                exec(`"${forgePath}" daemon stop`, (error, stdout, stderr) => {
                    if (error) {
                        vscode.window.showErrorMessage(
                            `Failed to stop Forge daemon: ${stderr || error.message}`
                        );
                    } else {
                        client.disconnect();
                        vscode.window.showInformationMessage('Forge daemon stopped');
                    }
                    resolve();
                });
            });
        }
    );
}

/**
 * Restart the Forge daemon
 */
async function restartForgeDaemon(): Promise<void> {
    const forgePath = getForgeCliPath();

    vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title: 'Restarting Forge daemon...',
            cancellable: false
        },
        async () => {
            return new Promise<void>((resolve) => {
                exec(`"${forgePath}" daemon restart`, (error, stdout, stderr) => {
                    if (error) {
                        vscode.window.showErrorMessage(
                            `Failed to restart Forge daemon: ${stderr || error.message}`
                        );
                    } else {
                        const client = getForgeClient();
                        setTimeout(async () => {
                            await client.connect();
                            vscode.window.showInformationMessage('Forge daemon restarted');
                        }, 1500);
                    }
                    resolve();
                });
            });
        }
    );
}

/**
 * Show Forge status in a quick pick
 */
async function showForgeStatus(client: ForgeClient): Promise<void> {
    if (!client.isConnected()) {
        const action = await vscode.window.showWarningMessage(
            'Forge daemon is not running',
            'Start Daemon'
        );
        if (action === 'Start Daemon') {
            await startForgeDaemon();
        }
        return;
    }

    const status = await client.getStatus();
    if (!status) {
        vscode.window.showErrorMessage('Failed to get Forge status');
        return;
    }

    const uptime = formatUptime(status.uptime_seconds);

    const items: vscode.QuickPickItem[] = [
        { label: '$(check) Status', description: 'Running' },
        { label: '$(clock) Uptime', description: uptime },
        { label: '$(file) Files Changed', description: status.files_changed.toString() },
        { label: '$(tools) Tools Executed', description: status.tools_executed.toString() },
        { label: '$(database) Cache Hits', description: status.cache_hits.toString() },
        { label: '$(error) Errors', description: status.errors.toString() },
        { label: '', kind: vscode.QuickPickItemKind.Separator },
        { label: '$(debug-stop) Stop Daemon', description: 'Stop the Forge daemon' },
        { label: '$(debug-restart) Restart Daemon', description: 'Restart the Forge daemon' }
    ];

    const selected = await vscode.window.showQuickPick(items, {
        title: 'DX Forge Status',
        placeHolder: 'Select an action'
    });

    if (selected?.label.includes('Stop')) {
        await stopForgeDaemon();
    } else if (selected?.label.includes('Restart')) {
        await restartForgeDaemon();
    }
}

/**
 * Show tool list in a quick pick
 */
async function showToolList(client: ForgeClient): Promise<void> {
    if (!client.isConnected()) {
        vscode.window.showWarningMessage('Forge daemon is not running');
        return;
    }

    const tools = await client.listTools();
    if (tools.length === 0) {
        vscode.window.showInformationMessage('No tools registered');
        return;
    }

    const items: vscode.QuickPickItem[] = tools.map(tool => ({
        label: `${getStatusIcon(tool.status)} ${tool.name}`,
        description: `v${tool.version}${tool.is_dummy ? ' [dummy]' : ''}`,
        detail: `Runs: ${tool.run_count} | Errors: ${tool.error_count}`
    }));

    const selected = await vscode.window.showQuickPick(items, {
        title: 'DX Tools',
        placeHolder: 'Select a tool to run'
    });

    if (selected) {
        const toolName = selected.label.replace(/^\$\([^)]+\)\s*/, '');
        const run = await vscode.window.showInformationMessage(
            `Run ${toolName}?`,
            'Run'
        );
        if (run === 'Run') {
            await client.runTool(toolName);
            vscode.window.showInformationMessage(`Running ${toolName}...`);
        }
    }
}

function getStatusIcon(status: string): string {
    switch (status) {
        case 'Ready': return '$(check)';
        case 'Running': return '$(sync~spin)';
        case 'Disabled': return '$(circle-slash)';
        case 'Error': return '$(error)';
        default: return '$(question)';
    }
}

function formatUptime(seconds: number): string {
    if (seconds < 60) return `${seconds}s`;
    if (seconds < 3600) return `${Math.floor(seconds / 60)}m ${seconds % 60}s`;
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    return `${h}h ${m}m`;
}
