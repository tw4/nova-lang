// Simplified version for testing
const vscode = require('vscode');

function activate(context) {
    console.log('🚀 SIMPLE Nova extension activated!');
    
    // Minimal completion provider
    const provider = vscode.languages.registerCompletionItemProvider('nova', {
        provideCompletionItems() {
            console.log('🔥 SIMPLE completion triggered!');
            
            const items = [
                new vscode.CompletionItem('print', vscode.CompletionItemKind.Function),
                new vscode.CompletionItem('let', vscode.CompletionItemKind.Keyword),
                new vscode.CompletionItem('fn', vscode.CompletionItemKind.Keyword),
            ];
            
            console.log('Returning items:', items.length);
            return items;
        }
    });

    context.subscriptions.push(provider);
    console.log('✅ SIMPLE completion provider registered');
}

function deactivate() {}

module.exports = { activate, deactivate };