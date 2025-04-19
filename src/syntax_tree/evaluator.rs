use std::collections::HashMap;
use crate::syntax_tree::{BinaryExpression, BinaryOperatorType, ASTLetStatement, NumberExpression, ParenthesizedExpression, UnaryExpression, UnaryOperatorType, VariableExpression, ASTTraverser};
use crate::syntax_tree::lexer::SourceCodeSpan;

pub struct ASTEvaluator {
    pub last_value: Option<i64>,
    pub variables: HashMap<String, i64>,
}

impl ASTEvaluator {
    pub fn new() -> Self {
        Self { last_value: None, variables: HashMap::new() }
    }
}

impl ASTTraverser for ASTEvaluator {
    fn goto_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.goto_expression(&let_statement.initializer);
        self.variables.insert(let_statement.identifier.span.literal.clone(), self.last_value.unwrap());
    }

    fn goto_variable_expression(&mut self, variable_expression: &VariableExpression) {
        self.last_value = Some(*self.variables.get(&variable_expression.identifier.span.literal).unwrap());
    }

    fn goto_number_expression(&mut self, number: &NumberExpression) {
        self.last_value = Some(number.number);
    }

    fn goto_error(&mut self, span: &SourceCodeSpan) {
        // TODO
    }

    fn goto_unary_expression(&mut self, unary_expression: &UnaryExpression) {
        self.goto_expression(&unary_expression.operand);
        let operand = self.last_value.unwrap();
        self.last_value = Some(match unary_expression.operator.kind {
            UnaryOperatorType::MINUS => -operand,
            UnaryOperatorType::NOT => !operand,
        });
    }

    fn goto_binary_expression(&mut self, expr: &BinaryExpression) {
        self.goto_expression(&expr.left);
        let left = self.last_value.unwrap();
        self.goto_expression(&expr.right);
        let right = self.last_value.unwrap();
        self.last_value = Some(match expr.operator.kind {
            BinaryOperatorType::PLUS => left + right,
            BinaryOperatorType::MINUS => left - right,
            BinaryOperatorType::MULTIPLY => left * right,
            BinaryOperatorType::DIVIDE => left / right,
            BinaryOperatorType::POWER => left.pow(right as u32),
            BinaryOperatorType::AND => left & right,
            BinaryOperatorType::OR => left | right,
            BinaryOperatorType::XOR => left ^ right,
        });
    }

    fn goto_parenthesized_expression(&mut self, parenthesized_expression: &ParenthesizedExpression) {
        self.goto_expression(&parenthesized_expression.expression);
    }
}