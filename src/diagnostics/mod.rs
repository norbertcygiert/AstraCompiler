use std::cell::RefCell;
use std::rc::Rc;

use crate::syntax_tree::lexer::{SourceCodeSpan, Token, TokenType};

pub mod output;

#[derive(Clone, Copy,Debug)]
pub enum DiagnosticKind {
    ERROR,
    WARNING,
    INFO,
}


#[derive(Clone,Debug)]
pub struct Diagnostic {
    pub message: String,
    pub span: SourceCodeSpan,
    pub kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn new(message: String, span: SourceCodeSpan, kind: DiagnosticKind) -> Self {
        Diagnostic { message, span, kind: kind }
    }
}

pub type DiagnosticsVectorCell = Rc<RefCell<DiagnosticsVector>>;

#[derive(Debug)]
pub struct DiagnosticsVector {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticsVector {
    pub fn new() -> Self {
        DiagnosticsVector { diagnostics: vec![] }
    }

    pub fn report_error(&mut self, message: String, span: SourceCodeSpan) {
        let error = Diagnostic::new(message, span, DiagnosticKind::ERROR);
        self.diagnostics.push(error);
    }

    pub fn report_warning(&mut self, message: String, span: SourceCodeSpan) {
        let warning = Diagnostic::new(message, span, DiagnosticKind::WARNING);
        self.diagnostics.push(warning);
    }

    pub fn report_unexpected_token(&mut self, expected: &TokenType, token: &Token) {
        self.report_error(format!("Expected <{}>, found <{}>", expected, token.kind), token.span.clone());
    }
    pub fn report_expected_expression(&mut self, token: &Token) {
        self.report_error(format!("Expected expression, found <{}>", token.kind), token.span.clone());
    }

    pub fn report_undeclared_variable(&mut self, token: &Token) {
        self.report_error(format!("Undeclared variable '{}'", token.span.literal), token.span.clone());
    }

    pub fn report_function_already_declared(&mut self, token: &Token) {
        self.report_error(format!("Function '{}' was already declared", token.span.literal), token.span.clone());
    }

    pub fn report_function_not_declared(&mut self, token: &Token) {
        self.report_error(format!("Function '{}' was not declared", token.span.literal), token.span.clone());
    }

    pub fn report_invalid_arguments(&mut self, token: &Token, expected: usize, found: usize) {
        self.report_error(format!("Invalid arguments for function '{}', expected {} arguments, found {}", 
        token.span.literal, expected, found), token.span.clone());
    }

}

#[cfg(test)]
mod test {
    use crate::syntax_tree::lexer::SourceCodeSpan;
    use crate::compilation_unit::CompilationUnit;
    use crate::diagnostics::{Diagnostic, DiagnosticKind};

    struct DiagnosticsVerifier {
        actual: Vec<Diagnostic>,
        expected: Vec<Diagnostic>,
    }

    impl DiagnosticsVerifier {
        pub fn new(input: &str, messages: Vec<&str>) -> Self {
            let messages_length = messages.len();
            let expected = Self::parse_input(input, messages);
            assert_eq!(expected.len(), messages_length, "Expected {} diagnostic messages, found {}", messages_length, expected.len());
            let actual = Self::compile(input);
            return Self { expected, actual };
        }

        fn compile(input: &str) -> Vec<Diagnostic> {
            let raw = Self::get_raw_text(input);
            let compilation_unit = CompilationUnit::compile(&raw);
            return match compilation_unit {
                Ok(_) => {
                    vec![]
                }
                Err(e) => {
                    e.borrow().diagnostics.clone()
                }
            }
        }

        fn get_raw_text(input: &str) -> String {
           return input.replace("«", "").replace("»", "");
        }

        fn parse_input(input: &str, messages: Vec<&str>) -> Vec<Diagnostic> {
            let raw_text = Self::get_raw_text(input);
            let mut start_index_stack = vec![];

            let mut current_position: usize = 0;
            let mut diagnostics = vec![];
            for c in input.chars() {
                match c {
                    '«' => {
                        start_index_stack.push(current_position);
                    }
                    '»' => {
                        let start_index = start_index_stack.pop().unwrap();
                        let end_index = current_position;
                        let literal = &raw_text[start_index..end_index];
                        let span = SourceCodeSpan::new(start_index, end_index, literal.to_string());
                        let message = messages[diagnostics.len()].to_string();
                        let diagnostic = Diagnostic::new(message, span, DiagnosticKind::ERROR);
                        diagnostics.push(diagnostic);
                    }
                    _ => {
                        current_position += 1;
                    }
                };
            }

            return diagnostics;
        }

        fn verify(&self) {
            assert_eq!(self.actual.len(), self.expected.len(), "Expected {} diagnostics, found {}", self.expected.len(), self.actual.len());
            for (actual, expected) in self.actual.iter().zip(self.expected.iter()) {
                assert_eq!(actual.message, expected.message, "Expected message '{}', found '{}'", expected.message, actual.message);
                assert_eq!(actual.span.start, expected.span.start, "Expected start index {}, found {}", expected.span.start, actual.span.start);
                assert_eq!(actual.span.end, expected.span.end, "Expected end index {}, found {}", expected.span.end, actual.span.end);
                assert_eq!(actual.span.literal, expected.span.literal, "Expected literal '{}', found '{}'", expected.span.literal, actual.span.literal);
            }

        }
    }

    #[test]
    fn should_report_undeclared_variable() {
        let input = "let a = «b»";
        let expected = vec![
            "Undeclared variable 'b'"
        ];

        let verifier = DiagnosticsVerifier::new(input, expected);
        verifier.verify();
    }

    #[test]
    fn should_report_expected_expression() {
        let input = "let a = «+»";
        let expected = vec![
            "Expected expression, found <+>"
        ];

        let verifier = DiagnosticsVerifier::new(input, expected);
        verifier.verify();
    }

    #[test]
    fn should_report_invalid_token() {
        let input = "let a = 8 «@» 2";
        let expected = vec![
            "Expected expression, found <INVALID>"
        ];

        let verifier = DiagnosticsVerifier::new(input, expected);
        verifier.verify();
    }

}