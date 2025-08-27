const vscode = require('vscode');
const path = require('path');

/**
 * Nova Debug Configuration Provider
 */
class NovaDebugConfigurationProvider {
    /**
     * Provide debug configurations for Nova files
     */
    provideDebugConfigurations(folder, token) {
        return [
            {
                name: 'Nova: Run Current File',
                type: 'nova',
                request: 'launch',
                program: '${file}',
                console: 'integratedTerminal',
                cwd: '${workspaceFolder}'
            },
            {
                name: 'Nova: Debug Current File',
                type: 'nova',
                request: 'launch',
                program: '${file}',
                console: 'integratedTerminal',
                cwd: '${workspaceFolder}',
                stopOnEntry: true
            }
        ];
    }

    /**
     * Resolve debug configuration before starting debug session
     */
    resolveDebugConfiguration(folder, config, token) {
        // If no configuration is provided, create a default one
        if (!config.type && !config.request && !config.name) {
            const editor = vscode.window.activeTextEditor;
            if (editor && editor.document.languageId === 'nova') {
                config.type = 'nova';
                config.name = 'Nova: Run Current File';
                config.request = 'launch';
                config.program = editor.document.fileName;
                config.console = 'integratedTerminal';
                config.cwd = '${workspaceFolder}';
            }
        }

        if (!config.program) {
            return vscode.window.showInformationMessage("Cannot find a program to debug").then(_ => {
                return undefined; // abort launch
            });
        }

        return config;
    }
}

/**
 * Nova Debug Adapter Descriptor Factory
 */
class NovaDebugAdapterDescriptorFactory {
    createDebugAdapterDescriptor(session, executable) {
        // For now, we'll use a simple command-based approach
        // In a full implementation, this would connect to an actual debug adapter
        const novaExecutable = this.findNovaExecutable();
        
        if (!novaExecutable) {
            vscode.window.showErrorMessage('Nova executable not found. Please ensure Nova is installed and in PATH.');
            return null;
        }

        // Return executable that will run the Nova interpreter
        return new vscode.DebugAdapterExecutable(novaExecutable, [session.configuration.program]);
    }

    findNovaExecutable() {
        // Look for Nova executable in various locations
        const possiblePaths = [
            // Relative to workspace
            path.join(vscode.workspace.rootPath || '', 'target', 'release', 'nova'),
            path.join(vscode.workspace.rootPath || '', 'target', 'debug', 'nova'),
            // System PATH
            'nova',
            // Common installation paths
            '/usr/local/bin/nova',
            '/usr/bin/nova',
        ];

        const fs = require('fs');
        for (const execPath of possiblePaths) {
            if (fs.existsSync(execPath)) {
                return execPath;
            }
        }

        return null;
    }
}

/**
 * Task Provider for Nova build/run tasks
 */
class NovaTaskProvider {
    provideTasks() {
        const tasks = [];
        
        // Build task
        const buildTask = new vscode.Task(
            { type: 'nova', command: 'build' },
            vscode.TaskScope.Workspace,
            'Build Nova Project',
            'nova',
            new vscode.ShellExecution('cargo build --release', {
                cwd: '${workspaceFolder}'
            }),
            '$rustc'
        );
        tasks.push(buildTask);

        // Run current file task
        const runTask = new vscode.Task(
            { type: 'nova', command: 'run' },
            vscode.TaskScope.Workspace,
            'Run Current Nova File',
            'nova',
            new vscode.ShellExecution('${workspaceFolder}/target/release/nova "${file}"', {
                cwd: '${workspaceFolder}'
            })
        );
        tasks.push(runTask);

        // Test task
        const testTask = new vscode.Task(
            { type: 'nova', command: 'test' },
            vscode.TaskScope.Workspace,
            'Test Nova Project',
            'nova',
            new vscode.ShellExecution('cargo test', {
                cwd: '${workspaceFolder}'
            }),
            '$rustc'
        );
        tasks.push(testTask);

        return tasks;
    }

    resolveTask(task) {
        // Resolve task if needed
        return task;
    }
}

/**
 * Register all debug and task providers
 */
function registerDebugSupport(context) {
    // Register debug configuration provider
    const debugConfigProvider = new NovaDebugConfigurationProvider();
    context.subscriptions.push(
        vscode.debug.registerDebugConfigurationProvider('nova', debugConfigProvider)
    );

    // Register debug adapter descriptor factory
    const debugAdapterFactory = new NovaDebugAdapterDescriptorFactory();
    context.subscriptions.push(
        vscode.debug.registerDebugAdapterDescriptorFactory('nova', debugAdapterFactory)
    );

    // Register task provider
    const taskProvider = new NovaTaskProvider();
    context.subscriptions.push(
        vscode.tasks.registerTaskProvider('nova', taskProvider)
    );

    // Register run file command
    const runFileCommand = vscode.commands.registerCommand('nova.runFile', async (uri) => {
        const fileUri = uri || vscode.window.activeTextEditor?.document.uri;
        if (!fileUri) {
            vscode.window.showErrorMessage('No Nova file selected');
            return;
        }

        // Create and execute run task
        const task = new vscode.Task(
            { type: 'nova', command: 'run-file' },
            vscode.TaskScope.Workspace,
            'Run Nova File',
            'nova',
            new vscode.ShellExecution(`nova "${fileUri.fsPath}"`, {
                cwd: path.dirname(fileUri.fsPath)
            })
        );

        vscode.tasks.executeTask(task);
    });

    context.subscriptions.push(runFileCommand);

    // Register debug file command
    const debugFileCommand = vscode.commands.registerCommand('nova.debugFile', async (uri) => {
        const fileUri = uri || vscode.window.activeTextEditor?.document.uri;
        if (!fileUri) {
            vscode.window.showErrorMessage('No Nova file selected');
            return;
        }

        const config = {
            type: 'nova',
            name: 'Debug Nova File',
            request: 'launch',
            program: fileUri.fsPath,
            console: 'integratedTerminal',
            stopOnEntry: true
        };

        vscode.debug.startDebugging(undefined, config);
    });

    context.subscriptions.push(debugFileCommand);
}

module.exports = {
    registerDebugSupport,
    NovaDebugConfigurationProvider,
    NovaDebugAdapterDescriptorFactory,
    NovaTaskProvider
};