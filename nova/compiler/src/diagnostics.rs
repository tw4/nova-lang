use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column, file: None }
    }

    pub fn with_file(line: usize, column: usize, file: String) -> Self {
        Self { line, column, file: Some(file) }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(file) = &self.file {
            write!(f, "{}:{}:{}", file, self.line, self.column)
        } else {
            write!(f, "{}:{}", self.line, self.column)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
    Hint,
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticLevel::Error => write!(f, "error"),
            DiagnosticLevel::Warning => write!(f, "warning"),
            DiagnosticLevel::Info => write!(f, "info"),
            DiagnosticLevel::Hint => write!(f, "hint"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub location: SourceLocation,
    pub code: Option<String>,
    pub help: Option<String>,
    pub related: Vec<(String, SourceLocation)>,
}

impl Diagnostic {
    pub fn error(message: String, location: SourceLocation) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message,
            location,
            code: None,
            help: None,
            related: Vec::new(),
        }
    }

    pub fn warning(message: String, location: SourceLocation) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            message,
            location,
            code: None,
            help: None,
            related: Vec::new(),
        }
    }

    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }

    pub fn with_related(mut self, message: String, location: SourceLocation) -> Self {
        self.related.push((message, location));
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format: level[code]: message
        //   --> location
        //   |
        // 1 | source line here
        //   |     ^^^^^ help text

        write!(f, "{}", self.level)?;
        
        if let Some(code) = &self.code {
            write!(f, "[{}]", code)?;
        }
        
        writeln!(f, ": {}", self.message)?;
        writeln!(f, "  --> {}", self.location)?;

        if let Some(help) = &self.help {
            writeln!(f, "  help: {}", help)?;
        }

        for (related_msg, related_loc) in &self.related {
            writeln!(f, "  note: {} (at {})", related_msg, related_loc)?;
        }

        Ok(())
    }
}

pub struct DiagnosticsEngine {
    diagnostics: Vec<Diagnostic>,
    error_count: usize,
    warning_count: usize,
}

impl DiagnosticsEngine {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            error_count: 0,
            warning_count: 0,
        }
    }

    pub fn report(&mut self, diagnostic: Diagnostic) {
        match diagnostic.level {
            DiagnosticLevel::Error => self.error_count += 1,
            DiagnosticLevel::Warning => self.warning_count += 1,
            _ => {}
        }
        self.diagnostics.push(diagnostic);
    }

    pub fn error(&mut self, message: String, location: SourceLocation) {
        self.report(Diagnostic::error(message, location));
    }

    pub fn warning(&mut self, message: String, location: SourceLocation) {
        self.report(Diagnostic::warning(message, location));
    }

    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    pub fn error_count(&self) -> usize {
        self.error_count
    }

    pub fn warning_count(&self) -> usize {
        self.warning_count
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.error_count = 0;
        self.warning_count = 0;
    }

    pub fn print_all(&self) {
        for diagnostic in &self.diagnostics {
            eprintln!("{}", diagnostic);
        }
        
        if self.error_count > 0 || self.warning_count > 0 {
            eprintln!();
            eprintln!("Summary: {} errors, {} warnings", self.error_count, self.warning_count);
        }
    }
}

// Predefined error codes and messages
pub struct ErrorCodes;

impl ErrorCodes {
    pub const SYNTAX_ERROR: &'static str = "E0001";
    pub const UNDEFINED_VARIABLE: &'static str = "E0002";
    pub const TYPE_MISMATCH: &'static str = "E0003";
    pub const INVALID_OPERATION: &'static str = "E0004";
    pub const DIVISION_BY_ZERO: &'static str = "E0005";
    pub const INDEX_OUT_OF_BOUNDS: &'static str = "E0006";
    pub const UNDEFINED_FUNCTION: &'static str = "E0007";
    pub const ARGUMENT_MISMATCH: &'static str = "E0008";
    pub const UNDEFINED_CLASS: &'static str = "E0009";
    pub const INVALID_PROPERTY: &'static str = "E0010";
    
    // Warnings
    pub const UNUSED_VARIABLE: &'static str = "W0001";
    pub const UNREACHABLE_CODE: &'static str = "W0002";
    pub const DEPRECATED_FEATURE: &'static str = "W0003";
}

// Helper functions for common diagnostics
pub fn undefined_variable_error(name: &str, location: SourceLocation) -> Diagnostic {
    Diagnostic::error(
        format!("Undefined variable '{}'", name),
        location,
    )
    .with_code(ErrorCodes::UNDEFINED_VARIABLE.to_string())
    .with_help("Make sure the variable is declared before using it".to_string())
}

pub fn type_mismatch_error(expected: &str, found: &str, location: SourceLocation) -> Diagnostic {
    Diagnostic::error(
        format!("Type mismatch: expected '{}', found '{}'", expected, found),
        location,
    )
    .with_code(ErrorCodes::TYPE_MISMATCH.to_string())
}

pub fn unused_variable_warning(name: &str, location: SourceLocation) -> Diagnostic {
    Diagnostic::warning(
        format!("Unused variable '{}'", name),
        location,
    )
    .with_code(ErrorCodes::UNUSED_VARIABLE.to_string())
    .with_help("Consider removing this variable or prefixing it with '_'".to_string())
}