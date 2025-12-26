/**
 * Forge Status Bar - Shows daemon connection status
 */

import * as vscode from 'vscode';
import { ForgeClient, ForgeStatus } from './client';

export class ForgeStatusBar {
    private statusBarItem: vscode.StatusBarItem;
    private client: ForgeClient;
    private updateInterval: NodeJS.Timeout | null = null;

    constructor(client: ForgeClient) {
        this.client = client;
        this.statusBarItem = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Left,
            100
        );
        this.statusBarItem.command = 'dx.forge.status';
        this.show();
    }

    /**
     * Show the status bar item
     */
    show(): void {
        const config = vscode.workspace.getConfiguration('dx.forge');
        if (config.get('showStatusBar', true)) {
            this.statusBarItem.show();
            this.startUpdating();
        }
    }

    /**
     * Hide the status bar item
     */
    hide(): void {
        this.statusBarItem.hide();
        this.stopUpdating();
    }

    /**
     * Start periodic status updates
     */
    private startUpdating(): void {
        this.updateStatus();
        this.updateInterval = setInterval(() => {
            this.updateStatus();
        }, 5000);
    }

    /**
     * Stop periodic updates
     */
    private stopUpdating(): void {
        if (this.updateInterval) {
            clearInterval(this.updateInterval);
            this.updateInterval = null;
        }
    }

    /**
     * Update status display
     */
    private async updateStatus(): Promise<void> {
        if (this.client.isConnected()) {
            const status = await this.client.getStatus();
            if (status) {
                this.showConnected(status);
            } else {
                this.showConnected();
            }
        } else {
            this.showDisconnected();
        }
    }

    /**
     * Show connected status
     */
    showConnected(status?: ForgeStatus): void {
        this.statusBarItem.text = '$(check) DX Forge';
        this.statusBarItem.backgroundColor = undefined;

        if (status) {
            const uptime = this.formatUptime(status.uptime_seconds);
            this.statusBarItem.tooltip = new vscode.MarkdownString(
                `**DX Forge** - Connected\n\n` +
                `- Uptime: ${uptime}\n` +
                `- Files changed: ${status.files_changed}\n` +
                `- Tools executed: ${status.tools_executed}\n` +
                `- Cache hits: ${status.cache_hits}\n` +
                `- Errors: ${status.errors}`
            );
        } else {
            this.statusBarItem.tooltip = 'DX Forge - Connected';
        }
    }

    /**
     * Show disconnected status
     */
    showDisconnected(): void {
        this.statusBarItem.text = '$(circle-slash) DX Forge';
        this.statusBarItem.backgroundColor = new vscode.ThemeColor(
            'statusBarItem.warningBackground'
        );
        this.statusBarItem.tooltip = 'DX Forge - Disconnected (click to start)';
    }

    /**
     * Show connecting status
     */
    showConnecting(): void {
        this.statusBarItem.text = '$(sync~spin) DX Forge';
        this.statusBarItem.backgroundColor = undefined;
        this.statusBarItem.tooltip = 'DX Forge - Connecting...';
    }

    /**
     * Format uptime in human-readable form
     */
    private formatUptime(seconds: number): string {
        if (seconds < 60) {
            return `${seconds}s`;
        } else if (seconds < 3600) {
            return `${Math.floor(seconds / 60)}m ${seconds % 60}s`;
        } else {
            const hours = Math.floor(seconds / 3600);
            const mins = Math.floor((seconds % 3600) / 60);
            return `${hours}h ${mins}m`;
        }
    }

    /**
     * Dispose resources
     */
    dispose(): void {
        this.stopUpdating();
        this.statusBarItem.dispose();
    }
}
