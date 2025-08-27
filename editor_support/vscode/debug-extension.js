// Debug script to test extension functionality
const vscode = require('vscode');

// Test completion provider
async function testCompletionProvider() {
    console.log('🔍 Testing Nova Completion Provider...');
    
    // Get all registered languages
    const languages = vscode.languages.getLanguages();
    console.log('Registered languages:', await languages);
    
    // Check if nova is registered
    const novaRegistered = (await languages).includes('nova');
    console.log('Nova language registered:', novaRegistered);
    
    // Get active editor
    const editor = vscode.window.activeTextEditor;
    if (editor) {
        console.log('Active editor language:', editor.document.languageId);
        console.log('File path:', editor.document.fileName);
        
        if (editor.document.languageId === 'nova') {
            console.log('✅ Nova file is active');
            
            // Try to trigger completion manually
            const position = editor.selection.active;
            console.log('Current position:', position.line, position.character);
            
            // Insert a trigger character
            await editor.edit(editBuilder => {
                editBuilder.insert(position, 'pr');
            });
            
            console.log('Inserted completion trigger');
            
            // Wait and then trigger completion
            setTimeout(() => {
                vscode.commands.executeCommand('editor.action.triggerSuggest');
            }, 500);
        }
    } else {
        console.log('❌ No active editor');
    }
}

// Export test function
module.exports = { testCompletionProvider };