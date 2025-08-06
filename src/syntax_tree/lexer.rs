use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    NUMERAL(i64),
    // OPERATORS
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    EQUALS,
    COMPARISON,
    NOTEQUALS, //These are inspired by x86 conditional jump instructions names
    GREATEREQUALS, 
    LESSEQUALS,
    GREATER,
    LESS,
    // BITWISE
    AMPERSAND,
    PIPE,
    CARET,
    POWER,
    NOT,
    // KEYWORDS
    LET,
    IF,
    ELSE,
    TRUE,
    FALSE,
    WHILE,
    FUNC,
    RETURN,
    // OTHER
    LEFTPAR,
    RIGHTPAR,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    SEMICOLON,
    WHITESPACE,
    IDENTIFIER,
    EOF,
    INVALID,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::NUMERAL(_) => write!(f, "NUMERAL"),
            // OPERATORS
            TokenType::PLUS => write!(f, "+"),
            TokenType::MINUS => write!(f, "-"),
            TokenType::ASTERISK => write!(f, "*"),
            TokenType::SLASH => write!(f, "/"),
            TokenType::EQUALS => write!(f, "="),
            TokenType::COMPARISON => write!(f, "=="),
            TokenType::NOTEQUALS => write!(f, "!="),
            TokenType::GREATEREQUALS => write!(f, ">="),
            TokenType::LESSEQUALS => write!(f, "<="),  
            TokenType::GREATER => write!(f, ">"),
            TokenType::LESS => write!(f, "<"),
            // BITWISE
            TokenType::AMPERSAND => write!(f, "&"),
            TokenType::PIPE => write!(f, "|"),
            TokenType::CARET => write!(f, "^"),
            TokenType::POWER => write!(f, "**"),
            TokenType::NOT => write!(f, "~"),
            // KEYWORDS
            TokenType::LET => write!(f, "LET"),
            TokenType::IF => write!(f, "IF"),
            TokenType::ELSE => write!(f, "ELSE"),
            TokenType::TRUE => write!(f, "TRUE"),
            TokenType::FALSE =>write!(f, "FALSE"),
            TokenType::WHILE =>write!(f, "WHILE"),
            TokenType::FUNC => write!(f, "FUNC"),
            TokenType::RETURN => write!(f, "RETURN"),

            TokenType::LEFTPAR => write!(f, "("),
            TokenType::RIGHTPAR => write!(f, ")"),
            TokenType::LEFTBRACE => write!(f, "{{"),
            TokenType::RIGHTBRACE => write!(f, "}}"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::SEMICOLON => write!(f, "SEMICOLON"),
            TokenType::WHITESPACE => write!(f, "WHITESPACE"),
            TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::INVALID => write!(f, "INVALID"),

        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SourceCodeSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}

impl SourceCodeSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self { Self { start, end, literal } }

    pub fn length(&self) -> usize { return self.end - self.start; }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) kind: TokenType,
    pub(crate) span: SourceCodeSpan,
}

impl Token {
    pub fn new(kind: TokenType, span: SourceCodeSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, current_pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos == self.input.len() {
            let eof_indicator: char = '\0';
            self.current_pos += 1;
            return Some(Token::new(
                TokenType::EOF,
                SourceCodeSpan::new(0, 0, eof_indicator.to_string()),
            ));
        }
        let c = self.current_char();
        return c.map(|c| {
            let start = self.current_pos;
            let kind;
            if Lexer::is_number_start(&c) {
                let number: i64 = self.consume_number();
                kind = TokenType::NUMERAL(number);
            } 
            else if Lexer::is_whitespace(&c){
                self.consume_token();
                kind = TokenType::WHITESPACE;
            } 
            //For simplicity and testing purposes both "let" and "var" are lexed (converted) as "let"
            else if Lexer::is_identifier_start(&c){
                let identifier = self.consume_identifier();
                kind = match identifier.as_str() {
                    "let" | "var" => TokenType::LET,
                    "if" => TokenType::IF,
                    "else" => TokenType::ELSE,
                    "true" => TokenType::TRUE,
                    "false" => TokenType::FALSE,
                    "while" => TokenType::WHILE,
                    "func" => TokenType::FUNC,
                    "return" => TokenType::RETURN,
                    _ => TokenType::IDENTIFIER,
                }

            } 
            else { kind = self.consume_symbol(); } 

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = SourceCodeSpan::new(start, end, literal);
            return Token::new(kind, span);
        });
    }

    fn consume_symbol(&mut self) -> TokenType {
        let c = self.consume_token().unwrap();
        return match c {
            '+' => TokenType::PLUS,
            '-' => TokenType::MINUS,
            '*' => {
                self.check_if_double_operator('*', TokenType::ASTERISK, TokenType::POWER)
            },
            '/' => TokenType::SLASH,
            '(' => TokenType::LEFTPAR,
            ')' => TokenType::RIGHTPAR,
            '{' => TokenType::LEFTBRACE,
            '}' => TokenType::RIGHTBRACE,
            ',' => TokenType::COMMA,
            ';' => TokenType::SEMICOLON,
            '=' => {
                self.check_if_double_operator('=', TokenType::EQUALS, TokenType::COMPARISON)
            },
            '!' => {
                self.check_if_double_operator('=', TokenType::INVALID, TokenType::NOTEQUALS)
            },
            '>' => {
                self.check_if_double_operator('=', TokenType::GREATER, TokenType::GREATEREQUALS)
            },
            '<' => {
                self.check_if_double_operator('=', TokenType::LESS, TokenType::LESSEQUALS)
            },
            '&' => TokenType::AMPERSAND,
            '|' => TokenType::PIPE,
            '^' => TokenType::CARET,
            '~' => TokenType::NOT,
            _ => TokenType::INVALID,
        };
    }


    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if Self::is_identifier_start(&c) {
                self.consume_token().unwrap();
                identifier.push(c);
            } else {
                break;
            }
        }
        
        return identifier;
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume_token().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }

        return number;
    }

    fn consume_token(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() { return None; }
        let c = self.current_char();
        self.current_pos += 1;

        return c;
    }

    //TODO: Verify that this function works correctly
    fn check_if_double_operator(&mut self, expected: char, current_op: TokenType, potential_op: TokenType) -> TokenType {
        if let Some(next) = self.current_char() {
            if next == expected {
                self.consume_token();
                return potential_op;
            } else {
                return current_op;
            }
        }
        return current_op;
    }

    fn is_number_start(c: &char) -> bool { return c.is_digit(10); }

    fn is_identifier_start(c: &char) -> bool { return c.is_alphabetic(); }

    fn is_whitespace(c: &char) -> bool { return c.is_whitespace(); }

    fn current_char(&self) -> Option<char> { return self.input.chars().nth(self.current_pos); }
}