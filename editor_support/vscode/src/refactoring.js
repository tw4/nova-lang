const vscode = require('vscode');

/**
 * Nova Refactoring Provider
 * Provides code refactoring capabilities like rename symbol, extract function, etc.
 */
class NovaRefactoringProvider {
    constructor() {
        this.symbolCache = new Map();
    }

    /**
     * Parse document to find all symbols
     */
    parseSymbols(document) {
        const text = document.getText();
        const lines = text.split('\n');
        const symbols = {
            variables: new Map(),
            functions: new Map(),
            classes: new Map(),
            methods: new Map(),
            properties: new Map()
        };

        let currentClass = null;
        let braceDepth = 0;

        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            const trimmed = line.trim();

            // Track brace depth for scope
            braceDepth += (line.match(/\{/g) || []).length;
            braceDepth -= (line.match(/\}/g) || []).length;

            // Variable declarations
            const varMatches = [...line.matchAll(/(?:let|const)\s+([a-zA-Z_][a-zA-Z0-9_]*)/g)];
            varMatches.forEach(match => {
                const varName = match[1];
                const startPos = new vscode.Position(i, match.index + match[0].indexOf(varName));
                const endPos = new vscode.Position(i, match.index + match[0].indexOf(varName) + varName.length);
                const range = new vscode.Range(startPos, endPos);
                
                if (!symbols.variables.has(varName)) {
                    symbols.variables.set(varName, []);
                }
                symbols.variables.get(varName).push({
                    range,
                    type: 'declaration',
                    line: i
                });
            });

            // Function declarations
            const fnMatch = line.match(/fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
            if (fnMatch) {
                const fnName = fnMatch[1];
                const startPos = new vscode.Position(i, fnMatch.index + fnMatch[0].indexOf(fnName));
                const endPos = new vscode.Position(i, fnMatch.index + fnMatch[0].indexOf(fnName) + fnName.length);
                const range = new vscode.Range(startPos, endPos);
                
                if (!symbols.functions.has(fnName)) {
                    symbols.functions.set(fnName, []);
                }
                symbols.functions.get(fnName).push({
                    range,
                    type: 'declaration',
                    line: i,
                    class: currentClass
                });
            }

            // Class declarations
            const classMatch = line.match(/class\s+([A-Z][a-zA-Z0-9_]*)/);
            if (classMatch) {
                const className = classMatch[1];
                currentClass = className;
                const startPos = new vscode.Position(i, classMatch.index + classMatch[0].indexOf(className));
                const endPos = new vscode.Position(i, classMatch.index + classMatch[0].indexOf(className) + className.length);
                const range = new vscode.Range(startPos, endPos);
                
                if (!symbols.classes.has(className)) {
                    symbols.classes.set(className, []);
                }
                symbols.classes.get(className).push({
                    range,
                    type: 'declaration',
                    line: i
                });
            }

            // Method declarations (inside classes)
            if (currentClass && braceDepth > 0) {
                const methodMatch = line.match(/(?:public|private|static)?\s*(?:fn\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
                if (methodMatch && !line.includes('class') && !line.includes('let') && !line.includes('const')) {
                    const methodName = methodMatch[1];
                    if (methodName !== currentClass && methodName !== 'if' && methodName !== 'while' && methodName !== 'for') {
                        const startPos = new vscode.Position(i, methodMatch.index + methodMatch[0].indexOf(methodName));
                        const endPos = new vscode.Position(i, methodMatch.index + methodMatch[0].indexOf(methodName) + methodName.length);
                        const range = new vscode.Range(startPos, endPos);
                        
                        if (!symbols.methods.has(methodName)) {
                            symbols.methods.set(methodName, []);
                        }
                        symbols.methods.get(methodName).push({
                            range,
                            type: 'declaration',
                            line: i,
                            class: currentClass
                        });
                    }
                }
            }

            // Property access
            const propMatches = [...line.matchAll(/\.([a-zA-Z_][a-zA-Z0-9_]*)/g)];
            propMatches.forEach(match => {
                const propName = match[1];
                const startPos = new vscode.Position(i, match.index + 1);
                const endPos = new vscode.Position(i, match.index + 1 + propName.length);
                const range = new vscode.Range(startPos, endPos);
                
                if (!symbols.properties.has(propName)) {
                    symbols.properties.set(propName, []);
                }
                symbols.properties.get(propName).push({
                    range,
                    type: 'access',
                    line: i
                });
            });

            // Variable/function usage
            const usageMatches = [...line.matchAll(/\b([a-zA-Z_][a-zA-Z0-9_]*)\b/g)];
            usageMatches.forEach(match => {
                const symbolName = match[1];
                
                // Skip keywords and built-ins
                if (this.isKeywordOrBuiltin(symbolName)) return;
                
                const startPos = new vscode.Position(i, match.index);
                const endPos = new vscode.Position(i, match.index + symbolName.length);
                const range = new vscode.Range(startPos, endPos);
                
                // Add to variables if it exists
                if (symbols.variables.has(symbolName)) {
                    symbols.variables.get(symbolName).push({
                        range,
                        type: 'usage',
                        line: i
                    });
                }
                
                // Add to functions if it exists
                if (symbols.functions.has(symbolName)) {
                    symbols.functions.get(symbolName).push({
                        range,
                        type: 'usage',
                        line: i
                    });
                }
            });

            // Reset current class when exiting class scope
            if (currentClass && braceDepth === 0 && line.includes('}')) {
                currentClass = null;
            }
        }

        return symbols;
    }

    /**
     * Check if a symbol is a keyword or built-in
     */
    isKeywordOrBuiltin(symbol) {
        const keywords = [
            'let', 'const', 'fn', 'if', 'else', 'elif', 'while', 'for', 'in', 'do', 'loop',
            'return', 'break', 'continue', 'yield', 'true', 'false', 'null', 'undefined', 'void',
            'and', 'or', 'not', 'is', 'instanceof', 'typeof', 'try', 'catch', 'finally', 'throw', 'throws',
            'import', 'from', 'export', 'use', 'module', 'class', 'extends', 'implements', 'super',
            'this', 'self', 'static', 'private', 'protected', 'public', 'final', 'abstract', 'virtual', 'override',
            'async', 'await', 'as', 'new', 'delete', 'constructor', 'init', 'deinit',
            'struct', 'enum', 'interface', 'type', 'trait', 'match', 'when', 'where', 'select', 'with',
            'switch', 'case', 'default'
        ];
        
        const builtins = [
            'print', 'println', 'log', 'warn', 'error', 'type', 'str', 'num', 'bool', 'array',
            'len', 'push', 'pop', 'filter', 'map', 'reduce', 'sort', 'reverse',
            'split', 'join', 'trim', 'toLowerCase', 'toUpperCase', 'charAt', 'indexOf', 'substring', 'replace', 'contains',
            'abs', 'max', 'min', 'sqrt', 'pow', 'floor', 'ceil', 'round', 'random', 'sin', 'cos', 'tan', 'pi', 'e',
            'read', 'write', 'readFile', 'writeFile', 'exists', 'mkdir', 'rmdir'
        ];
        
        return keywords.includes(symbol) || builtins.includes(symbol);
    }

    /**
     * Find all references to a symbol
     */
    findReferences(document, position) {
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) return [];

        const word = document.getText(wordRange);
        const symbols = this.parseSymbols(document);
        const references = [];

        // Check all symbol types
        for (const [symbolName, locations] of symbols.variables) {
            if (symbolName === word) {
                references.push(...locations.map(loc => new vscode.Location(document.uri, loc.range)));
            }
        }

        for (const [symbolName, locations] of symbols.functions) {
            if (symbolName === word) {
                references.push(...locations.map(loc => new vscode.Location(document.uri, loc.range)));
            }
        }

        for (const [symbolName, locations] of symbols.classes) {
            if (symbolName === word) {
                references.push(...locations.map(loc => new vscode.Location(document.uri, loc.range)));
            }
        }

        for (const [symbolName, locations] of symbols.methods) {
            if (symbolName === word) {
                references.push(...locations.map(loc => new vscode.Location(document.uri, loc.range)));
            }
        }

        return references;
    }

    /**
     * Prepare rename symbol
     */
    prepareRename(document, position) {
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) {
            throw new Error('No symbol found at position');
        }

        const word = document.getText(wordRange);
        
        // Don't allow renaming keywords or built-ins
        if (this.isKeywordOrBuiltin(word)) {
            throw new Error('Cannot rename language keywords or built-in functions');
        }

        return wordRange;
    }

    /**
     * Provide rename edits
     */
    provideRenameEdits(document, position, newName) {
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) return null;

        const word = document.getText(wordRange);
        
        // Validate new name
        if (!this.isValidIdentifier(newName)) {
            throw new Error('Invalid identifier name');
        }

        const references = this.findReferences(document, position);
        const workspaceEdit = new vscode.WorkspaceEdit();

        references.forEach(ref => {
            workspaceEdit.replace(ref.uri, ref.range, newName);
        });

        return workspaceEdit;
    }

    /**
     * Check if a string is a valid identifier
     */
    isValidIdentifier(name) {
        return /^[a-zA-Z_][a-zA-Z0-9_]*$/.test(name) && !this.isKeywordOrBuiltin(name);
    }

    /**
     * Extract function refactoring
     */
    async extractFunction(document, range) {
        const selectedText = document.getText(range);
        if (!selectedText.trim()) {
            vscode.window.showErrorMessage('Please select some code to extract');
            return;
        }

        // Prompt for function name
        const functionName = await vscode.window.showInputBox({
            prompt: 'Enter function name',
            placeHolder: 'extractedFunction',
            validateInput: (value) => {
                if (!value) return 'Function name is required';
                if (!this.isValidIdentifier(value)) return 'Invalid function name';
                return null;
            }
        });

        if (!functionName) return;

        // Analyze selected code for variables
        const { parameters, returnValue } = this.analyzeExtractedCode(selectedText, document, range);

        // Generate function
        const paramStr = parameters.length > 0 ? parameters.join(', ') : '';
        const returnStr = returnValue ? `return ${returnValue};` : '';
        
        const extractedFunction = `
fn ${functionName}(${paramStr}) {
    ${selectedText.split('\n').map(line => '    ' + line).join('\n')}
    ${returnStr}
}`;

        // Create function call
        const functionCall = `${functionName}(${parameters.join(', ')})`;

        // Apply edits
        const workspaceEdit = new vscode.WorkspaceEdit();
        
        // Replace selected code with function call
        workspaceEdit.replace(document.uri, range, functionCall);
        
        // Insert function definition
        const insertPosition = this.findBestInsertPosition(document, range);
        workspaceEdit.insert(document.uri, insertPosition, extractedFunction + '\n\n');

        await vscode.workspace.applyEdit(workspaceEdit);
    }

    /**
     * Analyze extracted code for parameters and return value
     */
    analyzeExtractedCode(code, document, range) {
        // Simple analysis - could be more sophisticated
        const variables = [];
        const varMatches = [...code.matchAll(/\b([a-zA-Z_][a-zA-Z0-9_]*)\b/g)];
        
        varMatches.forEach(match => {
            const varName = match[1];
            if (!this.isKeywordOrBuiltin(varName) && !variables.includes(varName)) {
                // Check if variable is declared outside the selection
                const beforeSelection = document.getText(new vscode.Range(new vscode.Position(0, 0), range.start));
                if (beforeSelection.includes(`let ${varName}`) || beforeSelection.includes(`const ${varName}`)) {
                    variables.push(varName);
                }
            }
        });

        // Simple heuristic for return value
        let returnValue = null;
        const lines = code.split('\n');
        const lastLine = lines[lines.length - 1].trim();
        if (!lastLine.includes('=') && !lastLine.includes('print') && lastLine.length > 0) {
            const match = lastLine.match(/\b([a-zA-Z_][a-zA-Z0-9_]*)\b/);
            if (match) {
                returnValue = match[1];
            }
        }

        return { parameters: variables, returnValue };
    }

    /**
     * Find best position to insert extracted function
     */
    findBestInsertPosition(document, range) {
        // Insert before the current function or at the beginning of the file
        for (let i = range.start.line; i >= 0; i--) {
            const line = document.lineAt(i);
            if (line.text.trim().startsWith('fn ')) {
                return new vscode.Position(i, 0);
            }
        }
        return new vscode.Position(0, 0);
    }
}

/**
 * Nova Rename Provider
 */
class NovaRenameProvider {
    constructor() {
        this.refactoring = new NovaRefactoringProvider();
    }

    prepareRename(document, position) {
        return this.refactoring.prepareRename(document, position);
    }

    provideRenameEdits(document, position, newName) {
        return this.refactoring.provideRenameEdits(document, position, newName);
    }
}

/**
 * Nova Reference Provider
 */
class NovaReferenceProvider {
    constructor() {
        this.refactoring = new NovaRefactoringProvider();
    }

    provideReferences(document, position, context) {
        return this.refactoring.findReferences(document, position);
    }
}

/**
 * Nova Code Action Provider
 */
class NovaCodeActionProvider {
    constructor() {
        this.refactoring = new NovaRefactoringProvider();
    }

    provideCodeActions(document, range, context) {
        const actions = [];

        // Extract function action
        if (!range.isEmpty) {
            const extractFunctionAction = new vscode.CodeAction(
                'Extract Function',
                vscode.CodeActionKind.RefactorExtract
            );
            
            extractFunctionAction.command = {
                title: 'Extract Function',
                command: 'nova.extractFunction',
                arguments: [document.uri, range]
            };
            
            actions.push(extractFunctionAction);
        }

        return actions;
    }
}

/**
 * Register refactoring providers
 */
function registerRefactoring(context) {
    const renameProvider = new NovaRenameProvider();
    const referenceProvider = new NovaReferenceProvider();
    const codeActionProvider = new NovaCodeActionProvider();
    const refactoringProvider = new NovaRefactoringProvider();

    // Register providers
    const renameDisposable = vscode.languages.registerRenameProvider('nova', renameProvider);
    const referenceDisposable = vscode.languages.registerReferenceProvider('nova', referenceProvider);
    const codeActionDisposable = vscode.languages.registerCodeActionsProvider('nova', codeActionProvider);

    // Register commands
    const extractFunctionCommand = vscode.commands.registerCommand('nova.extractFunction', async (uri, range) => {
        const document = await vscode.workspace.openTextDocument(uri);
        await refactoringProvider.extractFunction(document, range);
    });

    context.subscriptions.push(
        renameDisposable,
        referenceDisposable,
        codeActionDisposable,
        extractFunctionCommand
    );

    return {
        renameProvider,
        referenceProvider,
        codeActionProvider,
        refactoringProvider
    };
}

module.exports = {
    NovaRefactoringProvider,
    NovaRenameProvider,
    NovaReferenceProvider,
    NovaCodeActionProvider,
    registerRefactoring
};