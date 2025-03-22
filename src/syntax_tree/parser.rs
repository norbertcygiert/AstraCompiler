use crate::syntax_tree::syntax_tree::{StBinaryOperator, StExpression, StBinaryOperatorType, StStatement};
use crate::syntax_tree::lexer::{Token, TokenType};
#[allow(dead_code, unused_variables)]
pub struct Parser {
    tokens: Vec<Token>,
    current_pos: usize,
}
#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.iter().filter(|t| t.kind != TokenType::WHITESPACE).map(|t| t.clone()).collect(),
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
        return self.parse_binary_expression(0);
    }
    
    fn parse_primary_expression(&mut self) -> Option<StExpression> {
        let token = self.consume_token()?;
        return match token.kind {
            super::lexer::TokenType::NUMBER(val) => {
                Some(StExpression::number(val))
            },

            super::lexer::TokenType::LEFTPAR => {
                let expr = self.parse_expression()?; // Parentheses containing another expression
                let token = self.consume_token()?;
                if token.kind != super::lexer::TokenType::RIGHTPAR {
                    panic!("Expected Right Parentheses near token: {:?}", token);
                }
                Some(StExpression::parenthesized(expr))
            },

            _ => { None }
        }
    }

    fn parse_binary_operator(&mut self) -> Option<StBinaryOperator> {
        let token = self.current_token()?;
        let k = match token.kind {
            TokenType::PLUS => { Some(StBinaryOperatorType::ADD) },
            TokenType::MINUS => { Some(StBinaryOperatorType::SUBTRACT) },
            TokenType::STAR => { Some(StBinaryOperatorType::MULTIPLY) },
            TokenType::SLASH => { Some(StBinaryOperatorType::DIVIDE) },
            _ => None,
        };
        return k.map(|k| StBinaryOperator::new(k, token.clone()));
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<StExpression> {
        let mut left = self.parse_primary_expression()?;
        while let Some(op) = self.parse_binary_operator() { //checking next operator precedence
           
            if op.precedence() < precedence {
                break;
            }
            self.consume_token();
            let right = self.parse_binary_expression(op.precedence() + 1)?;
            left = StExpression::binary(op, left, right);
        };
        
        return Some(left);
    }

    //fn precedence(&Token) -> u8 {

    //}


    fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get((self.current_pos as isize + offset) as usize)
    }

    fn current_token(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume_token(&mut self) -> Option<&Token> {
        self.current_pos += 1;
        let token = self.peek(-1)?;
        return Some(token);
    }


}