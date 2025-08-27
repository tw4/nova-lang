const vscode = require('vscode');

/**
 * Nova Code Formatter
 * Provides code formatting and auto-indentation for Nova files
 */
class NovaFormatter {
    constructor() {
        this.indentSize = 4; // Default indent size
        this.useSpaces = true; // Use spaces instead of tabs
    }

    /**
     * Format the entire document
     */
    formatDocument(document) {
        const text = document.getText();
        const formatted = this.formatCode(text);
        
        const firstLine = document.lineAt(0);
        const lastLine = document.lineAt(document.lineCount - 1);
        const range = new vscode.Range(firstLine.range.start, lastLine.range.end);
        
        return [vscode.TextEdit.replace(range, formatted)];
    }

    /**
     * Format a specific range
     */
    formatRange(document, range) {
        const text = document.getText(range);
        const formatted = this.formatCode(text);
        return [vscode.TextEdit.replace(range, formatted)];
    }

    /**
     * Format code on typing
     */
    formatOnType(document, position, ch) {
        const edits = [];
        
        switch (ch) {
            case '}':
                return this.formatClosingBrace(document, position);
            case ';':
                return this.formatSemicolon(document, position);
            case '\n':
                return this.formatNewline(document, position);
            default:
                return edits;
        }
    }

    /**
     * Main code formatting function
     */
    formatCode(code) {
        const lines = code.split('\n');
        const formatted = [];
        let indentLevel = 0;
        let inString = false;
        let inComment = false;
        let inBlockComment = false;

        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            const trimmed = line.trim();
            
            // Skip empty lines but preserve them
            if (trimmed === '') {
                formatted.push('');
                continue;
            }

            // Handle string and comment states
            const { newInString, newInComment, newInBlockComment } = this.updateStates(
                trimmed, inString, inComment, inBlockComment
            );

            // Skip formatting if inside string or comment
            if (inString || inComment || inBlockComment) {
                formatted.push(line);
                inString = newInString;
                inComment = newInComment;
                inBlockComment = newInBlockComment;
                continue;
            }

            // Calculate indent level changes
            const indentChange = this.calculateIndentChange(trimmed);
            
            // Decrease indent for closing brackets at start of line
            if (this.startsWithClosingBracket(trimmed)) {
                indentLevel = Math.max(0, indentLevel - 1);
            }

            // Apply formatting
            const indentedLine = this.applyIndent(trimmed, indentLevel);
            formatted.push(indentedLine);

            // Update indent level
            indentLevel += indentChange;
            indentLevel = Math.max(0, indentLevel);

            // Update states
            inString = newInString;
            inComment = newInComment;
            inBlockComment = newInBlockComment;
        }

        return formatted.join('\n');
    }

    /**
     * Update string and comment states
     */
    updateStates(line, inString, inComment, inBlockComment) {
        let newInString = inString;
        let newInComment = inComment;
        let newInBlockComment = inBlockComment;

        for (let i = 0; i < line.length; i++) {
            const char = line[i];
            const nextChar = i + 1 < line.length ? line[i + 1] : '';
            const prevChar = i > 0 ? line[i - 1] : '';

            if (!newInString && !newInComment && !newInBlockComment) {
                if (char === '"' && prevChar !== '\\') {
                    newInString = true;
                } else if (char === '/' && nextChar === '/') {
                    newInComment = true;
                    break;
                } else if (char === '/' && nextChar === '*') {
                    newInBlockComment = true;
                    i++; // Skip next character
                }
            } else if (newInString) {
                if (char === '"' && prevChar !== '\\') {
                    newInString = false;
                }
            } else if (newInBlockComment) {
                if (char === '*' && nextChar === '/') {
                    newInBlockComment = false;
                    i++; // Skip next character
                }
            }
        }

        // Comments end at line end
        if (newInComment) {
            newInComment = false;
        }

        return { newInString, newInComment, newInBlockComment };
    }

    /**
     * Calculate indent level change for a line
     */
    calculateIndentChange(line) {
        let change = 0;
        
        // Increase indent for opening brackets
        const openingBrackets = (line.match(/[{([](?![^"]*"[^"]*$)/g) || []).length;
        const closingBrackets = (line.match(/[})\]](?![^"]*"[^"]*$)/g) || []).length;
        
        change += openingBrackets - closingBrackets;

        // Special cases for Nova syntax
        if (this.isControlStructure(line)) {
            if (!line.includes('{')) {
                change += 1; // Increase indent for control structures without braces
            }
        }

        if (this.isClassOrFunctionDeclaration(line)) {
            if (!line.includes('{')) {
                change += 1;
            }
        }

        return change;
    }

    /**
     * Check if line starts with closing bracket
     */
    startsWithClosingBracket(line) {
        return /^[})\]]/.test(line);
    }

    /**
     * Check if line is a control structure
     */
    isControlStructure(line) {
        return /^\s*(if|else|elif|while|for|try|catch|finally|switch|case|default)\b/.test(line);
    }

    /**
     * Check if line is a class or function declaration
     */
    isClassOrFunctionDeclaration(line) {
        return /^\s*(fn|class|interface|trait|enum)\b/.test(line);
    }

    /**
     * Apply indentation to a line
     */
    applyIndent(line, level) {
        const indent = this.useSpaces ? ' '.repeat(level * this.indentSize) : '\t'.repeat(level);
        return indent + line;
    }

    /**
     * Format closing brace
     */
    formatClosingBrace(document, position) {
        const line = document.lineAt(position.line);
        const lineText = line.text;
        const beforeBrace = lineText.substring(0, position.character - 1);
        const afterBrace = lineText.substring(position.character);

        // Only format if the closing brace is the only non-whitespace character before it
        if (beforeBrace.trim() === '') {
            const indentLevel = this.calculateIndentLevel(document, position.line) - 1;
            const newIndent = this.useSpaces ? ' '.repeat(Math.max(0, indentLevel) * this.indentSize) : '\t'.repeat(Math.max(0, indentLevel));
            const newLine = newIndent + '}' + afterBrace;
            
            const range = new vscode.Range(
                new vscode.Position(position.line, 0),
                new vscode.Position(position.line, lineText.length)
            );
            
            return [vscode.TextEdit.replace(range, newLine)];
        }
        
        return [];
    }

    /**
     * Format semicolon
     */
    formatSemicolon(document, position) {
        const line = document.lineAt(position.line);
        const lineText = line.text;
        
        // Add space after semicolon if not at end of line
        if (position.character < lineText.length && lineText[position.character] !== ' ') {
            return [vscode.TextEdit.insert(position, ' ')];
        }
        
        return [];
    }

    /**
     * Format newline (auto-indentation)
     */
    formatNewline(document, position) {
        const currentLine = document.lineAt(position.line);
        const currentLineText = currentLine.text;
        
        // Calculate indent for new line
        const indentLevel = this.calculateIndentLevel(document, position.line);
        const indent = this.useSpaces ? ' '.repeat(indentLevel * this.indentSize) : '\t'.repeat(indentLevel);
        
        // Check if we need to add extra indent
        let extraIndent = 0;
        const trimmed = currentLineText.trim();
        
        if (this.needsExtraIndent(trimmed)) {
            extraIndent = 1;
        }
        
        const totalIndent = this.useSpaces ? 
            ' '.repeat((indentLevel + extraIndent) * this.indentSize) : 
            '\t'.repeat(indentLevel + extraIndent);
        
        return [vscode.TextEdit.insert(position, totalIndent)];
    }

    /**
     * Check if line needs extra indent for next line
     */
    needsExtraIndent(line) {
        return /[{([]$/.test(line) || 
               /^\s*(if|else|elif|while|for|try|catch|finally)\b.*[^{]$/.test(line) ||
               /^\s*(fn|class)\b.*[^{]$/.test(line);
    }

    /**
     * Calculate current indent level
     */
    calculateIndentLevel(document, lineNumber) {
        let level = 0;
        
        for (let i = 0; i <= lineNumber; i++) {
            const line = document.lineAt(i);
            const text = line.text.trim();
            
            if (text === '' || text.startsWith('//') || text.startsWith('/*')) {
                continue;
            }
            
            // Count brackets
            const openBrackets = (text.match(/[{]/g) || []).length;
            const closeBrackets = (text.match(/[}]/g) || []).length;
            
            level += openBrackets - closeBrackets;
        }
        
        return Math.max(0, level);
    }
}

/**
 * Nova Document Formatting Provider
 */
class NovaDocumentFormattingEditProvider {
    constructor() {
        this.formatter = new NovaFormatter();
    }

    provideDocumentFormattingEdits(document, options, token) {
        this.formatter.useSpaces = options.insertSpaces;
        this.formatter.indentSize = options.tabSize;
        return this.formatter.formatDocument(document);
    }
}

/**
 * Nova Document Range Formatting Provider
 */
class NovaDocumentRangeFormattingEditProvider {
    constructor() {
        this.formatter = new NovaFormatter();
    }

    provideDocumentRangeFormattingEdits(document, range, options, token) {
        this.formatter.useSpaces = options.insertSpaces;
        this.formatter.indentSize = options.tabSize;
        return this.formatter.formatRange(document, range);
    }
}

/**
 * Nova On Type Formatting Provider
 */
class NovaOnTypeFormattingEditProvider {
    constructor() {
        this.formatter = new NovaFormatter();
    }

    provideOnTypeFormattingEdits(document, position, ch, options, token) {
        this.formatter.useSpaces = options.insertSpaces;
        this.formatter.indentSize = options.tabSize;
        return this.formatter.formatOnType(document, position, ch);
    }
}

/**
 * Register formatting providers
 */
function registerFormatting(context) {
    const documentFormattingProvider = new NovaDocumentFormattingEditProvider();
    const documentRangeFormattingProvider = new NovaDocumentRangeFormattingEditProvider();
    const onTypeFormattingProvider = new NovaOnTypeFormattingEditProvider();

    const documentFormattingDisposable = vscode.languages.registerDocumentFormattingEditProvider(
        'nova', 
        documentFormattingProvider
    );

    const documentRangeFormattingDisposable = vscode.languages.registerDocumentRangeFormattingEditProvider(
        'nova', 
        documentRangeFormattingProvider
    );

    const onTypeFormattingDisposable = vscode.languages.registerOnTypeFormattingEditProvider(
        'nova',
        onTypeFormattingProvider,
        '}', ';', '\n'
    );

    context.subscriptions.push(
        documentFormattingDisposable,
        documentRangeFormattingDisposable,
        onTypeFormattingDisposable
    );

    return {
        documentFormattingProvider,
        documentRangeFormattingProvider,
        onTypeFormattingProvider
    };
}

module.exports = {
    NovaFormatter,
    NovaDocumentFormattingEditProvider,
    NovaDocumentRangeFormattingEditProvider,
    NovaOnTypeFormattingEditProvider,
    registerFormatting
};