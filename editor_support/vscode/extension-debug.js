// Debug version of Nova extension - minimal completion only
const vscode = require('vscode');

function activate(context) {
    console.log('🚀 DEBUG Nova extension activated!');
    vscode.window.showInformationMessage('DEBUG Nova extension activated!');
    
    // Very simple completion provider
    const provider = vscode.languages.registerCompletionItemProvider(
        'nova', // Just the language ID
        {
            provideCompletionItems(document, position, token, context) {
                console.log('🔥 DEBUG completion triggered!');
                console.log('Document URI:', document.uri.toString());
                console.log('Language ID:', document.languageId);
                console.log('Position:', position.line + ':' + position.character);
                
                // Create basic completion items
                const items = [
                    {
                        label: 'print',
                        kind: vscode.CompletionItemKind.Function,
                        detail: 'print(value) -> void',
                        documentation: 'Print value to console',
                        insertText: 'print($1)',
                        insertTextFormat: vscode.InsertTextFormat.Snippet
                    },
                    {
                        label: 'let',
                        kind: vscode.CompletionItemKind.Keyword,
                        detail: 'Variable declaration',
                        documentation: 'Declare a variable'
                    },
                    {
                        label: 'fn',
                        kind: vscode.CompletionItemKind.Keyword,
                        detail: 'Function declaration',
                        documentation: 'Declare a function'
                    },
                    {
                        label: 'if',
                        kind: vscode.CompletionItemKind.Keyword,
                        detail: 'Conditional statement',
                        documentation: 'If conditional'
                    }
                ];
                
                console.log('📋 Returning completions:', items.map(i => i.label));
                return items;
            }
        },
        '.', ' ', '(' // Trigger characters
    );

    context.subscriptions.push(provider);
    
    // Test if provider registered
    setTimeout(() => {
        console.log('✅ DEBUG completion provider registered');
        console.log('📊 Subscriptions count:', context.subscriptions.length);
    }, 100);
}

function deactivate() {
    console.log('DEBUG Nova extension deactivated');
}

module.exports = { activate, deactivate };