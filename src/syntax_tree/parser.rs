use super::{lexer::Token, syntax_tree::{StExpression, StStatement}};
use crate::syntax_tree::lexer::{Lexer, TokenType};
#[allow(dead_code, unused_variables)]
pub struct Parser {
    tokens: Vec<Token>,
    current_pos: usize,
}
#[allow(dead_code)]
impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current_pos: 0,
        }
    }
    pub fn create_from_tokens(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_pos: 0,
        }
    }
    pub fn create_from_input(input:&str) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        Self {
            tokens,
            current_pos: 0,
        }
    }
    pub fn parse_input(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        Self {
            tokens,
            current_pos: 0,
        }
    }   

    pub fn next_statement(&mut self) -> Option<StStatement> {
        return self.parse_statement();
    }

    fn parse_statement(&mut self) -> Option<StStatement> {
        let token= self.current_token()?;
        if token.kind == TokenType::EOF {
            return None;
        }
        let expr: StExpression = self.parse_expression()?;
        return Some(StStatement::expression(expr));
    }

    fn parse_expression(&mut self) -> Option<StExpression> {
        let token = self.consume_expr()?;
        return match token.kind {
            super::lexer::TokenType::NUMBER(val) => {
                Some(StExpression::number(val))
            },
            _ => { None }
        }
    }

    fn peek(&self, offset: isize  ) -> Option<&Token> {
        self.tokens.get((self.current_pos as isize + offset) as usize)
    }

    fn current_token(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume_expr(&mut self) -> Option<&Token> {
        self.current_pos += 1;
        let token = self.peek(-1)?;
        return Some(token)
    }


}