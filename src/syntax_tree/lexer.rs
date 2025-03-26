use std::fmt::{Display, Formatter};
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {     
    NUMBER(i64),     
    PLUS,     
    MINUS,     
    STAR,     
    SLASH,     
    LEFTPAR,     
    RIGHTPAR, 
    EOF,
    INVALID,
    WHITESPACE
} 
impl Display for TokenType{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       match self{
            TokenType::NUMBER(n) => write!(f, "Number: {}", n),
            TokenType::PLUS => write!(f, "+"),
            TokenType::MINUS => write!(f, "-"),
            TokenType::STAR => write!(f, "*"),
            TokenType::SLASH => write!(f, "/"),
            TokenType::LEFTPAR => write!(f, "("),
            TokenType::RIGHTPAR => write!(f, ")"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::INVALID => write!(f, "Invalid"),
            TokenType::WHITESPACE => write!(f, "Whitespace"),
       }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenSpan {     
    pub(crate) start: usize,     
    pub(crate) end: usize,     
    pub(crate) content: String, 
} 
#[allow(dead_code)]
impl TokenSpan {     
    
    pub fn new(start: usize, end: usize, content: String)-> Self {         
        Self {start, end, content}  
    }      
    
    pub fn len(&self) -> usize{ self.end - self.start }
}
#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub struct Token { 
    pub(crate) kind: TokenType, 
    pub(crate) span: TokenSpan,
}

impl Token {
    pub fn new(kind: TokenType, span: TokenSpan) -> Self {
        Self { kind, span }
    }
}
pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}
#[allow(dead_code, unused_assignments)]
impl <'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, current_pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {

        if self.current_pos == self.input.len() {
            let eof_indicator: char = '\0';
            self.current_pos += 1; 
            return Some(Token::new (
                TokenType::EOF,
                TokenSpan::new(0,0,eof_indicator.to_string())
            ));
        }
        
        let c  = self.current_char();

        return c.map(|c:char|{
            let start: usize = self.current_pos;
            let mut kind = TokenType::INVALID;
            if Self::is_start_of_number(&c){
                let number: i64 = self.consume_num();
                kind = TokenType::NUMBER(number);
            }
            else if Self::is_whitespace(&c) {
                self.consume_token();
                kind = TokenType::WHITESPACE;
            }
            else {
                kind = self.consume_symbol();
            }
            let end: usize = self.current_pos;
            let content: String = self.input[start..end].to_string();
            let local_span: TokenSpan = TokenSpan::new(start, end, content);
            Token::new(kind, local_span)
        });
    }

    fn consume_symbol(&mut self) -> TokenType{
        let c = self.consume_token().unwrap();
        match c {
            '+' =>  TokenType::PLUS,
            '-' => TokenType::MINUS,
            '*' => TokenType::STAR,
            '/' => TokenType::SLASH,
            '(' => TokenType::LEFTPAR,
            ')' => TokenType::RIGHTPAR,
            _ => TokenType::INVALID,
        }
    }
    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    /*
    fn peek_char(&self) -> Option<char>{
        self.input.chars().nth(self.current_pos+1)
    }
    */

    fn consume_token(&mut self) -> Option<char>{
        if self.current_pos >= self.input.len(){
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;
        c
    }

    fn consume_num(&mut self)-> i64{
        let mut number: i64 = 0;
        while let Some(c) = self.current_char(){
            //While there are more digits to be read, divide by 10 and append
            if c.is_digit(10){
                self.consume_token().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            }
            else {
                break;
            }
        } 
        number
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }
    fn is_start_of_number(c: &char) -> bool {
        c.is_digit(10)
    }

    
}