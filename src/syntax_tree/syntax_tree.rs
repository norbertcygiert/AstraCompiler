//The syntax tree itself
#[derive(Debug)]
pub struct SyntaxTree {
    pub statements_vector: Vec<StStatement>,
}
#[allow(dead_code)]
impl SyntaxTree {
    pub fn new() -> Self {
        Self {
            statements_vector: Vec::new(),
        }
    }
    pub fn add_statement(&mut self, statement: StStatement) {
        self.statements_vector.push(statement);
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
pub enum StOperatorType {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}
#[derive(Debug)]
pub struct StBinaryOperator {
    kind: StOperatorType,
    token: super::lexer::Token,
}
impl StBinaryOperator {
    pub fn new(kind: StOperatorType, token: super::lexer::Token) -> Self {
        Self { kind, token }
    }
    pub fn precedence(&self) -> u8 {
        match self.kind {
            StOperatorType::ADD => 1,
            StOperatorType::SUBTRACT => 1,
            StOperatorType::MULTIPLY => 2,
            StOperatorType::DIVIDE => 2,
        }
    }
}
#[derive(Debug)]
pub struct StBinaryExpression {
    pub left: Box<StExpression>, // These structs require Box to avoid infinite size
    pub operator: StBinaryOperator,
    pub right: Box<StExpression>,
}
#[derive(Debug)]
pub struct StNumeralExpression {
    value: i64,
}
#[derive(Debug)]
pub struct StParenthesizedExpression {
    expression: Box<StExpression>,
}

#[derive(Debug)]
pub enum StExpressionType {
    NUMBER(StNumeralExpression),
    BINARY(StBinaryExpression),
    PARENTHESIZED(StParenthesizedExpression)
}


#[derive(Debug)]
pub struct StExpression {
    kind: StExpressionType,
}
impl StExpression {
    pub fn new(kind: StExpressionType) -> Self {
        Self { kind }
    }
    
    pub fn number(value: i64) -> Self {
        Self::new(StExpressionType::NUMBER(StNumeralExpression { value }))
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
}

