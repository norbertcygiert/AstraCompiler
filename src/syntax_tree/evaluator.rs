use crate::syntax_tree::syntax_tree::{StBinaryExpression, StNumeralExpression};

use super::traverser::AstTraverser;


pub struct Evaluator {
    pub last_value: Option<i64>,
}

impl Evaluator {
    pub fn new() -> Self{
        Self { last_value: None }
    }
}

impl AstTraverser for Evaluator {

    fn find_number(&mut self, num: &StNumeralExpression) {
        self.last_value = Some(num.value)
    }
    
    fn find_binary_expression(&mut self, expr: &StBinaryExpression) {
        self.find_expression(&expr.left);
        let left = self.last_value.unwrap();
        self.find_expression(&expr.right);
        let right = self.last_value.unwrap();
        self.last_value = Some(match expr.operator.kind {
            super::syntax_tree::StOperatorType::ADD => left + right,
            super::syntax_tree::StOperatorType::SUBTRACT => left - right,
            super::syntax_tree::StOperatorType::MULTIPLY => left * right,
            super::syntax_tree::StOperatorType::DIVIDE => left / right,
        });
    }

}