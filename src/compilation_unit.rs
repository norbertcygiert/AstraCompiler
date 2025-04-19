use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::syntax_tree::{AbstractSyntaxTree, ASTLetStatement, NumberExpression, UnaryExpression, VariableExpression, ASTTraverser, lexer::{Lexer, SourceCodeSpan},evaluator::ASTEvaluator, parser::Parser};
use crate::diagnostics::{DiagnosticsVectorCell, DiagnosticsVector, output::DiagnosticsPrinter};
use crate::code::source_code::SourceCode;
struct SymbolChecker {
    symbols: HashMap<String, ()>,
    diagnostics: DiagnosticsVectorCell,
}

impl SymbolChecker {
    fn new(diagnostics: DiagnosticsVectorCell) -> Self {
        SymbolChecker {
            symbols: HashMap::new(),
            diagnostics,
        }
    }
}

impl ASTTraverser for SymbolChecker {
    fn goto_let_statement(&mut self, let_statement: &ASTLetStatement) {
        let identifier = let_statement.identifier.span.literal.clone();
        self.goto_expression(&let_statement.initializer);
        self.symbols.insert(identifier, ());
    }

    fn goto_variable_expression(&mut self, variable_expression: &VariableExpression) {
        if self.symbols.get(&variable_expression.identifier.span.literal).is_none() {

            let mut diagnostics_binding = self.diagnostics.borrow_mut();
            diagnostics_binding.report_undeclared_variable(
                &variable_expression.identifier,
            );
        }
    }

    fn goto_number_expression(&mut self, number: &NumberExpression) {  }

    fn goto_error(&mut self, span: &SourceCodeSpan) { }

    fn goto_unary_expression(&mut self, unary_expression: &UnaryExpression) { self.goto_expression(&unary_expression.operand); }
}


pub struct CompilationUnit {
    pub ast: AbstractSyntaxTree,
    pub diagnostics_vector: DiagnosticsVectorCell,
}

impl CompilationUnit {

    pub fn compile(input: &str) -> CompilationUnit {
        let text = SourceCode::new(input.to_string());
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        let diagnostics_bag: DiagnosticsVectorCell = Rc::new(RefCell::new(DiagnosticsVector::new()));
        let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
        let mut parser = Parser::new(
            tokens,
            Rc::clone(&diagnostics_bag)
        );
        while let Some(stmt) = parser.next_statement() {
            ast.add_statement(stmt);
        }
        ast.visualize();
        if Self::check_diagnostics(&text, &diagnostics_bag).is_err() {
            return Self::init_compilation_unit(ast, diagnostics_bag);
        }
        let mut symbol_checker = SymbolChecker::new(Rc::clone(&diagnostics_bag));
        ast.visit(&mut symbol_checker);
        if Self::check_diagnostics(&text, &diagnostics_bag).is_err() {
            return Self::init_compilation_unit(ast, diagnostics_bag);
        }
        Self::init_compilation_unit(ast, diagnostics_bag)

    }


    pub fn run_if_valid(&self) {
        if self.diagnostics_vector.borrow().diagnostics.len() > 0 {
            return;
        }
        self.run();
    }

    fn run(&self) {
        let mut eval = ASTEvaluator::new();
        self.ast.visit(&mut eval);
        println!("Result: {:?}", eval.last_value);
    }

    fn init_compilation_unit(ast: AbstractSyntaxTree, diagnostics_bag: DiagnosticsVectorCell) -> CompilationUnit {
        CompilationUnit {
            ast,
            diagnostics_vector: diagnostics_bag,
        }
    }

    fn check_diagnostics( code: &SourceCode, diagnostics_bag: &DiagnosticsVectorCell) -> Result<(),()> {
        let diagnostics_binding = diagnostics_bag.borrow();
        if diagnostics_binding.diagnostics.len() > 0 {
            let diagnostics_printer = DiagnosticsPrinter::new(
                &code,
                &diagnostics_binding.diagnostics
            );
            diagnostics_printer.print();
            return Err(());
        }
        Ok(())
    }


}