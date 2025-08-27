const vscode = require('vscode');
const { LanguageClient, TransportKind } = require('vscode-languageclient/node');
const { registerDebugSupport } = require('./src/debug-provider');
const { registerSyntaxValidation } = require('./src/syntax-validator');
const { registerFormatting } = require('./src/formatter');
const { registerRefactoring } = require('./src/refactoring');
const { registerSymbolProviders } = require('./src/symbol-provider');
const { registerProjectScaffolding } = require('./src/project-scaffolding');

// Nova keywords - expanded set
const NOVA_KEYWORDS = [
    'let', 'const', 'fn', 'if', 'else', 'elif', 'while', 'for', 'in', 'do', 'loop',
    'return', 'break', 'continue', 'yield', 'true', 'false', 'null', 'undefined', 'void',
    'and', 'or', 'not', 'is', 'instanceof', 'typeof', 'try', 'catch', 'finally', 'throw', 'throws',
    'import', 'from', 'export', 'use', 'module', 'class', 'extends', 'implements', 'super',
    'this', 'self', 'static', 'private', 'protected', 'public', 'final', 'abstract', 'virtual', 'override',
    'async', 'await', 'as', 'new', 'delete', 'constructor', 'init', 'deinit',
    'struct', 'enum', 'interface', 'type', 'trait', 'match', 'when', 'where', 'select', 'with',
    'switch', 'case', 'default'
];

// Built-in functions - organized by category
const NOVA_BUILTINS = [
    // Console & Debugging
    { name: 'print', detail: 'print(value) -> void', documentation: 'Print a value to the console', category: 'console' },
    { name: 'println', detail: 'println(value) -> void', documentation: 'Print a value with newline', category: 'console' },
    { name: 'log', detail: 'log(value) -> void', documentation: 'Log a value for debugging', category: 'console' },
    { name: 'warn', detail: 'warn(message) -> void', documentation: 'Log a warning message', category: 'console' },
    { name: 'error', detail: 'error(message) -> void', documentation: 'Log an error message', category: 'console' },

    // Type System
    { name: 'type', detail: 'type(value) -> string', documentation: 'Get the type name of a value', category: 'type' },
    { name: 'str', detail: 'str(value) -> string', documentation: 'Convert a value to string', category: 'type' },
    { name: 'num', detail: 'num(value) -> number', documentation: 'Convert a value to number', category: 'type' },
    { name: 'bool', detail: 'bool(value) -> boolean', documentation: 'Convert a value to boolean', category: 'type' },
    { name: 'array', detail: 'array(values...) -> array', documentation: 'Create an array from values', category: 'type' },

    // Collection Operations
    { name: 'len', detail: 'len(collection) -> number', documentation: 'Get the length of a string or array', category: 'collection' },
    { name: 'push', detail: 'push(array, value) -> array', documentation: 'Add an element to the end of an array', category: 'collection' },
    { name: 'pop', detail: 'pop(array) -> any', documentation: 'Remove and return the last element', category: 'collection' },
    { name: 'filter', detail: 'filter(array, predicate) -> array', documentation: 'Filter array elements', category: 'collection' },
    { name: 'map', detail: 'map(array, transform) -> array', documentation: 'Transform array elements', category: 'collection' },
    { name: 'reduce', detail: 'reduce(array, accumulator, initial) -> any', documentation: 'Reduce array to single value', category: 'collection' },
    { name: 'sort', detail: 'sort(array, compareFn?) -> array', documentation: 'Sort array elements', category: 'collection' },
    { name: 'reverse', detail: 'reverse(array) -> array', documentation: 'Reverse array elements', category: 'collection' },

    // String Operations
    { name: 'split', detail: 'split(string, separator) -> array', documentation: 'Split a string into an array', category: 'string' },
    { name: 'join', detail: 'join(array, separator) -> string', documentation: 'Join array elements into a string', category: 'string' },
    { name: 'trim', detail: 'trim(string) -> string', documentation: 'Remove whitespace from both ends', category: 'string' },
    { name: 'toLowerCase', detail: 'toLowerCase(string) -> string', documentation: 'Convert to lowercase', category: 'string' },
    { name: 'toUpperCase', detail: 'toUpperCase(string) -> string', documentation: 'Convert to uppercase', category: 'string' },
    { name: 'charAt', detail: 'charAt(string, index) -> string', documentation: 'Get character at index', category: 'string' },
    { name: 'indexOf', detail: 'indexOf(string, substring) -> number', documentation: 'Find index of substring', category: 'string' },
    { name: 'substring', detail: 'substring(string, start, end?) -> string', documentation: 'Extract substring', category: 'string' },
    { name: 'replace', detail: 'replace(string, search, replace) -> string', documentation: 'Replace text in string', category: 'string' },
    { name: 'contains', detail: 'contains(string, substring) -> boolean', documentation: 'Check if string contains substring', category: 'string' },

    // Math Operations
    { name: 'abs', detail: 'abs(number) -> number', documentation: 'Get the absolute value', category: 'math' },
    { name: 'max', detail: 'max(a, b) -> number', documentation: 'Get the maximum of two numbers', category: 'math' },
    { name: 'min', detail: 'min(a, b) -> number', documentation: 'Get the minimum of two numbers', category: 'math' },
    { name: 'sqrt', detail: 'sqrt(number) -> number', documentation: 'Get the square root', category: 'math' },
    { name: 'pow', detail: 'pow(base, exponent) -> number', documentation: 'Raise base to the power of exponent', category: 'math' },
    { name: 'floor', detail: 'floor(number) -> number', documentation: 'Round down to nearest integer', category: 'math' },
    { name: 'ceil', detail: 'ceil(number) -> number', documentation: 'Round up to nearest integer', category: 'math' },
    { name: 'round', detail: 'round(number) -> number', documentation: 'Round to nearest integer', category: 'math' },
    { name: 'random', detail: 'random() -> number', documentation: 'Generate random number [0,1)', category: 'math' },
    { name: 'sin', detail: 'sin(radians) -> number', documentation: 'Sine of angle in radians', category: 'math' },
    { name: 'cos', detail: 'cos(radians) -> number', documentation: 'Cosine of angle in radians', category: 'math' },
    { name: 'tan', detail: 'tan(radians) -> number', documentation: 'Tangent of angle in radians', category: 'math' },
    { name: 'pi', detail: 'pi -> number', documentation: 'Mathematical constant π', category: 'math' },
    { name: 'e', detail: 'e -> number', documentation: 'Mathematical constant e', category: 'math' },

    // File I/O Operations
    { name: 'read', detail: 'read(path) -> string', documentation: 'Read file content as string', category: 'io' },
    { name: 'write', detail: 'write(path, content) -> void', documentation: 'Write content to file', category: 'io' },
    { name: 'readFile', detail: 'readFile(path) -> string', documentation: 'Read entire file content', category: 'io' },
    { name: 'writeFile', detail: 'writeFile(path, content) -> void', documentation: 'Write content to file', category: 'io' },
    { name: 'exists', detail: 'exists(path) -> boolean', documentation: 'Check if file/directory exists', category: 'io' },
    { name: 'mkdir', detail: 'mkdir(path) -> void', documentation: 'Create directory', category: 'io' },
    { name: 'rmdir', detail: 'rmdir(path) -> void', documentation: 'Remove directory', category: 'io' }
];

// Code snippets
const NOVA_SNIPPETS = [
    {
        label: 'class',
        insertText: new vscode.SnippetString('class ${1:ClassName} {\n\tconstructor(${2:params}) {\n\t\t${3:// initialization}\n\t}\n\n\t${4:// methods}\n}'),
        documentation: 'Create a new class with constructor',
        kind: vscode.CompletionItemKind.Snippet
    },
    {
        label: 'class-extends',
        insertText: new vscode.SnippetString('class ${1:ClassName} extends ${2:SuperClass} {\n\tconstructor(${3:params}) {\n\t\tsuper(${4:superParams});\n\t\t${5:// initialization}\n\t}\n\n\t${6:// methods}\n}'),
        documentation: 'Create a class that extends another class',
        kind: vscode.CompletionItemKind.Snippet
    },
    {
        label: 'fn',
        insertText: new vscode.SnippetString('fn ${1:functionName}(${2:params}) {\n\t${3:// function body}\n\treturn ${4:result};\n}'),
        documentation: 'Create a function',
        kind: vscode.CompletionItemKind.Snippet
    },
    {
        label: 'if',
        insertText: new vscode.SnippetString('if (${1:condition}) {\n\t${2:// if body}\n}'),
        documentation: 'If statement',
        kind: vscode.CompletionItemKind.Snippet
    },
    {
        label: 'if-else',
        insertText: new vscode.SnippetString('if (${1:condition}) {\n\t${2:// if body}\n} else {\n\t${3:// else body}\n}'),
        documentation: 'If-else statement',
        kind: vscode.CompletionItemKind.Snippet
    },
    {
        label: 'while',
        insertText: new vscode.SnippetString('while (${1:condition}) {\n\t${2:// loop body}\n}'),
        documentation: 'While loop',
        kind: vscode.CompletionItemKind.Snippet
    },
    {
        label: 'for',
        insertText: new vscode.SnippetString('for (${1:item} in ${2:collection}) {\n\t${3:// loop body}\n}'),
        documentation: 'For-in loop',
        kind: vscode.CompletionItemKind.Snippet
    },
    {
        label: 'try-catch',
        insertText: new vscode.SnippetString('try {\n\t${1:// try body}\n} catch (${2:error}) {\n\t${3:// error handling}\n}'),
        documentation: 'Try-catch block',
        kind: vscode.CompletionItemKind.Snippet
    },
    {
        label: 'constructor',
        insertText: new vscode.SnippetString('constructor(${1:params}) {\n\t${2:// initialization}\n}'),
        documentation: 'Class constructor',
        kind: vscode.CompletionItemKind.Snippet
    },
    {
        label: 'method',
        insertText: new vscode.SnippetString('${1:methodName}(${2:params}) {\n\t${3:// method body}\n\treturn ${4:result};\n}'),
        documentation: 'Class method',
        kind: vscode.CompletionItemKind.Snippet
    }
];

/**
 * Parse document to find variables, functions, and classes
 */
function parseDocumentSymbols(document) {
    const text = document.getText();
    const lines = text.split('\n');
    
    const symbols = {
        variables: new Set(),
        functions: new Map(),
        classes: new Map(),
        methods: new Map() // className -> method names
    };
    
    let currentClass = null;
    let braceDepth = 0;
    
    for (let i = 0; i < lines.length; i++) {
        const line = lines[i].trim();
        
        // Track brace depth for scope
        braceDepth += (line.match(/\{/g) || []).length;
        braceDepth -= (line.match(/\}/g) || []).length;
        
        // Variable declarations: let varName = ...
        const varMatch = line.match(/let\s+([a-zA-Z_][a-zA-Z0-9_]*)/g);
        if (varMatch) {
            varMatch.forEach(match => {
                const varName = match.replace('let ', '').trim();
                symbols.variables.add(varName);
                console.log(`🔍 Found variable: ${varName}`);
            });
        }
        
        // Constant declarations: const CONST_NAME = ...
        const constMatch = line.match(/const\s+([a-zA-Z_][a-zA-Z0-9_]*)/g);
        if (constMatch) {
            constMatch.forEach(match => {
                const constName = match.replace('const ', '').trim();
                symbols.variables.add(constName);
                console.log(`🔍 Found constant: ${constName}`);
            });
        }
        
        // Function declarations: fn functionName(...)
        const fnMatch = line.match(/fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
        if (fnMatch) {
            const fnName = fnMatch[1];
            symbols.functions.set(fnName, { line: i, params: [] });
            console.log(`🔍 Found function: ${fnName}`);
        }
        
        // Class declarations: class ClassName
        const classMatch = line.match(/class\s+([A-Z][a-zA-Z0-9_]*)/);
        if (classMatch) {
            const className = classMatch[1];
            currentClass = className;
            symbols.classes.set(className, { line: i, methods: new Set() });
            symbols.methods.set(className, new Set());
            console.log(`🔍 Found class: ${className}`);
        }
        
        // Method declarations inside classes
        if (currentClass && braceDepth > 0) {
            // Constructor
            if (line.includes('constructor(')) {
                symbols.classes.get(currentClass).methods.add('constructor');
                symbols.methods.get(currentClass).add('constructor');
                console.log(`🔍 Found constructor in ${currentClass}`);
            }
            
            // Regular methods: methodName(...) or fn methodName(...) or public/private fn methodName(...)
            const methodMatch = line.match(/(?:(?:public|private|static)\s+)?(?:fn\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
            if (methodMatch && 
                !line.includes('class ') && 
                !line.includes('let ') && 
                !line.includes('const ') &&
                !line.includes('if ') &&
                !line.includes('while ') &&
                !line.includes('for ')) {
                
                const methodName = methodMatch[1];
                if (methodName !== 'constructor' && 
                    methodName !== currentClass && 
                    methodName !== 'if' && 
                    methodName !== 'while' && 
                    methodName !== 'for') {
                    symbols.classes.get(currentClass).methods.add(methodName);
                    symbols.methods.get(currentClass).add(methodName);
                    console.log(`🔍 Found method: ${currentClass}.${methodName}`);
                }
            }
        }
        
        // Reset current class when we exit class scope
        if (currentClass && braceDepth === 0 && line.includes('}')) {
            currentClass = null;
        }
    }
    
    return symbols;
}

/**
 * Get completions for object property access
 */
function getPropertyCompletions(beforeCursor, symbols) {
    const completions = [];
    
    // Check if we're accessing a property/method
    const propertyMatch = beforeCursor.match(/([a-zA-Z_][a-zA-Z0-9_]*)\.\s*$/);
    if (propertyMatch) {
        const objectName = propertyMatch[1];
        console.log(`🔍 Property access on: ${objectName}`);
        
        // Check if it's a known class instance
        for (const [className, classInfo] of symbols.classes) {
            if (symbols.methods.has(className)) {
                symbols.methods.get(className).forEach(methodName => {
                    const item = new vscode.CompletionItem(methodName, vscode.CompletionItemKind.Method);
                    item.detail = `${className}.${methodName}()`;
                    item.documentation = `Method of ${className} class`;
                    item.sortText = '0' + methodName; // High priority
                    completions.push(item);
                });
            }
        }
        
        // Add common properties for all objects
        const commonProperties = [
            { name: 'length', kind: vscode.CompletionItemKind.Property, doc: 'Get length of object' },
            { name: 'toString', kind: vscode.CompletionItemKind.Method, doc: 'Convert to string' },
            { name: 'valueOf', kind: vscode.CompletionItemKind.Method, doc: 'Get primitive value' }
        ];
        
        commonProperties.forEach(prop => {
            const item = new vscode.CompletionItem(prop.name, prop.kind);
            item.documentation = prop.doc;
            item.sortText = '2' + prop.name;
            completions.push(item);
        });
    }
    
    return completions;
}

// Global language client instance
let languageClient = null;

/**
 * Start the language server
 */
async function startLanguageServer(context) {
    try {
        const fs = require('fs');
        const serverPath = context.asAbsolutePath('server-rust/target/release/nova-language-server');
        
        console.log('🔍 Looking for language server at:', serverPath);
        
        // Check if server binary exists
        if (!fs.existsSync(serverPath)) {
            console.warn('❌ Language server binary not found at:', serverPath);
            throw new Error(`Language server binary not found: ${serverPath}`);
        }
        
        console.log('✅ Language server binary found');
        
        const serverOptions = {
            run: { command: serverPath },
            debug: { command: serverPath }
        };

        const clientOptions = {
            documentSelector: [{ scheme: 'file', language: 'nova' }],
            synchronize: {
                fileEvents: vscode.workspace.createFileSystemWatcher('**/*.nova')
            }
        };

        languageClient = new LanguageClient(
            'nova-language-server',
            'Nova Language Server',
            serverOptions,
            clientOptions
        );

        await languageClient.start();
        console.log('✅ Nova Language Server started successfully');
        return true;
    } catch (error) {
        console.warn('❌ Failed to start Nova Language Server:', error.message);
        return false;
    }
}

/**
 * Stop the language server
 */
async function stopLanguageServer() {
    if (languageClient) {
        await languageClient.stop();
        languageClient = null;
    }
}

/**
 * Activation function for the extension
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
    console.log('🚀 Nova Language Support extension is now active!');
    
    // Show activation message
    vscode.window.showInformationMessage('Nova Language Support activated!');
    
    // Document symbol cache
    const documentSymbols = new Map();
    
    // Disable language server for now - using fallback providers
    console.log('🔧 Using JavaScript fallback providers (Language server disabled)');

    // Register completion provider
    const completionProvider = vscode.languages.registerCompletionItemProvider(
        { scheme: 'file', language: 'nova' },
        {
            provideCompletionItems(document, position, token, context) {
                console.log('🔥 Nova completion provider triggered!');
                console.log('Document language:', document.languageId);
                console.log('Position:', position.line, position.character);
                
                const completions = [];
                
                // Parse document symbols
                const symbols = parseDocumentSymbols(document);
                documentSymbols.set(document.uri.toString(), symbols);

                // Add keywords
                NOVA_KEYWORDS.forEach(keyword => {
                    const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
                    item.detail = `Nova keyword: ${keyword}`;
                    item.sortText = '1' + keyword; // Higher priority
                    completions.push(item);
                });

                // Add built-in functions
                NOVA_BUILTINS.forEach(builtin => {
                    const item = new vscode.CompletionItem(builtin.name, vscode.CompletionItemKind.Function);
                    item.detail = builtin.detail;
                    item.documentation = new vscode.MarkdownString(builtin.documentation);
                    item.sortText = '2' + builtin.name;
                    completions.push(item);
                });

                // Add snippets
                NOVA_SNIPPETS.forEach(snippet => {
                    const item = new vscode.CompletionItem(snippet.label, snippet.kind);
                    item.insertText = snippet.insertText;
                    item.documentation = new vscode.MarkdownString(snippet.documentation);
                    item.sortText = '0' + snippet.label; // Highest priority
                    completions.push(item);
                });
                
                // Add user-defined variables
                symbols.variables.forEach(varName => {
                    const item = new vscode.CompletionItem(varName, vscode.CompletionItemKind.Variable);
                    item.detail = `Variable: ${varName}`;
                    item.documentation = 'User-defined variable';
                    item.sortText = '1' + varName;
                    completions.push(item);
                });
                
                // Add user-defined functions
                symbols.functions.forEach((fnInfo, fnName) => {
                    const item = new vscode.CompletionItem(fnName, vscode.CompletionItemKind.Function);
                    item.detail = `Function: ${fnName}()`;
                    item.documentation = 'User-defined function';
                    item.sortText = '1' + fnName;
                    completions.push(item);
                });
                
                // Add class names
                symbols.classes.forEach((classInfo, className) => {
                    const item = new vscode.CompletionItem(className, vscode.CompletionItemKind.Class);
                    item.detail = `Class: ${className}`;
                    item.documentation = 'User-defined class';
                    item.sortText = '1' + className;
                    completions.push(item);
                });

                // Context-sensitive completions
                const lineText = document.lineAt(position.line).text;
                const beforeCursor = lineText.substring(0, position.character).trim();

                // After 'class' keyword, suggest class name pattern
                if (beforeCursor.endsWith('class ')) {
                    const classNameItem = new vscode.CompletionItem('ClassName', vscode.CompletionItemKind.Class);
                    classNameItem.insertText = new vscode.SnippetString('${1:ClassName}');
                    classNameItem.documentation = 'Class name pattern';
                    classNameItem.sortText = '00class';
                    completions.push(classNameItem);
                }

                // After class name, suggest 'extends'
                if (beforeCursor.match(/class\s+[A-Za-z_][A-Za-z0-9_]*\s*$/)) {
                    const extendsItem = new vscode.CompletionItem('extends', vscode.CompletionItemKind.Keyword);
                    extendsItem.detail = 'extends SuperClass';
                    extendsItem.documentation = 'Inherit from a parent class';
                    extendsItem.sortText = '00extends';
                    completions.push(extendsItem);
                }

                // Property access completion
                if (beforeCursor.includes('.')) {
                    const propertyCompletions = getPropertyCompletions(beforeCursor, symbols);
                    completions.push(...propertyCompletions);
                }

                console.log(`🔥 Nova completion: returning ${completions.length} items`);
                console.log('Sample completions:', completions.slice(0, 5).map(c => c.label));
                return completions;
            }
        },
        '.', ' ', '('  // Trigger characters
    );

    // Register hover provider
    const hoverProvider = vscode.languages.registerHoverProvider('nova', {
        provideHover(document, position, token) {
            const wordRange = document.getWordRangeAtPosition(position);
            if (!wordRange) {
                return;
            }

            const word = document.getText(wordRange);

            // Check for built-in functions
            const builtin = NOVA_BUILTINS.find(b => b.name === word);
            if (builtin) {
                const markdown = new vscode.MarkdownString();
                markdown.appendCodeblock(builtin.detail, 'nova');
                markdown.appendMarkdown('\\n\\n' + builtin.documentation);
                return new vscode.Hover(markdown);
            }

            // Check for keywords
            if (NOVA_KEYWORDS.includes(word)) {
                const keywordDocs = {
                    'class': 'Define a new class\\n\\n```nova\\nclass MyClass {\\n    constructor(param) {\\n        this.param = param;\\n    }\\n}\\n```',
                    'fn': 'Define a function\\n\\n```nova\\nfn myFunction(param) {\\n    return param * 2;\\n}\\n```',
                    'let': 'Declare a mutable variable\\n\\n```nova\\nlet myVar = \"Hello World\";\\n```',
                    'const': 'Declare a constant variable\\n\\n```nova\\nconst PI = 3.14159;\\n```',
                    'if': 'Conditional statement\\n\\n```nova\\nif (condition) {\\n    // code\\n}\\n```',
                    'extends': 'Class inheritance\\n\\n```nova\\nclass Child extends Parent {\\n    // child class\\n}\\n```',
                    'constructor': 'Class constructor\\n\\n```nova\\nconstructor(params) {\\n    this.property = params;\\n}\\n```',
                    'super': 'Call parent class method or constructor\\n\\n```nova\\nsuper(args);\\nsuper.methodName(args);\\n```',
                    'this': 'Reference to current instance\\n\\n```nova\\nthis.propertyName\\nthis.methodName(args)\\n```'
                };

                if (keywordDocs[word]) {
                    return new vscode.Hover(new vscode.MarkdownString(keywordDocs[word]));
                } else {
                    return new vscode.Hover(new vscode.MarkdownString(`**${word}** - Nova language keyword`));
                }
            }

            return;
        }
    });

    // Register definition provider (basic)
    const definitionProvider = vscode.languages.registerDefinitionProvider('nova', {
        provideDefinition(document, position, token) {
            // Basic implementation - could be expanded with actual parsing
            const wordRange = document.getWordRangeAtPosition(position);
            if (!wordRange) {
                return;
            }

            const word = document.getText(wordRange);
            const text = document.getText();
            
            // Look for function definitions
            const functionPattern = new RegExp(`fn\\s+${word}\\s*\\(`, 'g');
            let match;
            while ((match = functionPattern.exec(text)) !== null) {
                const pos = document.positionAt(match.index);
                return new vscode.Location(document.uri, pos);
            }

            // Look for class definitions
            const classPattern = new RegExp(`class\\s+${word}\\b`, 'g');
            while ((match = classPattern.exec(text)) !== null) {
                const pos = document.positionAt(match.index);
                return new vscode.Location(document.uri, pos);
            }

            return;
        }
    });

    // Document change listener to update symbols
    const documentChangeListener = vscode.workspace.onDidChangeTextDocument(event => {
        if (event.document.languageId === 'nova') {
            console.log('🔄 Document changed, updating symbols...');
            // Clear cached symbols for this document
            documentSymbols.delete(event.document.uri.toString());
        }
    });
    
    // Document open listener
    const documentOpenListener = vscode.workspace.onDidOpenTextDocument(document => {
        if (document.languageId === 'nova') {
            console.log('📂 Nova document opened, parsing symbols...');
            const symbols = parseDocumentSymbols(document);
            documentSymbols.set(document.uri.toString(), symbols);
        }
    });

    // Register commands
    const restartLanguageServerCommand = vscode.commands.registerCommand('nova.restartLanguageServer', async () => {
        vscode.window.showInformationMessage('Restarting Nova Language Server...');
        await stopLanguageServer();
        const started = await startLanguageServer(context);
        if (started) {
            vscode.window.showInformationMessage('Nova Language Server restarted successfully');
        } else {
            vscode.window.showErrorMessage('Failed to restart Nova Language Server');
        }
    });

    const showLanguageServerOutputCommand = vscode.commands.registerCommand('nova.showLanguageServerOutput', () => {
        if (languageClient) {
            languageClient.outputChannel.show();
        } else {
            vscode.window.showWarningMessage('Nova Language Server is not running');
        }
    });

    const buildLanguageServerCommand = vscode.commands.registerCommand('nova.buildLanguageServer', async () => {
        const terminal = vscode.window.createTerminal('Nova Language Server Build');
        terminal.show();
        terminal.sendText('cd server-rust && cargo build --release');
        vscode.window.showInformationMessage('Building Nova Language Server...');
    });

    const runFileCommand = vscode.commands.registerCommand('nova.runFile', async (uri) => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor || activeEditor.document.languageId !== 'nova') {
            vscode.window.showErrorMessage('Please open a Nova file first');
            return;
        }

        const filePath = uri ? uri.fsPath : activeEditor.document.uri.fsPath;
        const workspaceFolder = vscode.workspace.getWorkspaceFolder(vscode.Uri.file(filePath));
        
        if (!workspaceFolder) {
            vscode.window.showErrorMessage('Please open a workspace to run Nova files');
            return;
        }

        // Save the file first
        if (activeEditor.document.isDirty) {
            await activeEditor.document.save();
        }

        const terminal = vscode.window.createTerminal('Nova Runtime');
        terminal.show();
        terminal.sendText(`cd "${workspaceFolder.uri.fsPath}"`);
        terminal.sendText(`./target/release/nova "${filePath}"`);
    });

    // Register debug support
    registerDebugSupport(context);
    
    // Register syntax validation
    registerSyntaxValidation(context);
    
    // Register formatting providers
    registerFormatting(context);
    
    // Register refactoring providers
    registerRefactoring(context);
    
    // Register symbol providers
    registerSymbolProviders(context);
    
    // Register project scaffolding
    registerProjectScaffolding(context);

    // Add providers and commands to subscriptions
    context.subscriptions.push(completionProvider);
    context.subscriptions.push(hoverProvider);
    context.subscriptions.push(definitionProvider);
    context.subscriptions.push(documentChangeListener);
    context.subscriptions.push(documentOpenListener);
    context.subscriptions.push(restartLanguageServerCommand);
    context.subscriptions.push(showLanguageServerOutputCommand);
    context.subscriptions.push(buildLanguageServerCommand);
    context.subscriptions.push(runFileCommand);

    console.log('🎉 Nova Language Support: All providers, commands, and debug support registered successfully!');
    console.log('📋 Total subscriptions:', context.subscriptions.length);
    
    // Test if completion provider is working
    setTimeout(() => {
        console.log('🧪 Testing completion provider registration...');
        const testDoc = vscode.window.activeTextEditor?.document;
        if (testDoc && testDoc.languageId === 'nova') {
            console.log('✅ Nova document detected for testing');
        }
    }, 1000);
}

/**
 * Deactivation function for the extension
 */
async function deactivate() {
    console.log('Nova Language Support extension deactivated');
    await stopLanguageServer();
}

module.exports = {
    activate,
    deactivate
};