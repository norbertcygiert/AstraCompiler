use std::cell::Cell;

use crate::syntax_tree::{BinaryOperator, BinaryOperatorType, Expression, ASTStatement, UnaryOperator, UnaryOperatorType, lexer::{Token, TokenType}};
use crate::diagnostics::DiagnosticsVectorCell;

pub struct CompileTimeCounter {
    value: Cell<usize>,
}

impl CompileTimeCounter {
    pub fn new() -> Self {
        Self { value: Cell::new(0) }
    }

    pub fn add(&self, x: usize) {
        let current_value = self.value.get();
        self.value.set(current_value + x);
    }

    pub fn get_value(&self) -> usize {
        return self.value.get();
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: CompileTimeCounter,
    diagnostics_vec: DiagnosticsVectorCell,
}

impl Parser {
    pub fn new( tokens: Vec<Token>, diagnostics_vec: DiagnosticsVectorCell ) -> Self {
        Self {
            tokens: tokens.iter()
            .filter( |token| token.kind != TokenType::WHITESPACE )
            .map(|token| token.clone()).collect(),
            current: CompileTimeCounter::new(),
            diagnostics_vec,
        }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        if self.is_at_end() {
            return None;
        }
        return Some(self.parse_statement());
    }

    fn is_at_end(&self) -> bool {
        return self.current_token().kind == TokenType::EOF;
    }

    fn parse_statement(&mut self) -> ASTStatement {
        match self.current_token().kind {
            TokenType::LET => {
                self.parse_let_statement()
            },
            _ => {
                self.parse_expression_statement()
            }
        }
    }

    fn parse_let_statement(&mut self) -> ASTStatement {
        self.consume_with_check(TokenType::LET);
        let identifier = self.consume_with_check(TokenType::IDENTIFIER).clone();
        self.consume_with_check(TokenType::EQUALS);
        let expr = self.parse_expression();
        return ASTStatement::let_statement(identifier, expr);
    }

    fn parse_expression_statement(&mut self) -> ASTStatement {
        let expr = self.parse_expression();
        return ASTStatement::expression(expr);
    }

    fn parse_expression(&mut self) -> Expression {
        return self.parse_binary_expression(0);
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Expression {
        let mut left = self.parse_unary_expression();

        while let Some(operator) = self.parse_binary_operator() {
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            self.consume();
            let right = self.parse_binary_expression(operator_precedence);
            left = Expression::binary(operator, left, right);
        }

        return left;
    }

    fn parse_unary_expression(&mut self) -> Expression {
        if let Some(operator) = self.parse_unary_operator() {
            self.consume();
            let operand = self.parse_unary_expression();
            return Expression::unary(operator, operand);
        }
        return self.parse_primary_expression();
    }

    fn parse_unary_operator(&mut self) -> Option<UnaryOperator> {
        let token = self.current_token();
        let kind = match token.kind {
            TokenType::MINUS => {
                Some(UnaryOperatorType::MINUS)
            },
            TokenType::NOT => {
                Some(UnaryOperatorType::NOT)
            },
            _ => {
                None
            }
        };
        return kind.map(|kind| UnaryOperator::new(kind, token.clone()));
    }

    fn parse_binary_operator(&mut self) -> Option<BinaryOperator> {
        let token = self.current_token();
        let kind = match token.kind {
            TokenType::PLUS => {
                Some(BinaryOperatorType::PLUS)
            }
            TokenType::MINUS => { Some(BinaryOperatorType::MINUS) }
            TokenType::ASTERISK => { Some(BinaryOperatorType::MULTIPLY) }
            TokenType::SLASH => { Some(BinaryOperatorType::DIVIDE)},
            TokenType::AMPERSAND => { Some(BinaryOperatorType::AND) },
            TokenType::PIPE => { Some(BinaryOperatorType::OR) },
            TokenType::RETURN => { Some(BinaryOperatorType::XOR) },
            TokenType::POWER => { Some(BinaryOperatorType::POWER) },
            _ => { None }
        };
        return kind.map(|kind| BinaryOperator::new(kind, token.clone()));
    }

    fn parse_primary_expression(&mut self) -> Expression {
        let token = self.consume();
        return match token.kind {
            TokenType::NUMERAL(number) => {
                Expression::number(number)
            }
            TokenType::LEFTPAR => {
                let expr = self.parse_expression();
                self.consume_with_check(TokenType::RIGHTPAR);
                Expression::parenthesized(expr)
            }
            TokenType::IDENTIFIER => {
                Expression::identifier(token.clone())
            }
            _ => {
                self.diagnostics_vec.borrow_mut().report_expected_expression(token);
                Expression::error(
                    token.span.clone()
                )
            }
        };
    }

    fn peek(&self, offset: isize) -> &Token {
        let mut index = (self.current.get_value() as isize + offset) as usize;
        if index >= self.tokens.len() {
            index = self.tokens.len() - 1;
        }

        return self.tokens.get(index).unwrap();
    }

    fn current_token(&self) -> &Token { return self.peek(0); }

    fn consume(&self) -> &Token {
        self.current.add(1);
        return self.peek(-1);
    }

    fn consume_with_check(&self, kind: TokenType) -> &Token {
        let token = self.consume();
        if token.kind != kind {
            self.diagnostics_vec.borrow_mut().report_unexpected_token(&kind, token);
        }

        return token;
    }
}