/**
 * Forge Commands - VS Code command handlers for Forge integration
 */

import * as vscode from 'vscode';
import { ForgeClient, getForgeClient } from './client';
import { exec } from 'child_process';

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
 * Start the Forge daemon
 */
async function startForgeDaemon(): Promise<void> {
    const client = getForgeClient();

    if (client.isConnected()) {
        vscode.window.showInformationMessage('Forge daemon is already running');
        return;
    }

    vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title: 'Starting Forge daemon...',
            cancellable: false
        },
        async () => {
            return new Promise<void>((resolve) => {
                exec('dx forge start', (error, stdout, stderr) => {
                    if (error) {
                        vscode.window.showErrorMessage(
                            `Failed to start Forge daemon: ${stderr || error.message}`
                        );
                    } else {
                        // Try to connect
                        setTimeout(async () => {
                            const connected = await client.connect();
                            if (connected) {
                                vscode.window.showInformationMessage('Forge daemon started');
                            } else {
                                vscode.window.showWarningMessage(
                                    'Forge daemon started but connection failed'
                                );
                            }
                        }, 1000);
                    }
                    resolve();
                });
            });
        }
    );
}

/**
 * Stop the Forge daemon
 */
async function stopForgeDaemon(): Promise<void> {
    const client = getForgeClient();

    vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title: 'Stopping Forge daemon...',
            cancellable: false
        },
        async () => {
            return new Promise<void>((resolve) => {
                exec('dx forge stop', (error, stdout, stderr) => {
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
    vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title: 'Restarting Forge daemon...',
            cancellable: false
        },
        async () => {
            return new Promise<void>((resolve) => {
                exec('dx forge restart', (error, stdout, stderr) => {
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
