use super::lexer::{Token, TokenSpan};
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

    pub fn find(&self, traverser: &mut dyn AstTraverser){
        for s in &self.statements{
            traverser.find_statement(s);
        }
    }
}

pub trait AstTraverser {
    fn find_statement(&mut self, statement: &StStatement){
        self.rec_find_statement(statement);
    }
    fn rec_find_statement(&mut self, statement: &StStatement) {
        match &statement.kind {
            StStatementType::EXPRESSION(expr) => {
                self.find_expression(expr);
            }
        }
    }
    fn find_expression(&mut self, expression: &StExpression){
       self.rec_find_expression(expression);
    }
    fn rec_find_expression(&mut self, expression: &StExpression){
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
            StExpressionType::INVALID(span) => {
                self.find_error(span);
            }
        }
    }
    fn find_number(&mut self, num: &StNumeralExpression);
    fn find_error(&mut self, span: &TokenSpan);
    
    fn find_binary_expression(&mut self, expr: &StBinaryExpression){
        self.find_expression(&expr.left);
        self.find_expression(&expr.right);
    }

    fn find_parenthesized_expression(&mut self, expr: &StParenthesizedExpression){
        self.find_expression(&expr.expression);
    }

}



// Statements and their types
#[derive(Debug)]
pub enum StStatementType {
    EXPRESSION(StExpression),
}
#[derive(Debug)]
pub struct StStatement {
    kind: StStatementType,
}
impl StStatement {
    pub fn new(kind: StStatementType) -> Self {
        Self { kind }
    }
    pub fn expression(e: StExpression) -> Self {
        Self::new(StStatementType::EXPRESSION(e))
    }
}


// Expressions and their types, operators
#[derive(Debug)]
pub enum StBinaryOperatorType {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}
#[derive(Debug)]
#[allow(unused)]
pub struct StBinaryOperator {
    pub(crate)kind: StBinaryOperatorType,
    token: Token,
}
impl StBinaryOperator {
    pub fn new(kind: StBinaryOperatorType, token: Token) -> Self {
        Self { kind, token }
    }
    pub fn precedence(&self) -> u8 {
        match self.kind {
            StBinaryOperatorType::ADD => 1,
            StBinaryOperatorType::SUBTRACT => 1,
            StBinaryOperatorType::MULTIPLY => 2,
            StBinaryOperatorType::DIVIDE => 2,
        }
    }
}
#[derive(Debug)]
pub struct StBinaryExpression {
    pub(crate)left: Box<StExpression>, // These structs require Box to avoid infinite size
    pub(crate)operator: StBinaryOperator,
    pub(crate)right: Box<StExpression>,
}
#[derive(Debug)]
pub struct StNumeralExpression {
    pub(crate)value: i64,
}
#[derive(Debug)]
pub struct StParenthesizedExpression {
    expression: Box<StExpression>,
}

#[derive(Debug)]
pub enum StExpressionType {
    NUMBER(StNumeralExpression),
    BINARY(StBinaryExpression),
    PARENTHESIZED(StParenthesizedExpression),
    INVALID(TokenSpan),  
}

#[derive(Debug)]
pub struct StExpression {
    pub(crate) kind: StExpressionType,
}
impl StExpression {
    pub fn new(kind: StExpressionType) -> Self {
        StExpression { kind }
    }
    
    pub fn number(value: i64) -> Self {
        StExpression::new(StExpressionType::NUMBER(StNumeralExpression { value }))
    }

    pub fn binary(op: StBinaryOperator, left: StExpression, right: StExpression) -> Self {
        StExpression::new(StExpressionType::BINARY(StBinaryExpression{ 
            left: Box::new(left),
            operator: op, 
            right: Box::new(right)
        }))
    }

    pub fn parenthesized(expression: StExpression)  -> Self {
        StExpression::new(StExpressionType::PARENTHESIZED(StParenthesizedExpression { expression: Box::new(expression) }))
    }

    pub fn invalid(span: TokenSpan) -> Self {
        StExpression::new(StExpressionType::INVALID(span))
    }
}

