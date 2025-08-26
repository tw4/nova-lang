use crate::token::Token;
use crate::ast::SourceLocation;
use std::collections::VecDeque;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    line: usize,
    column: usize,
    file: Option<String>,
    interpolation_stack: VecDeque<InterpolationContext>,
}

#[derive(Debug, Clone)]
struct InterpolationContext {
    brace_count: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self::new_with_file(input, None)
    }

    pub fn new_with_file(input: &str, file: Option<String>) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = if chars.is_empty() { None } else { Some(chars[0]) };
        
        Lexer {
            input: chars,
            position: 0,
            current_char,
            line: 1,
            column: 1,
            file,
            interpolation_stack: VecDeque::new(),
        }
    }

    fn advance(&mut self) {
        if let Some('\n') = self.current_char {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position]);
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            None
        } else {
            Some(self.input[self.position + 1])
        }
    }

    fn peek_ahead(&self, offset: usize) -> Option<char> {
        if self.position + offset >= self.input.len() {
            None
        } else {
            Some(self.input[self.position + offset])
        }
    }

    fn current_location(&self) -> SourceLocation {
        SourceLocation::new(self.line, self.column, self.file.clone())
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        // Skip "//"
        self.advance();
        self.advance();
        
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) -> Result<(), String> {
        // Skip "/*"
        self.advance();
        self.advance();
        
        let mut depth = 1;
        
        while depth > 0 {
            match self.current_char {
                None => return Err("Unterminated block comment".to_string()),
                Some('/') if self.peek() == Some('*') => {
                    depth += 1;
                    self.advance();
                    self.advance();
                }
                Some('*') if self.peek() == Some('/') => {
                    depth -= 1;
                    self.advance();
                    self.advance();
                }
                Some(_) => self.advance(),
            }
        }
        
        Ok(())
    }

    fn read_number(&mut self) -> Result<f64, String> {
        let mut number = String::new();
        let mut has_dot = false;
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot && 
                      self.peek().map_or(false, |c| c.is_ascii_digit()) {
                has_dot = true;
                number.push(ch);
                self.advance();
            } else if ch == '_' {
                // Allow underscores in numbers for readability
                self.advance();
            } else {
                break;
            }
        }
        
        number.parse().map_err(|_| format!("Invalid number: {}", number))
    }

    fn read_string(&mut self, quote: char) -> Result<String, String> {
        let mut string = String::new();
        self.advance(); // Skip opening quote
        
        while let Some(ch) = self.current_char {
            if ch == quote {
                self.advance(); // Skip closing quote
                return Ok(string);
            } else if ch == '\\' {
                self.advance();
                match self.current_char {
                    Some('n') => string.push('\n'),
                    Some('t') => string.push('\t'),
                    Some('r') => string.push('\r'),
                    Some('\\') => string.push('\\'),
                    Some('"') => string.push('"'),
                    Some('\'') => string.push('\''),
                    Some('0') => string.push('\0'),
                    Some('x') => {
                        // Hexadecimal escape sequence
                        self.advance();
                        let hex = self.read_hex_escape(2)?;
                        if let Some(ch) = char::from_u32(hex) {
                            string.push(ch);
                        } else {
                            return Err(format!("Invalid hex escape: \\x{:02x}", hex));
                        }
                        continue;
                    }
                    Some('u') => {
                        // Unicode escape sequence
                        self.advance();
                        let unicode = self.read_hex_escape(4)?;
                        if let Some(ch) = char::from_u32(unicode) {
                            string.push(ch);
                        } else {
                            return Err(format!("Invalid unicode escape: \\u{:04x}", unicode));
                        }
                        continue;
                    }
                    Some(c) => string.push(c),
                    None => return Err("Unterminated string literal".to_string()),
                }
                self.advance();
            } else if ch == '\n' {
                return Err("Unterminated string literal".to_string());
            } else {
                string.push(ch);
                self.advance();
            }
        }
        
        Err("Unterminated string literal".to_string())
    }

    fn read_hex_escape(&mut self, digits: usize) -> Result<u32, String> {
        let mut value = 0;
        for _ in 0..digits {
            match self.current_char {
                Some(c) if c.is_ascii_hexdigit() => {
                    value = value * 16 + c.to_digit(16).unwrap();
                    self.advance();
                }
                _ => return Err("Invalid hex escape sequence".to_string()),
            }
        }
        Ok(value)
    }

    fn read_interpolated_string(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut current_string = String::new();
        
        // Skip 'f"'
        self.advance();
        self.advance();
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                // End of interpolated string
                if !current_string.is_empty() {
                    tokens.push(Token::StringEnd(current_string.clone()));
                }
                self.advance();
                break;
            } else if ch == '$' && self.peek() == Some('{') {
                // Start of interpolation
                if !current_string.is_empty() {
                    tokens.push(Token::StringMiddle(current_string.clone()));
                    current_string.clear();
                }
                
                tokens.push(Token::InterpolationStart);
                self.advance(); // Skip '$'
                self.advance(); // Skip '{'
                
                // Parse the expression inside ${}
                let mut brace_count = 1;
                let mut expr_tokens = Vec::new();
                
                while brace_count > 0 && self.current_char.is_some() {
                    let token = self.next_token()?;
                    match &token {
                        Token::LeftBrace => brace_count += 1,
                        Token::RightBrace => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                tokens.push(Token::InterpolationEnd);
                                continue;
                            }
                        }
                        _ => {}
                    }
                    expr_tokens.push(token);
                }
                
                tokens.extend(expr_tokens);
            } else if ch == '\\' {
                // Escape sequences
                self.advance();
                match self.current_char {
                    Some('n') => current_string.push('\n'),
                    Some('t') => current_string.push('\t'),
                    Some('r') => current_string.push('\r'),
                    Some('\\') => current_string.push('\\'),
                    Some('"') => current_string.push('"'),
                    Some('$') => current_string.push('$'),
                    Some(c) => {
                        current_string.push('\\');
                        current_string.push(c);
                    }
                    None => return Err("Unterminated string literal".to_string()),
                }
                self.advance();
            } else {
                current_string.push(ch);
                self.advance();
            }
        }
        
        if tokens.is_empty() {
            tokens.push(Token::StringEnd(current_string));
        }
        
        Ok(tokens)
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        identifier
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        loop {
            match self.current_char {
                None => return Ok(Token::Eof),
                Some(' ') | Some('\t') | Some('\r') => {
                    self.skip_whitespace();
                }
                Some('\n') => {
                    self.advance();
                    return Ok(Token::Newline);
                }
                Some('/') => {
                    match self.peek() {
                        Some('/') => {
                            self.skip_line_comment();
                            continue;
                        }
                        Some('*') => {
                            self.skip_block_comment()?;
                            continue;
                        }
                        Some('=') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::SlashEqual);
                        }
                        _ => {
                            self.advance();
                            return Ok(Token::Slash);
                        }
                    }
                }
                Some('+') => {
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::PlusEqual);
                        }
                        _ => {
                            self.advance();
                            return Ok(Token::Plus);
                        }
                    }
                }
                Some('-') => {
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::MinusEqual);
                        }
                        Some('>') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::Arrow);
                        }
                        _ => {
                            self.advance();
                            return Ok(Token::Minus);
                        }
                    }
                }
                Some('*') => {
                    match self.peek() {
                        Some('*') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::DoubleStar);
                        }
                        Some('=') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::StarEqual);
                        }
                        _ => {
                            self.advance();
                            return Ok(Token::Star);
                        }
                    }
                }
                Some('%') => {
                    self.advance();
                    return Ok(Token::Percent);
                }
                Some('&') => {
                    self.advance();
                    return Ok(Token::Ampersand);
                }
                Some('|') => {
                    self.advance();
                    return Ok(Token::Pipe);
                }
                Some('^') => {
                    self.advance();
                    return Ok(Token::Caret);
                }
                Some('~') => {
                    self.advance();
                    return Ok(Token::Tilde);
                }
                Some('<') => {
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::LessEqual);
                        }
                        Some('<') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::LeftShift);
                        }
                        _ => {
                            self.advance();
                            return Ok(Token::Less);
                        }
                    }
                }
                Some('>') => {
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::GreaterEqual);
                        }
                        Some('>') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::RightShift);
                        }
                        _ => {
                            self.advance();
                            return Ok(Token::Greater);
                        }
                    }
                }
                Some('=') => {
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::EqualEqual);
                        }
                        Some('>') => {
                            self.advance();
                            self.advance();
                            return Ok(Token::DoubleArrow);
                        }
                        _ => {
                            self.advance();
                            return Ok(Token::Equal);
                        }
                    }
                }
                Some('!') => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.advance();
                        return Ok(Token::BangEqual);
                    } else {
                        self.advance();
                        return Ok(Token::Bang);
                    }
                }
                Some('?') => {
                    if self.peek() == Some('?') {
                        self.advance();
                        self.advance();
                        return Ok(Token::DoubleQuestion);
                    } else {
                        self.advance();
                        return Ok(Token::Question);
                    }
                }
                Some('(') => {
                    self.advance();
                    return Ok(Token::LeftParen);
                }
                Some(')') => {
                    self.advance();
                    return Ok(Token::RightParen);
                }
                Some('{') => {
                    self.advance();
                    return Ok(Token::LeftBrace);
                }
                Some('}') => {
                    self.advance();
                    return Ok(Token::RightBrace);
                }
                Some('[') => {
                    self.advance();
                    return Ok(Token::LeftBracket);
                }
                Some(']') => {
                    self.advance();
                    return Ok(Token::RightBracket);
                }
                Some(',') => {
                    self.advance();
                    return Ok(Token::Comma);
                }
                Some(';') => {
                    self.advance();
                    return Ok(Token::Semicolon);
                }
                Some(':') => {
                    self.advance();
                    return Ok(Token::Colon);
                }
                Some('.') => {
                    self.advance();
                    return Ok(Token::Dot);
                }
                Some('"') => {
                    let string = self.read_string('"')?;
                    return Ok(Token::String(string));
                }
                Some('\'') => {
                    let string = self.read_string('\'')?;
                    return Ok(Token::String(string));
                }
                Some('f') if self.peek() == Some('"') => {
                    // Interpolated string
                    let tokens = self.read_interpolated_string()?;
                    // For now, return the first token and handle the rest later
                    if let Some(token) = tokens.first() {
                        return Ok(token.clone());
                    } else {
                        return Ok(Token::String(String::new()));
                    }
                }
                Some(ch) if ch.is_ascii_digit() => {
                    let number = self.read_number()?;
                    return Ok(Token::Number(number));
                }
                Some(ch) if ch.is_alphabetic() || ch == '_' => {
                    let identifier = self.read_identifier();
                    return Ok(Token::is_keyword(&identifier)
                        .unwrap_or_else(|| Token::Identifier(identifier)));
                }
                Some(ch) => {
                    return Err(format!("Unexpected character: '{}'", ch));
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            // Skip newlines in tokenize mode for simplicity
            if !matches!(token, Token::Newline) {
                tokens.push(token);
            }
        }
        
        Ok(tokens)
    }
}