"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const node_1 = require("vscode-languageserver/node");
const vscode_languageserver_textdocument_1 = require("vscode-languageserver-textdocument");
// Create a connection for the server
const connection = (0, node_1.createConnection)(node_1.ProposedFeatures.all);
// Create a simple text document manager
const documents = new node_1.TextDocuments(vscode_languageserver_textdocument_1.TextDocument);
let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;
let hasDiagnosticRelatedInformationCapability = false;
connection.onInitialize((params) => {
    const capabilities = params.capabilities;
    hasConfigurationCapability = !!(capabilities.workspace && !!capabilities.workspace.configuration);
    hasWorkspaceFolderCapability = !!(capabilities.workspace && !!capabilities.workspace.workspaceFolders);
    hasDiagnosticRelatedInformationCapability = !!(capabilities.textDocument &&
        capabilities.textDocument.publishDiagnostics &&
        capabilities.textDocument.publishDiagnostics.relatedInformation);
    const result = {
        capabilities: {
            textDocumentSync: node_1.TextDocumentSyncKind.Incremental,
            completionProvider: {
                resolveProvider: true,
                triggerCharacters: ['.', '(']
            },
            hoverProvider: true,
            definitionProvider: true
        }
    };
    if (hasWorkspaceFolderCapability) {
        result.capabilities.workspace = {
            workspaceFolders: {
                supported: true
            }
        };
    }
    return result;
});
connection.onInitialized(() => {
    if (hasConfigurationCapability) {
        connection.client.register(node_1.DidChangeConfigurationNotification.type, undefined);
    }
    if (hasWorkspaceFolderCapability) {
        connection.workspace.onDidChangeWorkspaceFolders(_event => {
            connection.console.log('Workspace folder change event received.');
        });
    }
});
// Nova language keywords and built-ins
const novaKeywords = [
    'let', 'fn', 'if', 'else', 'while', 'for', 'in', 'return',
    'true', 'false', 'null', 'and', 'or', 'not',
    'class', 'extends', 'super', 'this', 'constructor',
    'private', 'public', 'static', 'new'
];
const novaBuiltins = [
    {
        name: 'print',
        detail: 'print(value): void',
        documentation: 'Print a value to the console'
    },
    {
        name: 'len',
        detail: 'len(collection): number',
        documentation: 'Get the length of a string or array'
    },
    {
        name: 'type',
        detail: 'type(value): string',
        documentation: 'Get the type name of a value'
    },
    {
        name: 'str',
        detail: 'str(value): string',
        documentation: 'Convert a value to string'
    },
    {
        name: 'num',
        detail: 'num(value): number',
        documentation: 'Convert a value to number'
    },
    {
        name: 'push',
        detail: 'push(array, value): array',
        documentation: 'Add an element to the end of an array'
    },
    {
        name: 'pop',
        detail: 'pop(array): any',
        documentation: 'Remove and return the last element of an array'
    },
    {
        name: 'split',
        detail: 'split(string, separator): array',
        documentation: 'Split a string into an array'
    },
    {
        name: 'join',
        detail: 'join(array, separator): string',
        documentation: 'Join array elements into a string'
    }
];
// Auto-completion provider
connection.onCompletion((_textDocumentPosition) => {
    const completions = [];
    // Add keywords
    novaKeywords.forEach(keyword => {
        completions.push({
            label: keyword,
            kind: node_1.CompletionItemKind.Keyword,
            data: keyword
        });
    });
    // Add built-in functions
    novaBuiltins.forEach((builtin, index) => {
        completions.push({
            label: builtin.name,
            kind: node_1.CompletionItemKind.Function,
            detail: builtin.detail,
            documentation: builtin.documentation,
            data: index + 1000
        });
    });
    return completions;
});
connection.onCompletionResolve((item) => {
    if (item.data >= 1000) {
        const index = item.data - 1000;
        if (index < novaBuiltins.length) {
            const builtin = novaBuiltins[index];
            item.detail = builtin.detail;
            item.documentation = builtin.documentation;
        }
    }
    return item;
});
// Hover provider
connection.onHover((params) => {
    const document = documents.get(params.textDocument.uri);
    if (!document) {
        return null;
    }
    const position = params.position;
    const text = document.getText();
    const lines = text.split('\n');
    const line = lines[position.line];
    if (!line)
        return null;
    // Get word at position
    const wordMatch = line.match(/\b\w+\b/g);
    if (!wordMatch)
        return null;
    let charIndex = 0;
    for (const word of wordMatch) {
        const wordStart = line.indexOf(word, charIndex);
        const wordEnd = wordStart + word.length;
        if (position.character >= wordStart && position.character <= wordEnd) {
            // Check if it's a built-in function
            const builtin = novaBuiltins.find(b => b.name === word);
            if (builtin) {
                return {
                    contents: {
                        kind: node_1.MarkupKind.Markdown,
                        value: `**${builtin.detail}**\n\n${builtin.documentation}`
                    }
                };
            }
            // Check if it's a keyword
            if (novaKeywords.includes(word)) {
                return {
                    contents: {
                        kind: node_1.MarkupKind.Markdown,
                        value: `**${word}** - Nova language keyword`
                    }
                };
            }
            break;
        }
        charIndex = wordEnd;
    }
    return null;
});
// Document change handler for diagnostics
documents.onDidChangeContent(change => {
    validateTextDocument(change.document);
});
async function validateTextDocument(textDocument) {
    const text = textDocument.getText();
    const diagnostics = [];
    // Simple syntax validation
    const lines = text.split('\n');
    lines.forEach((line, lineNumber) => {
        // Check for unmatched parentheses
        const openParens = (line.match(/\(/g) || []).length;
        const closeParens = (line.match(/\)/g) || []).length;
        if (openParens > closeParens) {
            diagnostics.push({
                severity: node_1.DiagnosticSeverity.Error,
                range: {
                    start: { line: lineNumber, character: line.lastIndexOf('(') },
                    end: { line: lineNumber, character: line.length }
                },
                message: 'Unmatched opening parenthesis',
                source: 'nova-ls'
            });
        }
        // Check for undefined variables (simple heuristic)
        const assignments = line.match(/let\s+(\w+)/g);
        if (assignments) {
            assignments.forEach(assignment => {
                const varName = assignment.replace(/let\s+/, '');
                // This is a simple example - in reality you'd track scope properly
            });
        }
    });
    connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}
// Make the text document manager listen on the connection
documents.listen(connection);
// Listen on the connection
connection.listen();
//# sourceMappingURL=server.js.map