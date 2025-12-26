/**
 * Forge Client - WebSocket connection to Forge daemon
 */

import * as vscode from 'vscode';

// Types for Forge communication
export interface ForgeStatus {
    state: string;
    uptime_seconds: number;
    files_changed: number;
    tools_executed: number;
    cache_hits: number;
    errors: number;
}

export interface ToolInfo {
    id: string;
    name: string;
    version: string;
    status: string;
    is_dummy: boolean;
    run_count: number;
    error_count: number;
}

export interface ForgeEvent {
    type: 'file_changed' | 'tool_started' | 'tool_completed' | 'tool_failed' | 'error';
    data: any;
}

type EventHandler = (event: ForgeEvent) => void;

/**
 * Client for communicating with the Forge daemon
 */
export class ForgeClient {
    private ws: WebSocket | null = null;
    private reconnectAttempts = 0;
    private maxReconnectAttempts = 5;
    private reconnectDelay = 1000;
    private eventHandlers: EventHandler[] = [];
    private connected = false;
    private port: number;

    constructor() {
        const config = vscode.workspace.getConfiguration('dx.forge');
        this.port = config.get('port', 9876);
    }

    /**
     * Check if connected to daemon
     */
    isConnected(): boolean {
        return this.connected && this.ws !== null;
    }

    /**
     * Connect to the Forge daemon
     */
    async connect(): Promise<boolean> {
        if (this.isConnected()) {
            return true;
        }

        return new Promise((resolve) => {
            try {
                const url = `ws://127.0.0.1:${this.port}`;
                this.ws = new WebSocket(url);

                this.ws.onopen = () => {
                    this.connected = true;
                    this.reconnectAttempts = 0;
                    console.log('[Forge] Connected to daemon');
                    resolve(true);
                };

                this.ws.onclose = () => {
                    this.connected = false;
                    console.log('[Forge] Disconnected from daemon');
                    this.attemptReconnect();
                };

                this.ws.onerror = (error) => {
                    console.error('[Forge] WebSocket error:', error);
                    resolve(false);
                };

                this.ws.onmessage = (event) => {
                    try {
                        const data = JSON.parse(event.data);
                        this.handleMessage(data);
                    } catch (e) {
                        console.error('[Forge] Failed to parse message:', e);
                    }
                };
            } catch (error) {
                console.error('[Forge] Failed to connect:', error);
                resolve(false);
            }
        });
    }

    /**
     * Disconnect from the daemon
     */
    async disconnect(): Promise<void> {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
        this.connected = false;
    }

    /**
     * Attempt to reconnect with exponential backoff
     */
    private attemptReconnect(): void {
        if (this.reconnectAttempts >= this.maxReconnectAttempts) {
            console.log('[Forge] Max reconnect attempts reached');
            return;
        }

        this.reconnectAttempts++;
        const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);

        console.log(`[Forge] Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`);

        setTimeout(() => {
            this.connect();
        }, delay);
    }

    /**
     * Handle incoming messages
     */
    private handleMessage(data: any): void {
        const event: ForgeEvent = {
            type: data.type || 'error',
            data: data.data || data
        };

        for (const handler of this.eventHandlers) {
            try {
                handler(event);
            } catch (e) {
                console.error('[Forge] Event handler error:', e);
            }
        }
    }

    /**
     * Subscribe to events
     */
    onEvent(handler: EventHandler): void {
        this.eventHandlers.push(handler);
    }

    /**
     * Send a command to the daemon
     */
    private send(command: any): Promise<any> {
        return new Promise((resolve, reject) => {
            if (!this.isConnected()) {
                reject(new Error('Not connected to Forge daemon'));
                return;
            }

            try {
                this.ws!.send(JSON.stringify(command));
                // For now, resolve immediately - real impl would wait for response
                resolve({ success: true });
            } catch (error) {
                reject(error);
            }
        });
    }

    /**
     * Get daemon status
     */
    async getStatus(): Promise<ForgeStatus | null> {
        try {
            const response = await this.send({ command: 'GetStatus' });
            return response.status || null;
        } catch {
            return null;
        }
    }

    /**
     * List all tools
     */
    async listTools(): Promise<ToolInfo[]> {
        try {
            const response = await this.send({ command: 'ListTools' });
            return response.tools || [];
        } catch {
            return [];
        }
    }

    /**
     * Run a specific tool
     */
    async runTool(name: string): Promise<boolean> {
        try {
            await this.send({ command: 'RunTool', name });
            return true;
        } catch {
            return false;
        }
    }

    /**
     * Notify daemon of file change
     */
    async notifyFileChange(path: string, type: 'created' | 'modified' | 'deleted'): Promise<void> {
        try {
            await this.send({
                command: 'FileChanged',
                path,
                type
            });
        } catch (e) {
            console.error('[Forge] Failed to notify file change:', e);
        }
    }
}

/**
 * Singleton instance
 */
let forgeClientInstance: ForgeClient | null = null;

export function getForgeClient(): ForgeClient {
    if (!forgeClientInstance) {
        forgeClientInstance = new ForgeClient();
    }
    return forgeClientInstance;
}
