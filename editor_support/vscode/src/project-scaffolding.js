const vscode = require('vscode');
const fs = require('fs');
const path = require('path');

/**
 * Nova Project Scaffolding Provider
 * Creates new Nova projects from templates
 */
class NovaProjectScaffolder {
    constructor(extensionPath) {
        this.extensionPath = extensionPath;
        this.templatesPath = path.join(extensionPath, 'templates');
        this.availableTemplates = [
            {
                id: 'hello-world',
                name: 'Hello World',
                description: 'A simple "Hello, World!" program demonstrating basic Nova syntax',
                files: ['main.nova', 'README.md']
            },
            {
                id: 'web-app',
                name: 'Web Application',
                description: 'A simple web server with routing and object-oriented design',
                files: ['main.nova', 'README.md']
            },
            {
                id: 'cli-app',
                name: 'CLI Application',
                description: 'A command-line task manager with advanced features',
                files: ['main.nova', 'README.md']
            }
        ];
    }

    /**
     * Show template selection and create project
     */
    async createProject() {
        // Get workspace folder
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showErrorMessage('Please open a workspace folder first');
            return;
        }

        // Select template
        const templateItems = this.availableTemplates.map(template => ({
            label: template.name,
            description: template.description,
            detail: `Files: ${template.files.join(', ')}`,
            template: template
        }));

        const selectedTemplate = await vscode.window.showQuickPick(templateItems, {
            placeHolder: 'Select a Nova project template',
            matchOnDescription: true,
            matchOnDetail: true
        });

        if (!selectedTemplate) return;

        // Get project name
        const projectName = await vscode.window.showInputBox({
            prompt: 'Enter project name',
            placeHolder: 'my-nova-project',
            validateInput: (value) => {
                if (!value) return 'Project name is required';
                if (!/^[a-zA-Z0-9_-]+$/.test(value)) {
                    return 'Project name can only contain letters, numbers, hyphens, and underscores';
                }
                return null;
            }
        });

        if (!projectName) return;

        // Create project directory
        const workspaceRoot = workspaceFolders[0].uri.fsPath;
        const projectPath = path.join(workspaceRoot, projectName);

        try {
            await this.createProjectFromTemplate(selectedTemplate.template, projectPath, projectName);
            vscode.window.showInformationMessage(`Created Nova project: ${projectName}`);
            
            // Open main file
            const mainFilePath = path.join(projectPath, 'main.nova');
            if (fs.existsSync(mainFilePath)) {
                const document = await vscode.workspace.openTextDocument(mainFilePath);
                await vscode.window.showTextDocument(document);
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to create project: ${error.message}`);
        }
    }

    /**
     * Create project from template
     */
    async createProjectFromTemplate(template, projectPath, projectName) {
        // Create project directory
        if (fs.existsSync(projectPath)) {
            throw new Error(`Directory ${projectName} already exists`);
        }

        fs.mkdirSync(projectPath, { recursive: true });

        // Copy template files
        const templatePath = path.join(this.templatesPath, template.id);
        
        for (const fileName of template.files) {
            const sourcePath = path.join(templatePath, fileName);
            const destPath = path.join(projectPath, fileName);

            if (fs.existsSync(sourcePath)) {
                let content = fs.readFileSync(sourcePath, 'utf8');
                
                // Replace template variables
                content = this.replaceTemplateVariables(content, {
                    projectName,
                    className: this.toPascalCase(projectName),
                    date: new Date().toISOString().split('T')[0]
                });

                fs.writeFileSync(destPath, content);
            }
        }

        // Create additional project structure
        await this.createProjectStructure(projectPath, template);
    }

    /**
     * Create additional project structure
     */
    async createProjectStructure(projectPath, template) {
        // Create common directories
        const directories = ['src', 'tests', 'docs'];
        
        directories.forEach(dir => {
            const dirPath = path.join(projectPath, dir);
            if (!fs.existsSync(dirPath)) {
                fs.mkdirSync(dirPath, { recursive: true });
            }
        });

        // Create .gitignore
        const gitignoreContent = `# Nova build artifacts
target/
*.o
*.so
*.dylib
*.exe

# Editor files
.vscode/
*.swp
*.swo
*~

# OS files
.DS_Store
Thumbs.db

# Logs
*.log

# Dependencies
node_modules/
`;
        fs.writeFileSync(path.join(projectPath, '.gitignore'), gitignoreContent);

        // Create project configuration
        const projectConfig = {
            name: path.basename(projectPath),
            version: "0.1.0",
            description: template.description,
            main: "main.nova",
            scripts: {
                build: "nova build",
                run: "nova main.nova",
                test: "nova test"
            },
            keywords: ["nova", template.id.replace('-', ' ')],
            author: "",
            license: "MIT"
        };

        fs.writeFileSync(
            path.join(projectPath, 'nova.json'),
            JSON.stringify(projectConfig, null, 2)
        );
    }

    /**
     * Replace template variables in content
     */
    replaceTemplateVariables(content, variables) {
        let result = content;
        
        for (const [key, value] of Object.entries(variables)) {
            const regex = new RegExp(`\\{\\{${key}\\}\\}`, 'g');
            result = result.replace(regex, value);
        }

        return result;
    }

    /**
     * Convert string to PascalCase
     */
    toPascalCase(str) {
        return str
            .split(/[-_\s]+/)
            .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
            .join('');
    }

    /**
     * Generate file from template
     */
    async generateFile() {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('Please open a Nova file first');
            return;
        }

        const fileTemplates = [
            {
                label: 'Nova Class',
                description: 'Generate a new class with constructor and methods',
                template: 'class'
            },
            {
                label: 'Nova Function',
                description: 'Generate a function with parameters and documentation',
                template: 'function'
            },
            {
                label: 'Nova Interface',
                description: 'Generate an interface definition',
                template: 'interface'
            },
            {
                label: 'Nova Enum',
                description: 'Generate an enum with variants',
                template: 'enum'
            },
            {
                label: 'Test Function',
                description: 'Generate a test function',
                template: 'test'
            }
        ];

        const selected = await vscode.window.showQuickPick(fileTemplates, {
            placeHolder: 'Select a code template to generate'
        });

        if (!selected) return;

        const name = await vscode.window.showInputBox({
            prompt: `Enter ${selected.template} name`,
            placeHolder: selected.template === 'class' ? 'MyClass' : 'myFunction',
            validateInput: (value) => {
                if (!value) return `${selected.template} name is required`;
                if (selected.template === 'class' && !/^[A-Z][a-zA-Z0-9]*$/.test(value)) {
                    return 'Class names must start with uppercase letter';
                }
                if (selected.template !== 'class' && !/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(value)) {
                    return 'Invalid name format';
                }
                return null;
            }
        });

        if (!name) return;

        const code = this.generateCodeTemplate(selected.template, name);
        
        // Insert at cursor position
        const position = editor.selection.active;
        await editor.edit(editBuilder => {
            editBuilder.insert(position, code);
        });
    }

    /**
     * Generate code template
     */
    generateCodeTemplate(type, name) {
        switch (type) {
            case 'class':
                return `class ${name} {
    constructor() {
        // Initialize properties
    }
    
    // Add methods here
    
}`;

            case 'function':
                return `fn ${name}() {
    // Function implementation
    
}`;

            case 'interface':
                return `interface ${name} {
    // Define method signatures
    
}`;

            case 'enum':
                return `enum ${name} {
    // Define enum variants
    
}`;

            case 'test':
                return `fn test${name.charAt(0).toUpperCase() + name.slice(1)}() {
    // Test implementation
    assert(true, "Test should pass");
}`;

            default:
                return '';
        }
    }
}

/**
 * Register project scaffolding commands
 */
function registerProjectScaffolding(context) {
    const scaffolder = new NovaProjectScaffolder(context.extensionPath);

    const createProjectCommand = vscode.commands.registerCommand('nova.createProject', async () => {
        await scaffolder.createProject();
    });

    const generateFileCommand = vscode.commands.registerCommand('nova.generateFile', async () => {
        await scaffolder.generateFile();
    });

    context.subscriptions.push(createProjectCommand, generateFileCommand);

    return scaffolder;
}

module.exports = {
    NovaProjectScaffolder,
    registerProjectScaffolding
};