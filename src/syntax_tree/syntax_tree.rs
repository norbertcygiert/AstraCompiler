
pub struct SyntaxTree {
    pub statements_vector: Vec<StStatement>,
} 
// Statements
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
//Expressions (obivously composed of statements)
#[derive(Debug)]
#[allow(dead_code, unused_variables)]
pub enum StExpressionType {
    NUMBER(i64), // Parsing only numbers for now
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
        Self::new(StExpressionType::NUMBER(value))
    }
}

