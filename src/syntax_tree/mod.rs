use termion::color;

use crate::syntax_tree::lexer::{SourceCodeSpan, Token};

pub mod lexer;
pub mod parser;
pub mod evaluator;

pub struct AbstractSyntaxTree {
    pub statements: Vec<ASTStatement>,
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, visitor: &mut dyn ASTTraverser) {
        for statement in &self.statements {
            visitor.goto_statement(statement);
        }
    }

    pub fn visualize(&self) -> () {
        let mut printer = ASTPrinter::new();
        self.visit(&mut printer);
        println!("{}", printer.result);
    }
}

pub trait ASTTraverser {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementType::EXPRESSION(expr) => {
                self.goto_expression(expr);
            }
            ASTStatementType::LETSTATEMENT(expr) => {
                self.goto_let_statement(expr);
            }
        }
    }

    fn goto_let_statement(&mut self, let_statement: &ASTLetStatement);

    fn goto_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }

    fn expression_dispatch(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionType::NUMBER(number) => {
                self.goto_number_expression(number);
            }
            ExpressionType::BINARY(expr) => {
                self.goto_binary_expression(expr);
            }
            ExpressionType::PARENTHESIZED(expr) => {
                self.goto_parenthesized_expression(expr);
            }
            ExpressionType::ERROR(span) => {
                self.goto_error(span);
            }
            ExpressionType::VARIABLE(expr) => {
                self.goto_variable_expression(expr);
            }
            ExpressionType::UNARY(expr) => {
                self.goto_unary_expression(expr);
            }
        }
    }

    fn goto_expression(&mut self, expression: &Expression) { self.expression_dispatch(expression); }


    //TODO: Add a visitor for different expression types
    fn goto_variable_expression(&mut self, variable_expression: &VariableExpression);

    fn goto_number_expression(&mut self, number: &NumberExpression);

    fn goto_error(&mut self, span: &SourceCodeSpan);

    fn goto_unary_expression(&mut self, unary_expression: &UnaryExpression);

    fn goto_binary_expression(&mut self, binary_expression: &BinaryExpression) {
        self.goto_expression(&binary_expression.left);
        self.goto_expression(&binary_expression.right);
    }

    fn goto_parenthesized_expression(&mut self, parenthesized_expression: &ParenthesizedExpression) {
        self.goto_expression(&parenthesized_expression.expression);
    }

}

pub struct ASTPrinter {
    indent: usize,
    result: String,
}

impl ASTPrinter {
    const NUMBER_COLOR: color::LightGreen = color::LightGreen;
    const TEXT_COLOR: color::LightWhite = color::LightWhite;
    const KEYWORD_COLOR: color::Blue = color::Blue;
    const VARIABLE_COLOR: color::LightBlue = color::LightBlue;

    fn add_space(&mut self) { self.result.push_str(" "); }

    fn add_newline(&mut self) { self.result.push_str("\n"); }

    pub fn new() -> Self {
        Self { indent: 0, result: String::new() }
    }
}

impl ASTTraverser for ASTPrinter {
    fn goto_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.result.push_str(&format!("{}let", Self::KEYWORD_COLOR.fg_str()));
        self.add_space();
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), let_statement.identifier.span.literal, ));
        self.add_space();
        self.result.push_str(&format!("{}=", Self::TEXT_COLOR.fg_str(), ));
        self.add_space();
        self.goto_expression(&let_statement.initializer);
    }

    fn goto_statement(&mut self, statement: &ASTStatement) {
        Self::do_visit_statement(self, statement);
        self.result.push_str(&format!("{}\n", color::Fg(color::Reset) ));
    }

    fn goto_variable_expression(&mut self, variable_expression: &VariableExpression) {
        self.result.push_str(&format!("{}{}", Self::VARIABLE_COLOR.fg_str(), variable_expression.identifier.span.literal ));
    }

    fn goto_number_expression(&mut self, number: &NumberExpression) {
        self.result.push_str(&format!("{}{}", Self::NUMBER_COLOR.fg_str(), number.number ));
    }

    fn goto_error(&mut self, span: &SourceCodeSpan) {
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), span.literal ));
    }

    fn goto_unary_expression(&mut self, unary_expression: &UnaryExpression) {
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), unary_expression.operator.token.span.literal ));
        self.goto_expression(&unary_expression.operand);
    }

    fn goto_binary_expression(&mut self, binary_expression: &BinaryExpression) {
        self.goto_expression(&binary_expression.left);
        self.add_space();
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), binary_expression.operator.token.span.literal, ));
        self.add_space();
        self.goto_expression(&binary_expression.right);
    }

    fn goto_parenthesized_expression(&mut self, parenthesized_expression: &ParenthesizedExpression) {
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), "(", ));
        self.goto_expression(&parenthesized_expression.expression);
        self.result.push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), ")", ));
    }
}


pub enum ASTStatementType {
    EXPRESSION(Expression),
    LETSTATEMENT(ASTLetStatement),
}

pub struct ASTLetStatement {
    pub identifier: Token,
    pub initializer: Expression,
}

pub struct ASTStatement {
    kind: ASTStatementType,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementType) -> Self { ASTStatement { kind } }

    pub fn expression(expr: Expression) -> Self {
        return ASTStatement::new(ASTStatementType::EXPRESSION(expr));
    }

    pub fn let_statement(identifier: Token, initializer: Expression) -> Self {
        return ASTStatement::new(ASTStatementType::LETSTATEMENT(ASTLetStatement { identifier, initializer }));
    }
}

pub enum ExpressionType {
    NUMBER( NumberExpression ),
    BINARY( BinaryExpression ),
    UNARY( UnaryExpression ),
    PARENTHESIZED( ParenthesizedExpression ),
    VARIABLE( VariableExpression ),
    ERROR( SourceCodeSpan ),
}

pub enum UnaryOperatorType {
    MINUS,
    NOT,
}
pub struct UnaryOperator {
    kind: UnaryOperatorType,
    token: Token,
}
impl UnaryOperator {
    pub fn new(kind: UnaryOperatorType, token: Token) -> Self {
        UnaryOperator { kind, token }
    }
}

pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

pub struct VariableExpression {
    pub identifier: Token,
}

impl VariableExpression {
    pub fn identifier(&self) -> &str { return &self.identifier.span.literal; }
}

#[derive(Debug)]
pub enum BinaryOperatorType {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    POWER,
    AND,
    OR,
    XOR,
}

pub struct BinaryOperator {
    kind: BinaryOperatorType,
    token: Token,
}

impl BinaryOperator {
    pub fn new(kind: BinaryOperatorType, token: Token) -> Self {
        BinaryOperator { kind, token }
    }

    pub fn precedence(&self) -> u8 {
        match self.kind {
            //Wikipedia based precedence table
            BinaryOperatorType::POWER => 13,
            BinaryOperatorType::MULTIPLY => 12,
            BinaryOperatorType::DIVIDE => 12,
            BinaryOperatorType::PLUS => 11,
            BinaryOperatorType::MINUS => 11,
            BinaryOperatorType::AND => 7,
            BinaryOperatorType::XOR => 6,
            BinaryOperatorType::OR => 5,
        }
    }
}

pub struct BinaryExpression {
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
}

pub struct NumberExpression {
    number: i64,
}

pub struct ParenthesizedExpression {
    expression: Box<Expression>,
}

pub struct Expression {
    kind: ExpressionType,
}

impl Expression {
    pub fn new(kind: ExpressionType) -> Self {
        Self { kind }
    }

    pub fn number(number: i64) -> Self {
        return Expression::new(ExpressionType::NUMBER(NumberExpression { number }));
    }

    pub fn binary(operator: BinaryOperator, left: Expression, right: Expression) -> Self {
        return Expression::new(ExpressionType::BINARY(BinaryExpression { left: Box::new(left), operator, right: Box::new(right) }));
    }

    pub fn parenthesized(expression: Expression) -> Self {
        return Expression::new(ExpressionType::PARENTHESIZED(ParenthesizedExpression { expression: Box::new(expression) }));
    }

    pub fn identifier(identifier: Token) -> Self {
        return Expression::new(ExpressionType::VARIABLE(VariableExpression { identifier }));
    }

    pub fn unary(operator: UnaryOperator, operand: Expression) -> Self {
        return Expression::new(ExpressionType::UNARY(UnaryExpression { operator, operand: Box::new(operand) }));
    }

    pub fn error(span: SourceCodeSpan) -> Self {
        return Expression::new(ExpressionType::ERROR(span));
    }
}

#[cfg(test)]
mod test {
    use crate::syntax_tree::{AbstractSyntaxTree, BinaryExpression, ASTLetStatement, NumberExpression, ParenthesizedExpression, UnaryExpression, VariableExpression, ASTTraverser, lexer::SourceCodeSpan};
    use crate::compilation_unit::CompilationUnit;

    #[derive(Debug, PartialEq, Eq)]
    enum TestASTNode {
        NUMBER(i64),
        BINARY,
        UNARY,
        PARENTHESIZED,
        LETSTATEMENT,
        VAR(String),
    }

    struct ASTVerifier {
        expected: Vec<TestASTNode>,
        actual: Vec<TestASTNode>
    }

    impl ASTVerifier {
        pub fn new(input: &str, expected: Vec<TestASTNode>) -> Self {
            let compilation_unit = CompilationUnit::compile(input);
            assert_eq!(compilation_unit.diagnostics_vector.borrow().diagnostics.len(), 0, "Expected no diagnostics, but got {:?}", compilation_unit.diagnostics_vector.borrow().diagnostics);
            let mut verifier = ASTVerifier { expected, actual: Vec::new() };
            verifier.flatten_ast(&compilation_unit.ast);
            return verifier;
        }

        fn flatten_ast(&mut self, ast: &AbstractSyntaxTree)  {
            self.actual.clear();
            ast.visit( self);
        }

        pub fn verify(&self) {
            //First check if the number of nodes is the same
            assert_eq!(self.expected.len(), self.actual.len(), "Expected {} nodes, but got {}. Actual nodes: {:?}", self.expected.len(), self.actual.len(), self.actual);

            //Then just go through the zipped iterators (pairs of nodes which should be the same) 
            for (index, (expected, actual)) in self.expected.iter()
            .zip( self.actual.iter() )
            .enumerate() {
                assert_eq!(expected, actual, "Expected {:?} at index {}, but got {:?}", expected, index, actual);
            }
        }
    }

    impl ASTTraverser for ASTVerifier {
        fn goto_let_statement(&mut self, let_statement: &ASTLetStatement) {
            self.actual.push(TestASTNode::LETSTATEMENT);
            self.goto_expression(&let_statement.initializer);
        }

        fn goto_variable_expression(&mut self, variable_expression: &VariableExpression) {
            self.actual.push(TestASTNode::VAR( variable_expression.identifier().to_string() ));
        }

        fn goto_number_expression(&mut self, number: &NumberExpression) {
            self.actual.push(TestASTNode::NUMBER(number.number));
        }

        fn goto_unary_expression(&mut self, unary_expression: &UnaryExpression) {
            self.actual.push(TestASTNode::UNARY);
            self.goto_expression(&unary_expression.operand);
        }

        fn goto_binary_expression(&mut self, binary_expression: &BinaryExpression) {
            self.actual.push(TestASTNode::BINARY);
            self.goto_expression(&binary_expression.left);
            self.goto_expression(&binary_expression.right);
        }

        fn goto_parenthesized_expression(&mut self, parenthesized_expression: &ParenthesizedExpression) {
            self.actual.push(TestASTNode::PARENTHESIZED);
            self.goto_expression(&parenthesized_expression.expression);
        }

        fn goto_error(&mut self, span: &SourceCodeSpan) {
            //TODO
        }

    }


    fn assert_ast(input: &str, expected: Vec<TestASTNode>) {
        let verifier = ASTVerifier::new(input, expected);
        verifier.verify();
    }
}