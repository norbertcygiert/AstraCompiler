

use crate::syntax_tree::lexer::{SourceCodeSpan, Token};
use printer::ASTPrinter;
use traverser::ASTTraverser;
pub mod lexer;
pub mod parser;
pub mod evaluator;
pub mod printer;
pub mod traverser;

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





#[derive(Debug, Clone)]
pub enum ASTStatementType {
    EXPRESSION(Expression),
    LET(ASTLetStatement),
    IF(ASTIFStatement),
    WHILE(ASTWhileStatement),
    BLOCK(ASTBlockStatement),
    FUNCTION(ASTFunctionStatement),
    RETURN(ASTReturnStatement),

}
#[derive(Debug, Clone)]
pub struct ASTLetStatement {
    pub identifier: Token,
    pub initializer: Expression,
}

#[derive(Debug, Clone)]
pub struct ASTIFStatement {
    pub if_keyword: Token,
    pub condition: Expression,
    pub then_branch: Box<ASTStatement>,
    pub else_branch: Option<ASTElseStatement>,
}
#[derive(Debug, Clone)]
pub struct ASTElseStatement {
    pub else_keyword: Token,
    pub else_statement: Box<ASTStatement>,
}
#[derive(Debug, Clone)]
pub struct ASTWhileStatement {
    pub while_keyword: Token,
    pub condition: Expression,
    pub body: Box<ASTStatement>,
}
#[derive(Debug, Clone)]
pub struct ASTBlockStatement {
    pub statements: Vec<ASTStatement>,
}

#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub identifier: Token,
}
#[derive(Debug, Clone)]
pub struct ASTFunctionStatement {
    pub identifier: Token,
    pub parameters: Vec<FunctionParameter>,
    pub body: Box<ASTStatement>,
}
#[derive(Debug, Clone)]
pub struct ASTReturnStatement {
    pub return_keyword: Token,
    pub return_value: Option<Expression>,
}


impl ASTElseStatement {
    pub fn new(else_keyword: Token, else_statement: ASTStatement) -> Self {
        Self { else_keyword, else_statement: Box::new(else_statement) }
    }
}


#[derive(Debug, Clone)]
pub struct ASTStatement {
    kind: ASTStatementType,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementType) -> Self { ASTStatement { kind } }

    pub fn expression(expr: Expression) -> Self {
        return ASTStatement::new(ASTStatementType::EXPRESSION(expr));
    }

    pub fn let_statement(identifier: Token, initializer: Expression) -> Self {
        return ASTStatement::new(ASTStatementType::LET(ASTLetStatement { identifier, initializer }));
    }

    pub fn if_statement(if_keyword: Token, condition: Expression, then_branch: ASTStatement, else_branch: Option<ASTElseStatement>) -> Self {
        return ASTStatement::new(ASTStatementType::IF(ASTIFStatement { if_keyword, condition, then_branch: Box::new(then_branch), else_branch }));
    }
    
    pub fn while_statement(while_keyword: Token, condition: Expression, body: ASTStatement) -> Self {
        return ASTStatement::new(ASTStatementType::WHILE(ASTWhileStatement { while_keyword, condition, body: Box::new(body) }));
    }

    pub fn block_statement(statements: Vec<ASTStatement>) -> Self {
        return ASTStatement::new(ASTStatementType::BLOCK(ASTBlockStatement { statements }));
    }

    pub fn function(identifier: Token, parameters: Vec<FunctionParameter>, body: ASTStatement) -> Self {
        return ASTStatement::new(ASTStatementType::FUNCTION(ASTFunctionStatement { identifier, parameters, body: Box::new(body) }));
    }

    pub fn return_statement(return_keyword: Token, return_value: Option<Expression>) -> Self {
        return ASTStatement::new(ASTStatementType::RETURN(ASTReturnStatement { return_keyword, return_value }));
    }
}
#[derive(Debug, Clone)]
pub enum ExpressionType {
    NUMBER( NumberExpression ),
    BINARY( BinaryExpression ),
    UNARY( UnaryExpression ),
    PARENTHESIZED( ParenthesizedExpression ),
    VARIABLE( VariableExpression ),
    ASSIGNMENT( AssignmentExpression),
    FUNCTIONCALL( FunctionCallExpression ),
    BOOLEAN( BooleanExpression ),
    ERROR( SourceCodeSpan ),
}
#[derive(Debug, Clone)]
pub enum UnaryOperatorType {
    MINUS,
    NOT,
}
#[derive(Debug, Clone)]
pub struct UnaryOperator {
    kind: UnaryOperatorType,
    token: Token,
}
impl UnaryOperator {
    pub fn new(kind: UnaryOperatorType, token: Token) -> Self {
        UnaryOperator { kind, token }
    }
}
#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct VariableExpression {
    pub identifier: Token,
}

impl VariableExpression {
    pub fn identifier(&self) -> &str { return &self.identifier.span.literal; }
}

#[derive(Debug, Clone)]
pub enum BinaryOperatorType {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    POWER,
    AND,
    OR,
    XOR,
    EQUALS,
    NOTEQUALS,
    GREATER,
    LESS,
    GREATEREQUALS,
    LESSEQUALS,

}
#[derive(Debug, Clone)]
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
            BinaryOperatorType::EQUALS => 30,
            BinaryOperatorType::NOTEQUALS => 30,
            BinaryOperatorType::GREATER => 29,
            BinaryOperatorType::LESS => 29,
            BinaryOperatorType::GREATEREQUALS => 29,
            BinaryOperatorType::LESSEQUALS => 29,
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
#[derive(Debug, Clone)]
pub struct BinaryExpression {
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct NumberExpression {
    number: i64,
}
#[derive(Debug, Clone)]
pub struct ParenthesizedExpression {
    expression: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub token: Token,
    pub expression: Box<Expression>,
}
#[derive(Debug, Clone)]
pub struct FunctionCallExpression {
    pub identifier: Token,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct BooleanExpression {
    pub token: Token,
    pub value: bool,
}

#[derive(Debug, Clone)]
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

    pub fn assignment(token: Token, expression: Expression) -> Self {
        return Expression::new(ExpressionType::ASSIGNMENT(AssignmentExpression { token, expression: Box::new(expression) }));
    }

    pub fn boolean(token: Token, value: bool) -> Self {
        return Expression::new(ExpressionType::BOOLEAN(BooleanExpression { token, value }));
    }

    pub fn function_call(identifier: Token, arguments: Vec<Expression>) -> Self {
        return Expression::new(ExpressionType::FUNCTIONCALL(FunctionCallExpression { identifier, arguments }));
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
        BOOLEAN(bool),
        FUNCTIONCALL,
        LETSTATEMENT,
        ASSIGNMENT,
        BLOCK,
        IF,
        WHILE,
        RETURN,
        ELSE,
        FUNC,
        BINARY,
        UNARY,
        PARENTHESIZED,
        LET,
        VAR(String),
    }

    struct ASTVerifier {
        expected: Vec<TestASTNode>,
        actual: Vec<TestASTNode>
    }

    impl ASTVerifier {
        pub fn new(input: &str, expected: Vec<TestASTNode>) -> Self {
            let compilation_unit = CompilationUnit::compile(input).expect("Astra Compiler: Compilation failed");
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

    impl ASTTraverser<'_> for ASTVerifier {
        fn goto_function_statement(&mut self, function_statement: &super::ASTFunctionStatement) {
            self.actual.push(TestASTNode::FUNC);
            self.goto_statement(&function_statement.body);
        }

        fn goto_function_call_expression(&mut self, function_call_expression: &super::FunctionCallExpression) {
            self.actual.push(TestASTNode::FUNCTIONCALL);
            for argument in &function_call_expression.arguments {
                self.goto_expression(argument);
            }    
        }

        fn goto_return_statement(&mut self, return_statement: &super::ASTReturnStatement) {
            self.actual.push(TestASTNode::RETURN);
            if let Some(return_value) = &return_statement.return_value {
                self.goto_expression(return_value);
            }
        }
        fn goto_if_statement(&mut self, if_statement: &super::ASTIFStatement) {
            self.actual.push(TestASTNode::IF);
            self.goto_expression(&if_statement.condition);
            self.goto_statement(&if_statement.then_branch);
            if let Some(else_branch) = &if_statement.else_branch {
                self.actual.push(TestASTNode::ELSE);
                self.goto_statement(&else_branch.else_statement);
            }
        }

        fn goto_while_statement(&mut self, while_statement: &super::ASTWhileStatement) {
            self.actual.push(TestASTNode::WHILE);
            self.goto_expression(&while_statement.condition);
            self.goto_statement(&while_statement.body);
        }

        fn goto_assignment_expression(&mut self, assignment_expression: &super::AssignmentExpression) {
            self.actual.push(TestASTNode::ASSIGNMENT);
            self.goto_expression(&assignment_expression.expression);
        }
        fn goto_boolean_expression(&mut self, boolean_expression: &super::BooleanExpression) {
            self.actual.push(TestASTNode::BOOLEAN(boolean_expression.value));
        }
        fn goto_block_statement(&mut self, block_statement: &super::ASTBlockStatement) {
            self.actual.push(TestASTNode::BLOCK);
            for statement in &block_statement.statements {
                self.goto_statement(statement);
            }
        }
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