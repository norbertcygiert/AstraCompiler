pub enum TokenType {     
    Number(i64),     
    Plus,     
    Minus,     
    Star,     
    Slash,     
    LeftPar,     
    RightPar, 
} 
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


pub struct Token { 
    kind: TokenType, 
}