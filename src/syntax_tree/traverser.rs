use crate::syntax_tree::*;

pub trait ASTTraverser<'a> {
    fn statement_dispatch(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementType::EXPRESSION(expr) => {
                self.goto_expression(expr);
            },
            ASTStatementType::LET(expr) => {
                self.goto_let_statement(expr);
            },
            ASTStatementType::IF(if_statement) => {
                self.goto_if_statement(if_statement);
            },
            ASTStatementType::WHILE(while_statement) => {
                self.goto_while_statement(while_statement);
            },
            ASTStatementType::BLOCK(block_statement) => {
                self.goto_block_statement(block_statement);
            },
            ASTStatementType::FUNCTION(function_statement) => {
                self.goto_function_statement(function_statement);
            },
            ASTStatementType::RETURN(ret_statement) => {
                self.goto_return_statement(ret_statement);
            },
        }
    }

    fn expression_dispatch(&mut self, expression: &Expression) {
        match &expression.kind {
            ExpressionType::NUMBER(number) => self.goto_number_expression(number),
            ExpressionType::BINARY(expr) => self.goto_binary_expression(expr),
            ExpressionType::PARENTHESIZED(expr) => self.goto_parenthesized_expression(expr),
            ExpressionType::VARIABLE(expr) => self.goto_variable_expression(expr),
            ExpressionType::UNARY(expr) => self.goto_unary_expression(expr),
            ExpressionType::BOOLEAN(expr) => self.goto_boolean_expression(expr),
            ExpressionType::FUNCTIONCALL(expr) => self.goto_function_call_expression(expr),
            ExpressionType::ASSIGNMENT(expr) => self.goto_assignment_expression(expr),
            ExpressionType::ERROR(span) => self.goto_error(span),
        }
    }

    fn goto_expression(&mut self, expression: &Expression) { 
        self.expression_dispatch(expression); 
    }

    fn goto_statement(&mut self, statement: &ASTStatement) {
        self.statement_dispatch(statement);
    }
    

    //Expressions

    fn goto_variable_expression(&mut self, variable_expression: &VariableExpression);

    fn goto_number_expression(&mut self, number: &NumberExpression);

    fn goto_unary_expression(&mut self, unary_expression: &UnaryExpression);

    fn goto_binary_expression(&mut self, binary_expression: &BinaryExpression) {
        self.goto_expression(&binary_expression.left);
        self.goto_expression(&binary_expression.right);
    }

    fn goto_parenthesized_expression(&mut self, parenthesized_expression: &ParenthesizedExpression) {
        self.goto_expression(&parenthesized_expression.expression);
    }

    fn goto_boolean_expression(&mut self, boolean_expression: &BooleanExpression) {}

    fn goto_function_call_expression(&mut self, function_call_expression: &FunctionCallExpression) {
        for arg in &function_call_expression.arguments {
            self.goto_expression(arg);
        }
    }
    
    fn goto_assignment_expression(&mut self, assignment_expression: &AssignmentExpression) {
        self.goto_expression(&assignment_expression.expression)
    }

    fn goto_error(&mut self, span: &SourceCodeSpan);
    
    //Statements
    
    fn goto_let_statement(&mut self, let_statement: &ASTLetStatement);
    
    fn goto_if_statement(&mut self, if_statement: &ASTIfStatement) {
        self.goto_expression(&if_statement.condition);
        self.goto_statement(&if_statement.then_branch);
        if let Some(else_branch) = &if_statement.else_branch {
            self.goto_statement(&else_branch.else_statement);
        }
    }

    fn goto_while_statement(&mut self, while_statement: &ASTWhileStatement) {
        self.goto_expression(&while_statement.condition);
        self.goto_statement(&while_statement.body);
    }

    fn goto_block_statement(&mut self, block_statement: &ASTBlockStatement) {
        for statement in &block_statement.statements {
            self.goto_statement(statement);
        }
    }

    fn goto_function_statement(&mut self, function_statement: &ASTFunctionStatement) { }

    fn goto_return_statement(&mut self, return_statement: &ASTReturnStatement) {
        if let Some(expr) = &return_statement.return_value {
            self.goto_expression(expr);
        }
    }
    

}