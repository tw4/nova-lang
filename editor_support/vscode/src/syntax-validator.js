const vscode = require('vscode');

/**
 * Nova Syntax Validator
 * Provides real-time syntax validation and error highlighting
 */
class NovaSyntaxValidator {
    constructor() {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('nova');
        this.patterns = this.initializePatterns();
    }

    initializePatterns() {
        return {
            // Common syntax errors
            unclosedString: /"[^"]*$/gm,
            unclosedComment: /\/\*[^*]*$/gm,
            invalidFunctionDecl: /fn\s+[^a-zA-Z_]/g,
            invalidVariableDecl: /let\s+[^a-zA-Z_]/g,
            invalidClassDecl: /class\s+[^A-Z]/g,
            missingParens: /if\s*[^(]/g,
            missingBraces: /{\s*$/gm,
            trailingComma: /,\s*[}\]]/g,
            
            // Type-related errors
            invalidAssignment: /const\s+\w+\s*=\s*=/g,
            uninitializedConst: /const\s+\w+\s*;/g,
            
            // Best practices
            unusedVariable: /let\s+(\w+)\s*=.*?(?!\1)/g,
            magicNumbers: /\b(?!0|1)\d{2,}\b/g,
            longFunctions: /fn\s+\w+[^}]*{[^}]{500,}}/g,
            
            // Nova-specific patterns
            invalidMethodCall: /\.\s*[^a-zA-Z_]/g,
            invalidConstructor: /new\s+[^A-Z]/g,
            missingReturn: /fn\s+\w+.*{(?![^}]*return)[^}]*}/g
        };
    }

    /**
     * Validate Nova document and return diagnostics
     */
    validateDocument(document) {
        const text = document.getText();
        const diagnostics = [];
        const lines = text.split('\n');

        // Check for syntax errors
        this.checkUnclosedStrings(text, diagnostics, document);
        this.checkUnclosedComments(text, diagnostics, document);
        this.checkFunctionDeclarations(text, diagnostics, document);
        this.checkVariableDeclarations(text, diagnostics, document);
        this.checkClassDeclarations(text, diagnostics, document);
        this.checkControlStructures(text, diagnostics, document);
        this.checkBraces(lines, diagnostics);
        this.checkTrailingCommas(text, diagnostics, document);
        
        // Check for semantic errors
        this.checkConstAssignments(text, diagnostics, document);
        this.checkUninitializedConsts(text, diagnostics, document);
        
        // Best practice warnings
        this.checkUnusedVariables(text, diagnostics, document);
        this.checkMagicNumbers(text, diagnostics, document);
        this.checkMethodCalls(text, diagnostics, document);
        this.checkConstructors(text, diagnostics, document);

        return diagnostics;
    }

    checkUnclosedStrings(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.unclosedString)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                'Unclosed string literal',
                vscode.DiagnosticSeverity.Error
            ));
        });
    }

    checkUnclosedComments(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.unclosedComment)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                'Unclosed block comment',
                vscode.DiagnosticSeverity.Error
            ));
        });
    }

    checkFunctionDeclarations(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.invalidFunctionDecl)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                'Invalid function declaration. Function names must start with a letter or underscore',
                vscode.DiagnosticSeverity.Error
            ));
        });
    }

    checkVariableDeclarations(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.invalidVariableDecl)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                'Invalid variable declaration. Variable names must start with a letter or underscore',
                vscode.DiagnosticSeverity.Error
            ));
        });
    }

    checkClassDeclarations(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.invalidClassDecl)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                'Invalid class declaration. Class names must start with an uppercase letter',
                vscode.DiagnosticSeverity.Error
            ));
        });
    }

    checkControlStructures(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.missingParens)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                'Missing parentheses in control structure',
                vscode.DiagnosticSeverity.Error
            ));
        });
    }

    checkBraces(lines, diagnostics) {
        let braceStack = [];
        const braceMap = { '{': '}', '[': ']', '(': ')' };
        
        lines.forEach((line, lineNumber) => {
            for (let i = 0; i < line.length; i++) {
                const char = line[i];
                
                if (['{', '[', '('].includes(char)) {
                    braceStack.push({ char, line: lineNumber, column: i });
                } else if (['}', ']', ')'].includes(char)) {
                    if (braceStack.length === 0) {
                        diagnostics.push(new vscode.Diagnostic(
                            new vscode.Range(lineNumber, i, lineNumber, i + 1),
                            `Unmatched closing ${char}`,
                            vscode.DiagnosticSeverity.Error
                        ));
                    } else {
                        const opening = braceStack.pop();
                        if (braceMap[opening.char] !== char) {
                            diagnostics.push(new vscode.Diagnostic(
                                new vscode.Range(lineNumber, i, lineNumber, i + 1),
                                `Mismatched brace: expected ${braceMap[opening.char]} but found ${char}`,
                                vscode.DiagnosticSeverity.Error
                            ));
                        }
                    }
                }
            }
        });

        // Check for unclosed braces
        braceStack.forEach(opening => {
            diagnostics.push(new vscode.Diagnostic(
                new vscode.Range(opening.line, opening.column, opening.line, opening.column + 1),
                `Unclosed ${opening.char}`,
                vscode.DiagnosticSeverity.Error
            ));
        });
    }

    checkTrailingCommas(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.trailingComma)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                'Trailing comma before closing bracket',
                vscode.DiagnosticSeverity.Warning
            ));
        });
    }

    checkConstAssignments(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.invalidAssignment)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                'Cannot reassign to const variable',
                vscode.DiagnosticSeverity.Error
            ));
        });
    }

    checkUninitializedConsts(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.uninitializedConst)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                'Constants must be initialized when declared',
                vscode.DiagnosticSeverity.Error
            ));
        });
    }

    checkUnusedVariables(text, diagnostics, document) {
        const varDeclarations = [...text.matchAll(/let\s+(\w+)/g)];
        const varUsages = [...text.matchAll(/\b(\w+)\b/g)];
        
        const usedVars = new Set(varUsages.map(match => match[1]));
        
        varDeclarations.forEach(match => {
            const varName = match[1];
            if (!usedVars.has(varName) || text.indexOf(varName, match.index + match[0].length) === -1) {
                const position = document.positionAt(match.index);
                const range = new vscode.Range(position, position.translate(0, match[0].length));
                diagnostics.push(new vscode.Diagnostic(
                    range,
                    `Variable '${varName}' is declared but never used`,
                    vscode.DiagnosticSeverity.Information
                ));
            }
        });
    }

    checkMagicNumbers(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.magicNumbers)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                `Consider using a named constant instead of magic number ${match[0]}`,
                vscode.DiagnosticSeverity.Information
            ));
        });
    }

    checkMethodCalls(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.invalidMethodCall)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                'Invalid method call syntax',
                vscode.DiagnosticSeverity.Error
            ));
        });
    }

    checkConstructors(text, diagnostics, document) {
        const matches = [...text.matchAll(this.patterns.invalidConstructor)];
        matches.forEach(match => {
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position.translate(0, match[0].length));
            diagnostics.push(new vscode.Diagnostic(
                range,
                'Constructor calls must use a class name starting with uppercase letter',
                vscode.DiagnosticSeverity.Error
            ));
        });
    }

    /**
     * Update diagnostics for a document
     */
    updateDiagnostics(document) {
        if (document.languageId !== 'nova') {
            return;
        }

        const diagnostics = this.validateDocument(document);
        this.diagnosticCollection.set(document.uri, diagnostics);
    }

    /**
     * Clear diagnostics for a document
     */
    clearDiagnostics(uri) {
        this.diagnosticCollection.delete(uri);
    }

    /**
     * Dispose of the diagnostic collection
     */
    dispose() {
        this.diagnosticCollection.dispose();
    }
}

/**
 * Register syntax validation for Nova files
 */
function registerSyntaxValidation(context) {
    const validator = new NovaSyntaxValidator();

    // Validate on document open
    const onDidOpenTextDocument = vscode.workspace.onDidOpenTextDocument(document => {
        validator.updateDiagnostics(document);
    });

    // Validate on document change
    const onDidChangeTextDocument = vscode.workspace.onDidChangeTextDocument(event => {
        validator.updateDiagnostics(event.document);
    });

    // Clear diagnostics on document close
    const onDidCloseTextDocument = vscode.workspace.onDidCloseTextDocument(document => {
        validator.clearDiagnostics(document.uri);
    });

    // Validate all currently open documents
    vscode.workspace.textDocuments.forEach(document => {
        validator.updateDiagnostics(document);
    });

    context.subscriptions.push(
        validator,
        onDidOpenTextDocument,
        onDidChangeTextDocument,
        onDidCloseTextDocument
    );

    return validator;
}

module.exports = {
    NovaSyntaxValidator,
    registerSyntaxValidation
};