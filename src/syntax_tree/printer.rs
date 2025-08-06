use termion::color;

use crate::syntax_tree::*;
pub struct ASTPrinter {
    indent: usize,
    pub result: String, //Make this public for the visualizer
}

impl ASTPrinter {
    const NUMBER_COLOR: color::LightGreen = color::LightGreen;
    const TEXT_COLOR: color::LightWhite = color::LightWhite;
    const KEYWORD_COLOR: color::Blue = color::Blue;
    const VARIABLE_COLOR: color::LightBlue = color::LightBlue;
    const BOOLCOLOR: color::Green = color::Green;
    const STRINGCOLOR: color::LightCyan = color::LightCyan;

    pub fn new() -> Self {
        Self { indent: 0, result: String::new() }
    }

    fn add_space(&mut self) { self.result.push_str(" "); }

    fn add_newline(&mut self) { self.result.push_str("\n"); }

    fn add_indent(&mut self) {
        for _ in 0..self.indent {
            self.result.push_str("  ");
        }
    }

    fn add_boolean(&mut self, boolean: bool) {
        self.result.push_str(&format!("{}{}", Self::BOOLCOLOR.fg_str(), boolean));
    }

    fn add_string(&mut self, string: &str) {
        self.result.push_str(&format!("{}{}", Self::STRINGCOLOR.fg_str(), string));
    }

    
}

impl ASTTraverser<'_> for ASTPrinter {
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
        Self::statement_dispatch(self, statement);
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