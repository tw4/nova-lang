#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    New {
        class: Box<Expr>,
        args: Vec<Expr>,
    },
    Block(Vec<Stmt>),
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    Array(Vec<Expr>),
    Object(Vec<(String, Expr)>),
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Property {
        object: Box<Expr>,
        property: String,
    },
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    For {
        variable: String,
        iterable: Box<Expr>,
        body: Box<Expr>,
    },
    Assignment {
        target: Box<Expr>,
        value: Box<Expr>,
    },
    StringInterpolation(Vec<Expr>),
    Try {
        body: Box<Expr>,
        catch: Option<(String, Box<Expr>)>,
        finally: Option<Box<Expr>>,
    },
    Throw(Box<Expr>),
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    This,
    Super,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    Let {
        name: String,
        value: Expr,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Expr,
    },
    Return(Option<Expr>),
    Import {
        module: String,
        alias: Option<String>,
    },
    Class {
        name: String,
        superclass: Option<String>,
        methods: Vec<ClassMethod>,
        constructor: Option<ClassMethod>,
    },
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Literal>),
    Object(Vec<(String, Literal)>),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Minus,
    Plus,
    BitwiseNot,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentTarget {
    Identifier(String),
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Property {
        object: Box<Expr>,
        property: String,
    },
}

// Error types for better error handling
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AnnotatedExpr {
    pub expr: Expr,
    pub location: SourceLocation,
}

#[derive(Debug, Clone)]
pub struct AnnotatedStmt {
    pub stmt: Stmt,
    pub location: SourceLocation,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize, file: Option<String>) -> Self {
        Self { line, column, file }
    }

    pub fn unknown() -> Self {
        Self {
            line: 0,
            column: 0,
            file: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassMethod {
    pub name: String,
    pub params: Vec<String>,
    pub body: Expr,
    pub visibility: Visibility,
    pub is_static: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.file {
            Some(file) => write!(f, "{}:{}:{}", file, self.line, self.column),
            None => write!(f, "{}:{}", self.line, self.column),
        }
    }
}