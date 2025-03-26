use std::cell::Cell;

use crate::diagnostics::DiagnosticsVecCell;
use crate::syntax_tree::syntax_tree::{StBinaryOperator, StExpression, StBinaryOperatorType, StStatement};
use crate::syntax_tree::lexer::{Token, TokenType};
#[allow(dead_code, unused_variables)]

pub struct ProgramCounter {
    value: Cell<usize>,
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self { value: Cell::new(0) }
    }
    pub fn add(&self, x: usize) {
        self.value.set(self.value.get() + x);
    }
    pub fn get(&self) -> usize {
        self.value.get()
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current_pos: ProgramCounter,
    diagnostics_vec: DiagnosticsVecCell,
}
#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>, diagnostics_vec: DiagnosticsVecCell ) -> Self {
        Self {
            tokens: tokens.iter().filter(|t| t.kind != TokenType::WHITESPACE).map(|t| t.clone()).collect(),
            current_pos: ProgramCounter::new(),
            diagnostics_vec,
        }
    }

    pub fn next_statement(&mut self) -> Option<StStatement> {
        if self.current_token().kind == TokenType::EOF {
            return None;
        }
        return Some(self.parse_statement());
    }

    fn parse_statement(&mut self) -> StStatement {
        let expr = self.parse_expression();
        return StStatement::expression(expr);
    }

    fn parse_expression(&mut self) -> StExpression{
        return self.parse_binary_expression(0);
    }
    
    fn parse_primary_expression(&mut self) -> StExpression {
        let token: &Token = self.consume_token();
        return match token.kind {
            super::lexer::TokenType::NUMBER(val) => {
                StExpression::number(val)
            },
            super::lexer::TokenType::LEFTPAR => {
                let expr = self.parse_expression(); // Parentheses containing another expression
                let token = self.consume_token();
                if token.kind != super::lexer::TokenType::RIGHTPAR {
                    panic!("Expected Right Parentheses near token: {:?}", token);
                }
                StExpression::parenthesized(expr)
            },
            _ => { 
                self.diagnostics_vec.borrow_mut().throw_expected_expression(token); 
                StExpression::invalid(token.span.clone())
            }
        }
    }

    fn parse_binary_operator(&mut self) -> Option<StBinaryOperator> {
        let token = self.current_token();
        let k = match token.kind {
            TokenType::PLUS => { Some(StBinaryOperatorType::ADD) },
            TokenType::MINUS => { Some(StBinaryOperatorType::SUBTRACT) },
            TokenType::STAR => { Some(StBinaryOperatorType::MULTIPLY) },
            TokenType::SLASH => { Some(StBinaryOperatorType::DIVIDE) },
            _ => { None }
        };
        return k.map(|k| StBinaryOperator::new(k, token.clone()));
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> StExpression {
        let mut left = self.parse_primary_expression();
        while let Some(op) = self.parse_binary_operator() { //checking next operator precedence
            if op.precedence() < precedence {
                break;
            }
            self.consume_token();
            let right = self.parse_binary_expression(op.precedence() + 1);
            left = StExpression::binary(op, left, right);
        };
        
        return left;
    }

    fn peek(&self, offset: isize) -> &Token {
        let mut index = (self.current_pos.get() as isize + offset) as usize;
        if index >= self.tokens.len() {
            index = self.tokens.len() - 1;
        }
        self.tokens.get(index).unwrap()
    }

    fn current_token(&self) -> &Token {
        self.peek(0)
    }

    fn consume_token(&self) -> &Token {
        self.current_pos.add(1);
        self.peek(-1)
    }

    fn consume_with_check(&self, kind: TokenType) -> &Token {
        let token = self.consume_token();
        if token.kind != kind {
            self.diagnostics_vec.borrow_mut().throw_unexpected_token(&kind, token);
        }
        token
    }


}