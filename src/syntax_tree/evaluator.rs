use std::collections::HashMap;
use crate::compilation_unit::GlobalScope;
use crate::syntax_tree::*;


pub struct Frame {
    variables: HashMap<String, i64>,
}

impl Frame {
    pub fn new() -> Self {
        Self { variables: HashMap::new() }
    }

    pub fn get(&self, name: &str) -> Option<&i64> {
        return self.variables.get(name);
    }

    pub fn insert(&mut self, name: String, value: i64) {
        self.variables.insert(name, value);
    }
}

pub struct FramesVector {
    frames: Vec<Frame>,
}

impl FramesVector {
    fn new() -> Self {
        Self { frames: vec![Frame::new()] }
    }

    fn push(&mut self) {
        self.frames.push(Frame::new());
    }

    fn pop(&mut self) -> Option<Frame> {
        return self.frames.pop();
    }

    fn get(&self, name: &str) -> Option<&i64> {
        for frame in self.frames.iter().rev() {
            if let Some(value) = frame.get(name) {
                return Some(value);
            }
        }
        None
    }

    fn insert(&mut self, indentifier: String, value: i64) {
        self.frames.last_mut().unwrap().insert(indentifier, value);
    }

    fn update(&mut self, indentifier: String, value: i64) {
        for frame in self.frames.iter_mut().rev() {
            if frame.get(&indentifier).is_some() {
                frame.insert(indentifier, value);
                return;
            }
        }
        panic!("Error: Variable {} not found", indentifier);
    }
}


pub struct ASTEvaluator<'a> {
    pub last_value: Option<i64>,
    pub frames: FramesVector,
    pub global_scope: &'a GlobalScope,
}

impl <'a> ASTEvaluator <'a>{
    pub fn new(global_scope: &'a GlobalScope) -> Self {
        Self { last_value: None, frames: FramesVector::new(), global_scope }
    }
    fn evaluate_boolean_instruction<F>(&mut self, instruction: F) -> bool where F: FnOnce() -> bool {
        return instruction(); //TODO: Verify that this works
    }

    fn push_frame(&mut self) {
        self.frames.push();
    }
    fn pop_frame(&mut self) {
        self.frames.pop();
    }
}
impl <'a> ASTTraverser<'_> for ASTEvaluator<'a> {

    fn goto_while_statement(&mut self, while_statement: &ASTWhileStatement) {
        self.push_frame();
        self.goto_expression(&while_statement.condition);
        while self.last_value.unwrap() != 0 {
            self.goto_statement(&while_statement.body);
            self.goto_expression(&while_statement.condition);
        }
        self.pop_frame();
    }

    fn goto_if_statement(&mut self, if_statement: &ASTIFStatement) {
        self.push_frame();
        self.goto_expression(&if_statement.condition);
        if self.last_value.unwrap() != 0 {
            self.push_frame();
            self.goto_statement(&if_statement.then_branch);
            self.pop_frame();
        } else if let Some(else_branch) = &if_statement.else_branch {
            self.push_frame();
            self.goto_statement(&else_branch.else_statement);
            self.pop_frame();
        }
        self.pop_frame();
    }

    fn goto_block_statement(&mut self, block_statement: &ASTBlockStatement) {
        self.push_frame();
        for statement in &block_statement.statements {
            self.goto_statement(statement);
        }
        self.pop_frame();
    }

    fn goto_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.goto_expression(&let_statement.initializer);
        self.frames.insert(let_statement.identifier.span.literal.clone(), self.last_value.unwrap());
    }

    fn goto_variable_expression(&mut self, variable_expression: &VariableExpression) {
        self.last_value = Some(*self.frames.get(&variable_expression.identifier.span.literal).unwrap());
    }

    fn goto_number_expression(&mut self, number: &NumberExpression) {
        self.last_value = Some(number.number);
    }

    fn goto_error(&mut self, span: &SourceCodeSpan) {
        todo!();
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
            BinaryOperatorType::GREATER => if left > right { 1 } else { 0 },
            BinaryOperatorType::LESS => if left < right { 1 } else { 0 },
            BinaryOperatorType::GREATEREQUALS => if left >= right { 1 } else { 0 },
            BinaryOperatorType::LESSEQUALS => if left <= right { 1 } else { 0 },
            BinaryOperatorType::EQUALS => if left == right { 1 } else { 0 },
            BinaryOperatorType::NOTEQUALS => if left != right { 1 } else { 0 },
        });
    }

    fn goto_parenthesized_expression(&mut self, parenthesized_expression: &ParenthesizedExpression) {
        self.goto_expression(&parenthesized_expression.expression);
    }
}