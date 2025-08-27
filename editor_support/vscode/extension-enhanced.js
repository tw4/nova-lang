const vscode = require('vscode');
const path = require('path');
const { workspace } = vscode;
const {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} = require('vscode-languageclient/node');

// Import debug support
const { registerDebugSupport } = require('./src/debug-provider');

let client;

// Nova keywords and built-ins from Rust implementation
const NOVA_KEYWORDS = [
    'let', 'const', 'fn', 'if', 'else', 'while', 'for', 'in', 
    'return', 'break', 'continue', 'true', 'false', 'null',
    'and', 'or', 'not', 'try', 'catch', 'finally', 'throw',
    'import', 'from', 'export', 'class', 'extends', 'super',
    'this', 'static', 'private', 'public', 'async', 'await',
    'as', 'new', 'constructor'
];

const NOVA_BUILTINS = [
    { name: 'print', detail: 'print(value) -> void', docs: 'Print a value to the console' },
    { name: 'len', detail: 'len(collection) -> number', docs: 'Get the length of a string or array' },
    { name: 'type', detail: 'type(value) -> string', docs: 'Get the type name of a value' },
    { name: 'str', detail: 'str(value) -> string', docs: 'Convert a value to string' },
    { name: 'num', detail: 'num(value) -> number', docs: 'Convert a value to number' },
    { name: 'push', detail: 'push(array, value) -> array', docs: 'Add an element to the end of an array' },
    { name: 'pop', detail: 'pop(array) -> any', docs: 'Remove and return the last element of an array' },
    { name: 'split', detail: 'split(string, separator) -> array', docs: 'Split a string into an array' },
    { name: 'join', detail: 'join(array, separator) -> string', docs: 'Join array elements into a string' },
    { name: 'abs', detail: 'abs(number) -> number', docs: 'Get the absolute value of a number' },
    { name: 'max', detail: 'max(a, b) -> number', docs: 'Get the maximum of two numbers' },
    { name: 'min', detail: 'min(a, b) -> number', docs: 'Get the minimum of two numbers' },
    { name: 'sqrt', detail: 'sqrt(number) -> number', docs: 'Get the square root of a number' },
    { name: 'pow', detail: 'pow(base, exponent) -> number', docs: 'Raise base to the power of exponent' },
    { name: 'floor', detail: 'floor(number) -> number', docs: 'Round down to the nearest integer' },
    { name: 'ceil', detail: 'ceil(number) -> number', docs: 'Round up to the nearest integer' },
    { name: 'round', detail: 'round(number) -> number', docs: 'Round to the nearest integer' },
    { name: 'random', detail: 'random() -> number', docs: 'Generate a random number between 0 and 1' }
];

/**
 * Check if Nova LSP server binary exists
 */
function findNovaLspServer(context) {
    const possiblePaths = [
        // Try to find compiled Rust LSP server
        context.asAbsolutePath(path.join('server-rust', 'target', 'release', 'nova-lsp')),
        context.asAbsolutePath(path.join('server-rust', 'target', 'debug', 'nova-lsp')),
        // Fallback paths
        path.join(__dirname, 'server-rust', 'target', 'release', 'nova-lsp'),
        path.join(__dirname, 'server-rust', 'target', 'debug', 'nova-lsp'),
    ];

    const fs = require('fs');
    for (const serverPath of possiblePaths) {
        if (fs.existsSync(serverPath)) {
            console.log('Found Nova LSP server at:', serverPath);
            return serverPath;
        }
    }

    return null;
}

/**
 * Create and start the Nova Language Server
 */
async function startLanguageServer(context) {
    const serverPath = findNovaLspServer(context);
    
    if (!serverPath) {
        console.warn('Nova LSP server not found. Language server features will be limited.');
        vscode.window.showWarningMessage(
            'Nova LSP server not found. Please build the server with: cd server-rust && cargo build --release'
        );
        return null;
    }

    // Server options for the Rust LSP server
    const serverOptions = {
        run: { 
            command: serverPath,
            transport: TransportKind.stdio 
        },
        debug: { 
            command: serverPath,
            transport: TransportKind.stdio 
        }
    };

    // Client options
    const clientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'nova' },
            { scheme: 'untitled', language: 'nova' }
        ],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/*.nova')
        },
        outputChannelName: 'Nova Language Server',
        traceOutputChannel: vscode.window.createOutputChannel('Nova LSP Trace'),
        revealOutputChannelOn: vscode.LanguageClientRevealOutputChannelOn.OnLogMessage,
    };

    // Create language client
    const languageClient = new LanguageClient(
        'nova-language-server',
        'Nova Language Server',
        serverOptions,
        clientOptions
    );

    try {
        console.log('Starting Nova Language Server...');
        await languageClient.start();
        console.log('Nova Language Server started successfully');
        return languageClient;
    } catch (error) {
        console.error('Failed to start Nova Language Server:', error);
        vscode.window.showErrorMessage(`Failed to start Nova Language Server: ${error.message}`);
        return null;
    }
}

/**
 * Register fallback completion provider if LSP is not available
 */
function registerFallbackProviders(context) {
    console.log('Registering fallback completion providers...');

    // Fallback completion provider
    const completionProvider = vscode.languages.registerCompletionItemProvider(
        'nova',
        {
            provideCompletionItems(document, position, token, context) {
                const completions = [];

                // Add keywords
                NOVA_KEYWORDS.forEach(keyword => {
                    const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
                    item.detail = `Nova keyword: ${keyword}`;
                    item.sortText = '1' + keyword;
                    completions.push(item);
                });

                // Add built-in functions
                NOVA_BUILTINS.forEach(builtin => {
                    const item = new vscode.CompletionItem(builtin.name, vscode.CompletionItemKind.Function);
                    item.detail = builtin.detail;
                    item.documentation = new vscode.MarkdownString(builtin.docs);
                    item.sortText = '2' + builtin.name;
                    completions.push(item);
                });

                return completions;
            }
        },
        '.', ' ', '('
    );

    // Fallback hover provider
    const hoverProvider = vscode.languages.registerHoverProvider('nova', {
        provideHover(document, position, token) {
            const wordRange = document.getWordRangeAtPosition(position);
            if (!wordRange) return;

            const word = document.getText(wordRange);
            const builtin = NOVA_BUILTINS.find(b => b.name === word);

            if (builtin) {
                const markdown = new vscode.MarkdownString();
                markdown.appendCodeblock(builtin.detail, 'nova');
                markdown.appendMarkdown('\n\n' + builtin.docs);
                return new vscode.Hover(markdown);
            }

            // Basic keyword documentation
            if (NOVA_KEYWORDS.includes(word)) {
                return new vscode.Hover(new vscode.MarkdownString(`**${word}** - Nova language keyword`));
            }

            return null;
        }
    });

    context.subscriptions.push(completionProvider);
    context.subscriptions.push(hoverProvider);
}

/**
 * Register Nova-specific commands
 */
function registerCommands(context) {
    // Restart language server command
    const restartCommand = vscode.commands.registerCommand('nova.restartLanguageServer', async () => {
        if (client) {
            console.log('Restarting Nova Language Server...');
            await client.stop();
            client = await startLanguageServer(context);
            
            if (client) {
                vscode.window.showInformationMessage('Nova Language Server restarted successfully');
            } else {
                vscode.window.showErrorMessage('Failed to restart Nova Language Server');
            }
        }
    });

    // Show language server output
    const showOutputCommand = vscode.commands.registerCommand('nova.showLanguageServerOutput', () => {
        if (client) {
            client.outputChannel.show();
        } else {
            vscode.window.showWarningMessage('Nova Language Server is not running');
        }
    });

    // Build LSP server command
    const buildServerCommand = vscode.commands.registerCommand('nova.buildLanguageServer', async () => {
        const terminal = vscode.window.createTerminal('Nova LSP Build');
        const serverDir = path.join(context.extensionPath, 'server-rust');
        terminal.sendText(`cd "${serverDir}" && cargo build --release`);
        terminal.show();
        vscode.window.showInformationMessage('Building Nova Language Server... Check terminal for progress.');
    });

    context.subscriptions.push(restartCommand);
    context.subscriptions.push(showOutputCommand);
    context.subscriptions.push(buildServerCommand);
}

/**
 * Register document formatting provider
 */
function registerFormattingProvider(context) {
    const formattingProvider = vscode.languages.registerDocumentFormattingEditProvider('nova', {
        provideDocumentFormattingEdits(document, options, token) {
            // Basic formatting - could be enhanced with actual formatter
            const edits = [];
            const text = document.getText();
            
            // Simple indentation fix
            const lines = text.split('\n');
            let indentLevel = 0;
            const indentString = options.insertSpaces ? ' '.repeat(options.tabSize) : '\t';
            
            for (let i = 0; i < lines.length; i++) {
                const line = lines[i];
                const trimmed = line.trim();
                
                if (trimmed === '') continue;
                
                // Decrease indent for closing braces
                if (trimmed.startsWith('}')) {
                    indentLevel = Math.max(0, indentLevel - 1);
                }
                
                const expectedIndent = indentString.repeat(indentLevel);
                const currentIndent = line.match(/^\\s*/)[0];
                
                if (currentIndent !== expectedIndent) {
                    const range = new vscode.Range(i, 0, i, currentIndent.length);
                    edits.push(new vscode.TextEdit(range, expectedIndent));
                }
                
                // Increase indent for opening braces
                if (trimmed.endsWith('{')) {
                    indentLevel++;
                }
            }
            
            return edits;
        }
    });

    context.subscriptions.push(formattingProvider);
}

/**
 * Register status bar item
 */
function registerStatusBar(context) {
    const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.command = 'nova.showLanguageServerOutput';
    statusBarItem.text = '$(gear) Nova LSP';
    statusBarItem.tooltip = 'Nova Language Server Status - Click to show output';
    statusBarItem.show();

    // Update status based on LSP server state
    if (client) {
        client.onReady().then(() => {
            statusBarItem.text = '$(check) Nova LSP';
            statusBarItem.color = new vscode.ThemeColor('statusBarItem.activeBackground');
        });
        
        client.onDidChangeState((event) => {
            if (event.newState === vscode.LanguageClient.State.Running) {
                statusBarItem.text = '$(check) Nova LSP';
            } else {
                statusBarItem.text = '$(x) Nova LSP';
            }
        });
    } else {
        statusBarItem.text = '$(x) Nova LSP';
        statusBarItem.tooltip = 'Nova Language Server not available - Click to build';
        statusBarItem.command = 'nova.buildLanguageServer';
    }

    context.subscriptions.push(statusBarItem);
}

/**
 * Extension activation function
 */
async function activate(context) {
    console.log('Nova Language Support extension is activating...');

    // Try to start the language server
    client = await startLanguageServer(context);

    // Register fallback providers if LSP is not available
    if (!client) {
        registerFallbackProviders(context);
    }

    // Register commands
    registerCommands(context);

    // Register formatting provider
    registerFormattingProvider(context);

    // Register status bar
    registerStatusBar(context);

    // Register additional providers
    registerAdditionalProviders(context);

    // Register debug and task support
    registerDebugSupport(context);

    console.log('Nova Language Support extension activated successfully!');
}

/**
 * Register additional language providers
 */
function registerAdditionalProviders(context) {
    // Document symbol provider
    const documentSymbolProvider = vscode.languages.registerDocumentSymbolProvider('nova', {
        provideDocumentSymbols(document, token) {
            const symbols = [];
            const text = document.getText();
            
            // Simple regex-based symbol detection
            const functionPattern = /fn\s+(\w+)\s*\(/g;
            const classPattern = /class\s+(\w+)/g;
            
            let match;
            
            // Find functions
            while ((match = functionPattern.exec(text)) !== null) {
                const position = document.positionAt(match.index);
                const symbol = new vscode.DocumentSymbol(
                    match[1],
                    '',
                    vscode.SymbolKind.Function,
                    new vscode.Range(position, position),
                    new vscode.Range(position, position)
                );
                symbols.push(symbol);
            }
            
            // Find classes
            while ((match = classPattern.exec(text)) !== null) {
                const position = document.positionAt(match.index);
                const symbol = new vscode.DocumentSymbol(
                    match[1],
                    '',
                    vscode.SymbolKind.Class,
                    new vscode.Range(position, position),
                    new vscode.Range(position, position)
                );
                symbols.push(symbol);
            }
            
            return symbols;
        }
    });

    // Code lens provider
    const codeLensProvider = vscode.languages.registerCodeLensProvider('nova', {
        provideCodeLenses(document, token) {
            const codeLenses = [];
            const text = document.getText();
            
            // Add "Run" code lens for main functions
            const mainPattern = /fn\s+main\s*\(/g;
            let match;
            
            while ((match = mainPattern.exec(text)) !== null) {
                const position = document.positionAt(match.index);
                const range = new vscode.Range(position, position);
                
                const runLens = new vscode.CodeLens(range, {
                    title: '▶ Run',
                    command: 'nova.runFile',
                    arguments: [document.uri]
                });
                
                codeLenses.push(runLens);
            }
            
            return codeLenses;
        }
    });

    context.subscriptions.push(documentSymbolProvider);
    context.subscriptions.push(codeLensProvider);
}

/**
 * Extension deactivation function
 */
async function deactivate() {
    if (client) {
        console.log('Stopping Nova Language Server...');
        await client.stop();
    }
    console.log('Nova Language Support extension deactivated');
}

module.exports = {
    activate,
    deactivate
};