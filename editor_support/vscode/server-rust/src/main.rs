use std::collections::HashMap;
use std::path::PathBuf;

use dashmap::DashMap;
use ropey::Rope;
use serde_json::Value;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tracing::info;

// Import Nova compiler components
use nova_compiler::{lexer::Lexer, parser::Parser, ast::*, value::Value as NovaValue};

#[derive(Debug)]
pub struct Document {
    pub uri: Url,
    pub content: Rope,
    pub version: i32,
}

pub struct NovaLanguageServer {
    client: Client,
    documents: DashMap<Url, Document>,
    workspace_folders: RwLock<Vec<WorkspaceFolder>>,
}

impl NovaLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DashMap::new(),
            workspace_folders: RwLock::new(Vec::new()),
        }
    }

    // Analyze Nova code and return diagnostics
    async fn analyze_document(&self, uri: &Url) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        if let Some(document) = self.documents.get(uri) {
            let content = document.content.to_string();
            
            // Tokenize the code
            let mut lexer = Lexer::new(&content);
            let tokens = match lexer.tokenize() {
                Ok(tokens) => tokens,
                Err(error) => {
                    // Add lexer error as diagnostic
                    diagnostics.push(Diagnostic::new_simple(
                        Range::new(Position::new(0, 0), Position::new(0, 1)),
                        format!("Lexer error: {}", error)
                    ));
                    return diagnostics;
                }
            };

            // Parse the tokens
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(_ast) => {
                    // AST parsed successfully - no syntax errors
                    info!("Successfully parsed Nova file: {}", uri);
                }
                Err(parse_error) => {
                    // Add parse error as diagnostic
                    let diagnostic = Diagnostic {
                        range: Range::new(Position::new(0, 0), Position::new(0, 1)),
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: Some(NumberOrString::String("parse_error".to_string())),
                        source: Some("nova-lsp".to_string()),
                        message: format!("Parse error: {}", parse_error),
                        related_information: None,
                        tags: None,
                        code_description: None,
                        data: None,
                    };
                    diagnostics.push(diagnostic);
                }
            }
        }

        diagnostics
    }

    // Get completion items for Nova language
    fn get_completions(&self, position: Position, document: &Document) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        // Add Nova keywords
        let keywords = [
            "let", "const", "fn", "if", "else", "while", "for", "in", 
            "return", "break", "continue", "true", "false", "null",
            "and", "or", "not", "try", "catch", "finally", "throw",
            "import", "from", "export", "class", "extends", "super",
            "this", "static", "private", "public", "async", "await",
            "as", "new", "constructor"
        ];

        for keyword in keywords {
            items.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(format!("Nova keyword: {}", keyword)),
                documentation: None,
                deprecated: Some(false),
                preselect: Some(false),
                sort_text: Some(format!("1_{}", keyword)),
                filter_text: None,
                insert_text: None,
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                insert_text_mode: None,
                text_edit: None,
                additional_text_edits: None,
                command: None,
                commit_characters: None,
                data: None,
                tags: None,
                label_details: None,
            });
        }

        // Add built-in functions
        let builtins = [
            ("print", "print(value) -> void", "Print a value to console"),
            ("len", "len(collection) -> number", "Get length of string/array"),
            ("type", "type(value) -> string", "Get type name of value"),
            ("str", "str(value) -> string", "Convert value to string"),
            ("num", "num(value) -> number", "Convert value to number"),
            ("push", "push(array, item) -> array", "Add item to array"),
            ("pop", "pop(array) -> any", "Remove last item from array"),
        ];

        for (name, signature, description) in builtins {
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(signature.to_string()),
                documentation: Some(Documentation::String(description.to_string())),
                deprecated: Some(false),
                preselect: Some(false),
                sort_text: Some(format!("2_{}", name)),
                filter_text: None,
                insert_text: None,
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                insert_text_mode: None,
                text_edit: None,
                additional_text_edits: None,
                command: None,
                commit_characters: None,
                data: None,
                tags: None,
                label_details: None,
            });
        }

        items
    }

    // Get hover information
    fn get_hover(&self, position: Position, document: &Document) -> Option<Hover> {
        // Get word at position
        let line = document.content.line(position.line as usize);
        let line_str = line.to_string();
        
        // Simple word extraction (could be improved)
        let chars: Vec<char> = line_str.chars().collect();
        let col = position.character as usize;
        
        if col >= chars.len() {
            return None;
        }

        // Find word boundaries
        let mut start = col;
        let mut end = col;
        
        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
            start -= 1;
        }
        
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }
        
        let word: String = chars[start..end].iter().collect();
        
        // Provide hover information for known symbols
        let hover_info = match word.as_str() {
            "print" => Some("**print(value)** - Print a value to the console\n\n```nova\nprint(\"Hello World!\");\n```"),
            "len" => Some("**len(collection)** - Get the length of a string or array\n\n```nova\nlen(\"hello\"); // 5\nlen([1, 2, 3]); // 3\n```"),
            "type" => Some("**type(value)** - Get the type name of a value\n\n```nova\ntype(42); // \"number\"\ntype(\"hello\"); // \"string\"\n```"),
            "class" => Some("**class** - Define a new class\n\n```nova\nclass MyClass {\n    constructor(param) {\n        this.param = param;\n    }\n}\n```"),
            "fn" => Some("**fn** - Define a function\n\n```nova\nfn myFunction(param) {\n    return param * 2;\n}\n```"),
            _ => None,
        };

        hover_info.map(|info| Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: info.to_string(),
            }),
            range: Some(Range::new(
                Position::new(position.line, start as u32),
                Position::new(position.line, end as u32),
            )),
        })
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for NovaLanguageServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        info!("Initializing Nova Language Server");

        // Store workspace folders
        if let Some(folders) = params.workspace_folders {
            *self.workspace_folders.write().await = folders;
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), " ".to_string()]),
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("nova-lsp".to_string()),
                        inter_file_dependencies: true,
                        workspace_diagnostics: false,
                        work_done_progress_options: WorkDoneProgressOptions::default(),
                    },
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "Nova Language Server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("Nova Language Server initialized");
        
        self.client
            .log_message(MessageType::INFO, "Nova Language Server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let document = Document {
            uri: params.text_document.uri.clone(),
            content: Rope::from_str(&params.text_document.text),
            version: params.text_document.version,
        };

        self.documents.insert(params.text_document.uri.clone(), document);

        // Analyze document and send diagnostics
        let diagnostics = self.analyze_document(&params.text_document.uri).await;
        
        self.client
            .publish_diagnostics(params.text_document.uri.clone(), diagnostics, Some(params.text_document.version))
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(mut document) = self.documents.get_mut(&params.text_document.uri) {
            // Update document content
            for change in params.content_changes {
                if let Some(range) = change.range {
                    let start_idx = document.content.line_to_char(range.start.line as usize) 
                        + range.start.character as usize;
                    let end_idx = document.content.line_to_char(range.end.line as usize) 
                        + range.end.character as usize;
                    document.content.remove(start_idx..end_idx);
                    document.content.insert(start_idx, &change.text);
                } else {
                    // Full document update
                    document.content = Rope::from_str(&change.text);
                }
            }
            document.version = params.text_document.version;
        }

        // Re-analyze document and send updated diagnostics
        let diagnostics = self.analyze_document(&params.text_document.uri).await;
        
        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, Some(params.text_document.version))
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.remove(&params.text_document.uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        if let Some(document) = self.documents.get(uri) {
            let items = self.get_completions(position, &document);
            Ok(Some(CompletionResponse::Array(items)))
        } else {
            Ok(None)
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(document) = self.documents.get(uri) {
            Ok(self.get_hover(position, &document))
        } else {
            Ok(None)
        }
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        // Basic implementation - could be expanded with actual symbol tracking
        Ok(None)
    }

    async fn diagnostic(&self, params: DocumentDiagnosticParams) -> Result<DocumentDiagnosticReportResult> {
        let diagnostics = self.analyze_document(&params.text_document.uri).await;
        
        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: diagnostics,
                },
            }),
        ))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| NovaLanguageServer::new(client));
    
    info!("Starting Nova Language Server");
    Server::new(stdin, stdout, socket).serve(service).await;
}