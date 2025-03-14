#[derive(Debug)]
pub enum TokenType {     
    Number(i64),     
    Plus,     
    Minus,     
    Star,     
    Slash,     
    LeftPar,     
    RightPar, 
    EOF,
    Invalid,
} 
#[derive(Debug)]
pub struct TokenSpan {     
    start: usize,     
    end: usize,     
    content: String, 
} 
impl TokenSpan {     
    
    pub fn new(start: usize, end: usize, content: String)-> Self {         
        Self {start, end, content}  
    }      
    
    pub fn len(&self) -> usize{ self.end - self.start }
}
#[derive(Debug)]
pub struct Token { 
    kind: TokenType, 
    span: TokenSpan,
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

impl <'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, current_pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos > self.input.len(){
            return None;
        }

        if self.current_pos == self.input.len() {
            let eof_indicator: char = '\0';
            self.current_pos += 1; 
            return Some(Token::new (
                TokenType::EOF,
                TokenSpan::new(0,0,eof_indicator.to_string())
            ));
        }
        
        let start: usize = self.current_pos;
        let c: char = self.current_char();
        let mut kind = TokenType::Invalid;
        if Self::is_start_of_number(&c){
            let number: i64 = self.consume_num();
            kind = TokenType::Number(number);
        };
        let end: usize = self.current_pos;
        let literal: String = self.input[start..end].to_string();
        let local_span: TokenSpan = TokenSpan::new(start, end, literal);
        Some(Token::new(kind, local_span))
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.current_pos).unwrap()
    }

    fn consume_token(&mut self) -> Option<char>{
        let c: char = self.current_char();
        self.current_pos += 1;
        if self.current_pos >= self.input.len(){
            return None;
        }
        Some(c)
    }

    fn consume_num(&mut self)-> i64{
        let mut number: i64 = 0;
        while let Some(c) = self.consume_token(){
            //While there are more digits to be read, divide by 10 and append
            if c.is_digit(10){
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            }
            else {
                break;
            }
        } 
        number
    }

    fn is_start_of_number(c: &char) -> bool {
        c.is_digit(10)
    }

    
}