#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithLocation {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Identifier(String),
    Number(f64),
    String(String),
    
    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    DoubleStar,    // **
    
    // Comparison operators
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Bitwise operators
    Ampersand,     // &
    Pipe,          // |
    Caret,         // ^
    Tilde,         // ~
    LeftShift,     // <<
    RightShift,    // >>
    
    // Assignment operators
    PlusEqual,     // +=
    MinusEqual,    // -=
    StarEqual,     // *=
    SlashEqual,    // /=
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Arrow,         // ->
    DoubleArrow,   // =>
    Question,      // ?
    DoubleQuestion, // ??
    
    // Keywords
    Let,
    Const,
    Fn,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    True,
    False,
    Null,
    And,
    Or,
    Not,
    Try,
    Catch,
    Finally,
    Throw,
    Import,
    From,
    Export,
    Class,
    Extends,
    Super,
    This,
    Static,
    Private,
    Public,
    Async,
    Await,
    As,
    New,
    Constructor,
    
    // String interpolation
    StringStart,       // f"
    StringMiddle(String), // middle part of interpolated string
    StringEnd(String),    // end part of interpolated string
    InterpolationStart,   // ${
    InterpolationEnd,     // }
    
    // Special tokens
    Newline,
    Eof,
    Error(String),
}

impl Token {
    pub fn is_keyword(identifier: &str) -> Option<Token> {
        match identifier {
            "let" => Some(Token::Let),
            "const" => Some(Token::Const),
            "fn" => Some(Token::Fn),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "while" => Some(Token::While),
            "for" => Some(Token::For),
            "in" => Some(Token::In),
            "return" => Some(Token::Return),
            "break" => Some(Token::Break),
            "continue" => Some(Token::Continue),
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            "null" => Some(Token::Null),
            "and" => Some(Token::And),
            "or" => Some(Token::Or),
            "not" => Some(Token::Not),
            "try" => Some(Token::Try),
            "catch" => Some(Token::Catch),
            "finally" => Some(Token::Finally),
            "throw" => Some(Token::Throw),
            "import" => Some(Token::Import),
            "from" => Some(Token::From),
            "export" => Some(Token::Export),
            "class" => Some(Token::Class),
            "extends" => Some(Token::Extends),
            "super" => Some(Token::Super),
            "this" => Some(Token::This),
            "static" => Some(Token::Static),
            "private" => Some(Token::Private),
            "public" => Some(Token::Public),
            "async" => Some(Token::Async),
            "await" => Some(Token::Await),
            "as" => Some(Token::As),
            "new" => Some(Token::New),
            "constructor" => Some(Token::Constructor),
            _ => None,
        }
    }

    pub fn is_assignment_operator(&self) -> bool {
        matches!(self, 
            Token::Equal | 
            Token::PlusEqual | 
            Token::MinusEqual | 
            Token::StarEqual | 
            Token::SlashEqual
        )
    }

    pub fn is_binary_operator(&self) -> bool {
        matches!(self,
            Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Percent |
            Token::DoubleStar | Token::EqualEqual | Token::BangEqual | Token::Less |
            Token::Greater | Token::LessEqual | Token::GreaterEqual | Token::And |
            Token::Or | Token::Ampersand | Token::Pipe | Token::Caret |
            Token::LeftShift | Token::RightShift
        )
    }

    pub fn is_unary_operator(&self) -> bool {
        matches!(self, Token::Bang | Token::Minus | Token::Plus | Token::Tilde)
    }

    pub fn precedence(&self) -> Option<u8> {
        match self {
            Token::Or => Some(1),
            Token::And => Some(2),
            Token::Pipe => Some(3),
            Token::Caret => Some(4),
            Token::Ampersand => Some(5),
            Token::EqualEqual | Token::BangEqual => Some(6),
            Token::Less | Token::Greater | Token::LessEqual | Token::GreaterEqual => Some(7),
            Token::LeftShift | Token::RightShift => Some(8),
            Token::Plus | Token::Minus => Some(9),
            Token::Star | Token::Slash | Token::Percent => Some(10),
            Token::DoubleStar => Some(11),
            _ => None,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Identifier(name) => write!(f, "identifier '{}'", name),
            Token::Number(n) => write!(f, "number '{}'", n),
            Token::String(s) => write!(f, "string \"{}\"", s),
            Token::Plus => write!(f, "'+'"),
            Token::Minus => write!(f, "'-'"),
            Token::Star => write!(f, "'*'"),
            Token::Slash => write!(f, "'/'"),
            Token::Percent => write!(f, "'%'"),
            Token::DoubleStar => write!(f, "'**'"),
            Token::Equal => write!(f, "'='"),
            Token::EqualEqual => write!(f, "'=='"),
            Token::Bang => write!(f, "'!'"),
            Token::BangEqual => write!(f, "'!='"),
            Token::Less => write!(f, "'<'"),
            Token::Greater => write!(f, "'>'"),
            Token::LessEqual => write!(f, "'<='"),
            Token::GreaterEqual => write!(f, "'>='"),
            Token::And => write!(f, "'and'"),
            Token::Or => write!(f, "'or'"),
            Token::LeftParen => write!(f, "'('"),
            Token::RightParen => write!(f, "')'"),
            Token::LeftBrace => write!(f, "'{{'"),
            Token::RightBrace => write!(f, "'}}'"),
            Token::LeftBracket => write!(f, "'['"),
            Token::RightBracket => write!(f, "']'"),
            Token::Comma => write!(f, "','"),
            Token::Semicolon => write!(f, "';'"),
            Token::Colon => write!(f, "':'"),
            Token::Dot => write!(f, "'.'"),
            Token::Arrow => write!(f, "'->'"),
            Token::Let => write!(f, "'let'"),
            Token::Fn => write!(f, "'fn'"),
            Token::If => write!(f, "'if'"),
            Token::Else => write!(f, "'else'"),
            Token::While => write!(f, "'while'"),
            Token::For => write!(f, "'for'"),
            Token::In => write!(f, "'in'"),
            Token::Return => write!(f, "'return'"),
            Token::True => write!(f, "'true'"),
            Token::False => write!(f, "'false'"),
            Token::Null => write!(f, "'null'"),
            Token::Try => write!(f, "'try'"),
            Token::Catch => write!(f, "'catch'"),
            Token::Throw => write!(f, "'throw'"),
            Token::Import => write!(f, "'import'"),
            Token::Newline => write!(f, "newline"),
            Token::Eof => write!(f, "end of file"),
            Token::Error(msg) => write!(f, "error: {}", msg),
            _ => write!(f, "{:?}", self),
        }
    }
}