const vscode = require('vscode');
const path = require('path');

/**
 * Nova Symbol Provider
 * Provides workspace-wide symbol search and navigation
 */
class NovaSymbolProvider {
    constructor() {
        this.symbolCache = new Map();
        this.lastCacheUpdate = new Map();
        this.cacheTimeout = 5000; // 5 seconds
    }

    /**
     * Parse a Nova file for symbols
     */
    parseFileSymbols(document) {
        const text = document.getText();
        const lines = text.split('\n');
        const symbols = [];

        let currentClass = null;
        let currentFunction = null;
        let braceDepth = 0;

        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            const trimmed = line.trim();

            if (trimmed === '' || trimmed.startsWith('//') || trimmed.startsWith('/*')) {
                continue;
            }

            // Track brace depth
            braceDepth += (line.match(/\{/g) || []).length;
            braceDepth -= (line.match(/\}/g) || []).length;

            // Class declarations
            const classMatch = line.match(/class\s+([A-Z][a-zA-Z0-9_]*)/);
            if (classMatch) {
                const className = classMatch[1];
                currentClass = className;
                
                const startPos = new vscode.Position(i, classMatch.index + classMatch[0].indexOf(className));
                const range = new vscode.Range(startPos, startPos.translate(0, className.length));
                const location = new vscode.Location(document.uri, range);
                
                symbols.push(new vscode.SymbolInformation(
                    className,
                    vscode.SymbolKind.Class,
                    '',
                    location
                ));
                
                // Find class end for container range
                let classEnd = i;
                let tempBraceDepth = 0;
                for (let j = i; j < lines.length; j++) {
                    tempBraceDepth += (lines[j].match(/\{/g) || []).length;
                    tempBraceDepth -= (lines[j].match(/\}/g) || []).length;
                    if (tempBraceDepth === 0 && lines[j].includes('}')) {
                        classEnd = j;
                        break;
                    }
                }
                
                const containerRange = new vscode.Range(startPos, new vscode.Position(classEnd, lines[classEnd].length));
                symbols[symbols.length - 1].containerName = '';
                symbols[symbols.length - 1].location = new vscode.Location(document.uri, containerRange);
            }

            // Function declarations
            const fnMatch = line.match(/(?:(?:public|private|static|async)\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
            if (fnMatch) {
                const fnName = fnMatch[1];
                currentFunction = fnName;
                
                const startPos = new vscode.Position(i, fnMatch.index + fnMatch[0].indexOf(fnName));
                const range = new vscode.Range(startPos, startPos.translate(0, fnName.length));
                const location = new vscode.Location(document.uri, range);
                
                const symbolKind = currentClass ? vscode.SymbolKind.Method : vscode.SymbolKind.Function;
                const containerName = currentClass || '';
                
                symbols.push(new vscode.SymbolInformation(
                    fnName,
                    symbolKind,
                    containerName,
                    location
                ));
            }

            // Constructor declarations
            const constructorMatch = line.match(/constructor\s*\(/);
            if (constructorMatch && currentClass) {
                const startPos = new vscode.Position(i, constructorMatch.index);
                const range = new vscode.Range(startPos, startPos.translate(0, 'constructor'.length));
                const location = new vscode.Location(document.uri, range);
                
                symbols.push(new vscode.SymbolInformation(
                    'constructor',
                    vscode.SymbolKind.Constructor,
                    currentClass,
                    location
                ));
            }

            // Variable declarations
            const varMatches = [...line.matchAll(/(?:let|const)\s+([a-zA-Z_][a-zA-Z0-9_]*)/g)];
            varMatches.forEach(match => {
                const varName = match[1];
                const startPos = new vscode.Position(i, match.index + match[0].indexOf(varName));
                const range = new vscode.Range(startPos, startPos.translate(0, varName.length));
                const location = new vscode.Location(document.uri, range);
                
                const containerName = currentClass || currentFunction || '';
                
                symbols.push(new vscode.SymbolInformation(
                    varName,
                    vscode.SymbolKind.Variable,
                    containerName,
                    location
                ));
            });

            // Enum declarations
            const enumMatch = line.match(/enum\s+([A-Z][a-zA-Z0-9_]*)/);
            if (enumMatch) {
                const enumName = enumMatch[1];
                const startPos = new vscode.Position(i, enumMatch.index + enumMatch[0].indexOf(enumName));
                const range = new vscode.Range(startPos, startPos.translate(0, enumName.length));
                const location = new vscode.Location(document.uri, range);
                
                symbols.push(new vscode.SymbolInformation(
                    enumName,
                    vscode.SymbolKind.Enum,
                    '',
                    location
                ));
            }

            // Interface declarations
            const interfaceMatch = line.match(/interface\s+([A-Z][a-zA-Z0-9_]*)/);
            if (interfaceMatch) {
                const interfaceName = interfaceMatch[1];
                const startPos = new vscode.Position(i, interfaceMatch.index + interfaceMatch[0].indexOf(interfaceName));
                const range = new vscode.Range(startPos, startPos.translate(0, interfaceName.length));
                const location = new vscode.Location(document.uri, range);
                
                symbols.push(new vscode.SymbolInformation(
                    interfaceName,
                    vscode.SymbolKind.Interface,
                    '',
                    location
                ));
            }

            // Trait declarations
            const traitMatch = line.match(/trait\s+([A-Z][a-zA-Z0-9_]*)/);
            if (traitMatch) {
                const traitName = traitMatch[1];
                const startPos = new vscode.Position(i, traitMatch.index + traitMatch[0].indexOf(traitName));
                const range = new vscode.Range(startPos, startPos.translate(0, traitName.length));
                const location = new vscode.Location(document.uri, range);
                
                symbols.push(new vscode.SymbolInformation(
                    traitName,
                    vscode.SymbolKind.Interface, // Use Interface for traits
                    '',
                    location
                ));
            }

            // Reset context when exiting scopes
            if (currentClass && braceDepth === 0 && line.includes('}')) {
                currentClass = null;
            }
            if (currentFunction && braceDepth <= 1 && line.includes('}')) {
                currentFunction = null;
            }
        }

        return symbols;
    }

    /**
     * Get cached symbols or parse if needed
     */
    async getCachedSymbols(document) {
        const uri = document.uri.toString();
        const lastModified = document.version || 0;
        const cacheEntry = this.symbolCache.get(uri);
        const lastUpdate = this.lastCacheUpdate.get(uri) || 0;

        const now = Date.now();
        const shouldUpdate = !cacheEntry || 
                           (now - lastUpdate) > this.cacheTimeout ||
                           (cacheEntry.version !== lastModified);

        if (shouldUpdate) {
            const symbols = this.parseFileSymbols(document);
            this.symbolCache.set(uri, { symbols, version: lastModified });
            this.lastCacheUpdate.set(uri, now);
            return symbols;
        }

        return cacheEntry.symbols;
    }

    /**
     * Clear cache for a document
     */
    clearCache(uri) {
        const uriString = uri.toString();
        this.symbolCache.delete(uriString);
        this.lastCacheUpdate.delete(uriString);
    }

    /**
     * Provide document symbols
     */
    async provideDocumentSymbols(document) {
        if (document.languageId !== 'nova') {
            return [];
        }

        return await this.getCachedSymbols(document);
    }

    /**
     * Provide workspace symbols
     */
    async provideWorkspaceSymbols(query) {
        const symbols = [];
        
        // Get all Nova files in workspace
        const novaFiles = await vscode.workspace.findFiles('**/*.nova', '**/node_modules/**', 1000);
        
        for (const fileUri of novaFiles) {
            try {
                const document = await vscode.workspace.openTextDocument(fileUri);
                const fileSymbols = await this.getCachedSymbols(document);
                
                // Filter symbols by query
                const filteredSymbols = query 
                    ? fileSymbols.filter(symbol => 
                        symbol.name.toLowerCase().includes(query.toLowerCase()))
                    : fileSymbols;
                
                symbols.push(...filteredSymbols);
                
                // Limit results to avoid performance issues
                if (symbols.length > 500) {
                    break;
                }
            } catch (error) {
                console.warn(`Error processing file ${fileUri.fsPath}:`, error);
            }
        }

        return symbols.slice(0, 500); // Limit to 500 results
    }

    /**
     * Provide document symbol hierarchy (for outline)
     */
    async provideDocumentSymbolsHierarchy(document) {
        if (document.languageId !== 'nova') {
            return [];
        }

        const flatSymbols = await this.getCachedSymbols(document);
        const hierarchicalSymbols = [];
        const symbolMap = new Map();

        // Group symbols by container
        flatSymbols.forEach(symbol => {
            if (!symbol.containerName) {
                // Top-level symbol
                const docSymbol = new vscode.DocumentSymbol(
                    symbol.name,
                    '',
                    symbol.kind,
                    symbol.location.range,
                    symbol.location.range
                );
                hierarchicalSymbols.push(docSymbol);
                symbolMap.set(symbol.name, docSymbol);
            } else {
                // Nested symbol
                const container = symbolMap.get(symbol.containerName);
                if (container) {
                    const docSymbol = new vscode.DocumentSymbol(
                        symbol.name,
                        '',
                        symbol.kind,
                        symbol.location.range,
                        symbol.location.range
                    );
                    container.children.push(docSymbol);
                }
            }
        });

        return hierarchicalSymbols;
    }
}

/**
 * Nova Document Symbol Provider
 */
class NovaDocumentSymbolProvider {
    constructor() {
        this.symbolProvider = new NovaSymbolProvider();
    }

    async provideDocumentSymbols(document) {
        return await this.symbolProvider.provideDocumentSymbolsHierarchy(document);
    }
}

/**
 * Nova Workspace Symbol Provider
 */
class NovaWorkspaceSymbolProvider {
    constructor() {
        this.symbolProvider = new NovaSymbolProvider();
    }

    async provideWorkspaceSymbols(query) {
        return await this.symbolProvider.provideWorkspaceSymbols(query);
    }
}

/**
 * Nova Go to Symbol Provider
 */
class NovaGoToSymbolProvider {
    constructor() {
        this.symbolProvider = new NovaSymbolProvider();
    }

    async provideDefinition(document, position) {
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) return [];

        const word = document.getText(wordRange);
        const symbols = await this.symbolProvider.getCachedSymbols(document);
        
        // Find symbol definitions
        const definitions = symbols.filter(symbol => 
            symbol.name === word && 
            symbol.kind !== vscode.SymbolKind.Variable // Exclude variable usages
        );

        return definitions.map(symbol => symbol.location);
    }

    async provideDeclaration(document, position) {
        // For Nova, declarations are the same as definitions
        return this.provideDefinition(document, position);
    }
}

/**
 * Register symbol providers
 */
function registerSymbolProviders(context) {
    const documentSymbolProvider = new NovaDocumentSymbolProvider();
    const workspaceSymbolProvider = new NovaWorkspaceSymbolProvider();
    const goToSymbolProvider = new NovaGoToSymbolProvider();
    const symbolProviderInstance = new NovaSymbolProvider();

    // Register providers
    const documentSymbolDisposable = vscode.languages.registerDocumentSymbolProvider(
        'nova', 
        documentSymbolProvider
    );

    const workspaceSymbolDisposable = vscode.languages.registerWorkspaceSymbolProvider(
        workspaceSymbolProvider
    );

    const definitionDisposable = vscode.languages.registerDefinitionProvider(
        'nova',
        goToSymbolProvider
    );

    const declarationDisposable = vscode.languages.registerDeclarationProvider(
        'nova',
        goToSymbolProvider
    );

    // Clear cache when documents change
    const onDidChangeTextDocument = vscode.workspace.onDidChangeTextDocument(event => {
        if (event.document.languageId === 'nova') {
            symbolProviderInstance.clearCache(event.document.uri);
        }
    });

    const onDidCloseTextDocument = vscode.workspace.onDidCloseTextDocument(document => {
        if (document.languageId === 'nova') {
            symbolProviderInstance.clearCache(document.uri);
        }
    });

    // Commands for navigation
    const goToSymbolCommand = vscode.commands.registerCommand('nova.goToSymbol', async () => {
        await vscode.commands.executeCommand('workbench.action.showAllSymbols');
    });

    const goToSymbolInFileCommand = vscode.commands.registerCommand('nova.goToSymbolInFile', async () => {
        await vscode.commands.executeCommand('workbench.action.gotoSymbol');
    });

    const outlineCommand = vscode.commands.registerCommand('nova.showOutline', async () => {
        await vscode.commands.executeCommand('outline.focus');
    });

    context.subscriptions.push(
        documentSymbolDisposable,
        workspaceSymbolDisposable,
        definitionDisposable,
        declarationDisposable,
        onDidChangeTextDocument,
        onDidCloseTextDocument,
        goToSymbolCommand,
        goToSymbolInFileCommand,
        outlineCommand
    );

    return {
        documentSymbolProvider,
        workspaceSymbolProvider,
        goToSymbolProvider,
        symbolProvider: symbolProviderInstance
    };
}

module.exports = {
    NovaSymbolProvider,
    NovaDocumentSymbolProvider,
    NovaWorkspaceSymbolProvider,
    NovaGoToSymbolProvider,
    registerSymbolProviders
};