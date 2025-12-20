/**
 * DX Forge Daemon Manager
 * 
 * Manages the dx-forge daemon process for binary generation and other tasks.
 */

import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';

export interface ForgeConfig {
    binaryPath?: string;
    autoStart: boolean;
    autoBuild: boolean;
}

type DaemonStatus = 'stopped' | 'starting' | 'running' | 'error';

export class ForgeDaemon implements vscode.Disposable {
    private process: cp.ChildProcess | null = null;
    private status: DaemonStatus = 'stopped';
    private outputChannel: vscode.OutputChannel;
    private statusBar: vscode.StatusBarItem;
    private config: ForgeConfig;

    constructor(config?: Partial<ForgeConfig>) {
        this.config = {
            binaryPath: config?.binaryPath,
            autoStart: config?.autoStart ?? true,
            autoBuild: config?.autoBuild ?? true
        };

        this.outputChannel = vscode.window.createOutputChannel('DX Forge');
        this.statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
        this.statusBar.command = 'dx-hologram.showForgeOutput';
        this.updateStatusBar();
    }

    async start(): Promise<boolean> {
        if (this.status === 'running') {
            return true;
        }

        this.status = 'starting';
        this.updateStatusBar();
        this.outputChannel.appendLine('[Forge] Starting daemon...');

        try {
            const forgePath = await this.findForgeBinary();

            if (!forgePath) {
                this.outputChannel.appendLine('[Forge] Error: dx-forge binary not found');
                this.status = 'error';
                this.updateStatusBar();
                return false;
            }

            this.process = cp.spawn(forgePath, ['daemon', '--watch'], {
                stdio: ['pipe', 'pipe', 'pipe'],
                cwd: this.getWorkspaceRoot()
            });

            this.process.stdout?.on('data', (data: Buffer) => {
                this.outputChannel.appendLine(`[Forge] ${data.toString().trim()}`);
            });

            this.process.stderr?.on('data', (data: Buffer) => {
                this.outputChannel.appendLine(`[Forge Error] ${data.toString().trim()}`);
            });

            this.process.on('close', (code) => {
                this.outputChannel.appendLine(`[Forge] Daemon exited with code ${code}`);
                this.status = 'stopped';
                this.updateStatusBar();
                this.process = null;
            });

            this.process.on('error', (err) => {
                this.outputChannel.appendLine(`[Forge Error] ${err.message}`);
                this.status = 'error';
                this.updateStatusBar();
            });

            // Wait for startup
            await new Promise(resolve => setTimeout(resolve, 500));

            if (this.process && this.process.exitCode === null) {
                this.status = 'running';
                this.updateStatusBar();
                this.outputChannel.appendLine('[Forge] Daemon started successfully');
                return true;
            }

            return false;

        } catch (error) {
            this.outputChannel.appendLine(`[Forge Error] Failed to start: ${error}`);
            this.status = 'error';
            this.updateStatusBar();
            return false;
        }
    }

    stop(): void {
        if (this.process) {
            this.outputChannel.appendLine('[Forge] Stopping daemon...');
            this.process.kill();
            this.process = null;
        }
        this.status = 'stopped';
        this.updateStatusBar();
    }

    async restart(): Promise<boolean> {
        this.stop();
        await new Promise(resolve => setTimeout(resolve, 100));
        return this.start();
    }

    async buildBinary(filePath: string): Promise<boolean> {
        // Calculate hashed output path for .dxs file
        const workspaceRoot = this.getWorkspaceRoot() || '';
        const outputPath = this.getBinaryPath(filePath, workspaceRoot);

        // If daemon is running, send build command
        if (this.status === 'running' && this.process?.stdin) {
            this.outputChannel.appendLine(`[Forge] Building: ${path.basename(filePath)} -> .dx/serializer/${path.basename(path.dirname(outputPath))}/${path.basename(outputPath)}`);
            this.process.stdin.write(`build ${filePath}\n`);
            return true;
        }

        // Otherwise, run one-off build
        return this.runBuildCommand(filePath, outputPath);
    }

    /**
     * FNV-1a hash for path uniqueness (matches dx-serializer)
     */
    private hashPath(relativePath: string): string {
        const FNV_OFFSET = BigInt('0xcbf29ce484222325');
        const FNV_PRIME = BigInt('0x100000001b3');

        let hash = FNV_OFFSET;
        for (let i = 0; i < relativePath.length; i++) {
            hash ^= BigInt(relativePath.charCodeAt(i));
            hash = (hash * FNV_PRIME) & BigInt('0xffffffffffffffff');
        }

        // Return lower 32 bits as 8 hex chars
        return (hash & BigInt('0xffffffff')).toString(16).padStart(8, '0');
    }

    /**
     * Get the .dxs output path with hashed subfolder
     */
    private getBinaryPath(sourcePath: string, projectRoot: string): string {
        // Normalize and get relative path
        const normalizedSource = sourcePath.replace(/\\/g, '/');
        const normalizedRoot = projectRoot.replace(/\\/g, '/');
        
        let relativePath = normalizedSource;
        if (normalizedSource.startsWith(normalizedRoot)) {
            relativePath = normalizedSource.substring(normalizedRoot.length);
            if (relativePath.startsWith('/')) {
                relativePath = relativePath.substring(1);
            }
        }

        // Hash the relative path
        const hash = this.hashPath(relativePath);

        // Get filename without extension
        const filename = path.basename(sourcePath, path.extname(sourcePath)) || 'dx';

        // Return: .dx/serializer/{hash}/{filename}.dxs
        return path.join(projectRoot, '.dx', 'serializer', hash, `${filename}.dxs`);
    }

    private async runBuildCommand(input: string, output: string): Promise<boolean> {
        const forgePath = await this.findForgeBinary();

        if (!forgePath) {
            this.outputChannel.appendLine('[Forge] Error: dx-forge binary not found');
            vscode.window.showErrorMessage('dx-forge binary not found. Please install dx-forge.');
            return false;
        }

        return new Promise((resolve) => {
            this.outputChannel.appendLine(`[Forge] Building: ${input} → ${output}`);

            const child = cp.spawn(forgePath, ['build', '--input', input, '--output', output], {
                cwd: this.getWorkspaceRoot()
            });

            child.stdout?.on('data', (data: Buffer) => {
                this.outputChannel.appendLine(`[Forge] ${data.toString().trim()}`);
            });

            child.stderr?.on('data', (data: Buffer) => {
                this.outputChannel.appendLine(`[Forge Error] ${data.toString().trim()}`);
            });

            child.on('close', (code) => {
                if (code === 0) {
                    this.outputChannel.appendLine(`[Forge] Build successful: ${output}`);
                    resolve(true);
                } else {
                    this.outputChannel.appendLine(`[Forge] Build failed with code ${code}`);
                    resolve(false);
                }
            });

            child.on('error', (err) => {
                this.outputChannel.appendLine(`[Forge Error] ${err.message}`);
                resolve(false);
            });
        });
    }

    private async findForgeBinary(): Promise<string | null> {
        // Check configured path
        if (this.config.binaryPath) {
            return this.config.binaryPath;
        }

        // Check in workspace target directory
        const workspaceRoot = this.getWorkspaceRoot();
        if (workspaceRoot) {
            const possiblePaths = [
                path.join(workspaceRoot, 'target', 'release', 'dx-forge'),
                path.join(workspaceRoot, 'target', 'debug', 'dx-forge'),
                path.join(workspaceRoot, 'target', 'release', 'dx-forge.exe'),
                path.join(workspaceRoot, 'target', 'debug', 'dx-forge.exe')
            ];

            for (const p of possiblePaths) {
                try {
                    await vscode.workspace.fs.stat(vscode.Uri.file(p));
                    return p;
                } catch {
                    // Not found, continue
                }
            }
        }

        // Check system PATH
        try {
            const result = await new Promise<string>((resolve, reject) => {
                cp.exec('which dx-forge || where dx-forge', (err, stdout) => {
                    if (err) reject(err);
                    else resolve(stdout.trim().split('\n')[0]);
                });
            });
            return result;
        } catch {
            // Not in PATH
        }

        return null;
    }

    private getWorkspaceRoot(): string | undefined {
        const folders = vscode.workspace.workspaceFolders;
        return folders?.[0]?.uri.fsPath;
    }

    private updateStatusBar(): void {
        const icons: Record<DaemonStatus, string> = {
            'stopped': '$(circle-outline)',
            'starting': '$(sync~spin)',
            'running': '$(check)',
            'error': '$(error)'
        };

        const labels: Record<DaemonStatus, string> = {
            'stopped': 'Forge: Stopped',
            'starting': 'Forge: Starting...',
            'running': 'Forge: Running',
            'error': 'Forge: Error'
        };

        this.statusBar.text = `${icons[this.status]} ${labels[this.status]}`;
        this.statusBar.show();
    }

    getStatus(): DaemonStatus {
        return this.status;
    }

    showOutput(): void {
        this.outputChannel.show();
    }

    dispose(): void {
        this.stop();
        this.outputChannel.dispose();
        this.statusBar.dispose();
    }
}
