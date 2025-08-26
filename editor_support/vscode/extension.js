// Nova Language Extension for VS Code
const path = require('path');
const { workspace, ExtensionContext } = require('vscode');

const {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} = require('vscode-languageclient/node');

let client;

function activate(context) {
    console.log('Nova Language Extension is now active!');

    // Server executable path
    const serverModule = context.asAbsolutePath(
        path.join('server', 'out', 'server.js')
    );

    // Debug options for the server
    const debugOptions = { execArgv: ['--nolazy', '--inspect=6009'] };

    // Server options
    const serverOptions = {
        run: { module: serverModule, transport: TransportKind.ipc },
        debug: {
            module: serverModule,
            transport: TransportKind.ipc,
            options: debugOptions
        }
    };

    // Client options
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'nova' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/.clientrc')
        }
    };

    // Create and start the language client
    client = new LanguageClient(
        'novaLanguageServer',
        'Nova Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client (also launches the server)
    client.start();
}

function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

module.exports = {
    activate,
    deactivate
};