use super::syntax_tree::{StBinaryExpression, StExpression, StExpressionType, StNumeralExpression, StParenthesizedExpression, StStatement, StStatementType};

#[derive(Debug)]
pub struct ActualST {
    pub statements: Vec<StStatement>,
}

impl ActualST {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }

    pub fn push_statement(&mut self, s: StStatement){
        self.statements.push(s);
    }

    pub fn find(&mut self, traverser: &mut dyn AstTraverser){
        for s in &self.statements{
            traverser.find_statement(s);
        }
    }
}

pub trait AstTraverser {

    fn find_statement(&mut self, statement: &StStatement){
        match &statement.kind {
            StStatementType::EXPRESSION(expr) =>{
                self.find_expression(expr);
            }
        }

    }
    fn find_expression(&mut self, expression: &StExpression){
        match &expression.kind {
            StExpressionType::NUMBER(n) => {
                self.find_number(n);
            }
            StExpressionType::BINARY(b) => {
                self.find_binary_expression(b);
            },
            StExpressionType::PARENTHESIZED(p) => {
                self.find_parenthesized_expression(p);
            },
        }
    }
    fn find_number(&mut self, num: &StNumeralExpression);

    fn find_binary_expression(&mut self, expr: &StBinaryExpression){
        self.find_expression(&expr.left);
        self.find_expression(&expr.right);
    }

    fn find_parenthesized_expression(&mut self, expr: &StParenthesizedExpression){
        self.find_expression(&expr.expression);
    }
}