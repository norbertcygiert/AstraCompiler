pub mod output;

use std::{cell::RefCell, rc::Rc};

use crate::syntax_tree::lexer::{Token, TokenSpan, TokenType};

#[derive(Debug)]
pub enum DiagnosticsType {
    WARNING,
    ERROR,
    NOTE,
}
#[derive(Debug)]
pub struct Diagnostic {
    pub content: String,
    pub span: TokenSpan,
    pub diagnostic_type: DiagnosticsType,
}

impl Diagnostic {
    pub fn new(content: String, span: TokenSpan, diagnostic_type: DiagnosticsType) -> Self {
        Diagnostic {
            content,
            span,
            diagnostic_type,
        }
    }
}
pub type DiagnosticsVecCell = Rc<RefCell<DiagnosticsVec>>;
#[derive(Debug)]
pub struct DiagnosticsVec {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticsVec {
    pub fn new() -> Self {
        DiagnosticsVec {
            diagnostics: vec![]
        }
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn throw_error(&mut self, content: String, span: TokenSpan) {
        let error = Diagnostic::new(content, span, DiagnosticsType::ERROR);
        self.push(error);
    }

    pub fn throw_warning(&mut self, content: String, span: TokenSpan){
        let warning = Diagnostic::new(content, span, DiagnosticsType::WARNING);
        self.push(warning);
    }

    pub fn throw_note(&mut self, content: String, span: TokenSpan){
        let note = Diagnostic::new(content, span, DiagnosticsType::NOTE);
        self.push(note);
    }

    pub fn throw_unexpected_token(&mut self, exp: &TokenType, fnd: &Token){
        self.throw_error(format!("Unexpected token! Expected: {:?}, found: {:?}", exp, fnd.kind), fnd.span.clone());
    }

    pub fn throw_expected_expression(&mut self, fnd: &Token){
        self.throw_error(format!("Expected expression! Instead found: {:?}", fnd.kind), fnd.span.clone());
    }
}
