// Minimal test extension for debugging
const vscode = require('vscode');

function activate(context) {
    console.log('🚀 MINIMAL Nova extension activated!');
    vscode.window.showInformationMessage('Nova extension activated!');
    
    // Simple completion provider that definitely works
    const provider = vscode.languages.registerCompletionItemProvider(
        { scheme: 'file', language: 'nova' },
        {
            provideCompletionItems(document, position, token, context) {
                console.log('🔥 MINIMAL completion triggered!');
                console.log('Document URI:', document.uri.toString());
                console.log('Language ID:', document.languageId);
                console.log('Position:', position.line, position.character);
                
                // Create simple completion items
                const print = new vscode.CompletionItem('print', vscode.CompletionItemKind.Function);
                print.detail = 'print(value) -> void';
                print.documentation = 'Print value to console';
                
                const let_keyword = new vscode.CompletionItem('let', vscode.CompletionItemKind.Keyword);
                let_keyword.detail = 'Variable declaration';
                
                const fn_keyword = new vscode.CompletionItem('fn', vscode.CompletionItemKind.Keyword);
                fn_keyword.detail = 'Function declaration';
                
                const items = [print, let_keyword, fn_keyword];
                
                console.log('📋 Returning completions:', items.map(i => i.label));
                return items;
            }
        },
        '.', ' '  // Trigger characters
    );

    context.subscriptions.push(provider);
    
    // Test if provider registered successfully
    setTimeout(() => {
        console.log('✅ Completion provider registered');
        console.log('📊 Total subscriptions:', context.subscriptions.length);
    }, 100);
}

function deactivate() {
    console.log('Nova extension deactivated');
}

module.exports = { activate, deactivate };