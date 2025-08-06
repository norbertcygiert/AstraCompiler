use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::syntax_tree::{evaluator::ASTEvaluator, lexer::{Lexer, SourceCodeSpan}, parser::Parser, ASTFunctionStatement, ASTLetStatement, ASTStatement, AbstractSyntaxTree, NumberExpression, UnaryExpression, VariableExpression};
use crate::diagnostics::{DiagnosticsVectorCell, DiagnosticsVector, output::DiagnosticsPrinter};
use crate::code::source_code::SourceCode;
use crate::syntax_tree::traverser::ASTTraverser;

pub struct GlobalScope {
    variables: HashMap<String, ()>,
    pub functions: HashMap<String, FunctionSymbol>,
}
pub struct FunctionSymbol {
    pub parameters: Vec<String>,
    pub body: ASTStatement,
}

impl GlobalScope {
    pub fn new() -> Self {
        GlobalScope {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    fn declare_variable(&mut self, identifier: String) {
        self.variables.insert(identifier, ());
    }
    
    fn lookup_variable(&self, identifier: &str) -> bool {
        return self.variables.get(identifier).is_some();
    }

    fn declare_function(&mut self, identifier: &str, function: &ASTStatement, parameters: Vec<String>) -> Result<(), ()> {
        if self.functions.contains_key(identifier) {
            return Err(());
        }
        let function = FunctionSymbol { parameters, body: function.clone() };
        self.functions.insert(identifier.to_string(), function);
        Ok(())
    }
    fn lookup_function(&self, identifier: &str) -> Option<&FunctionSymbol> {
        return self.functions.get(identifier);
    }
    
}

struct LocalScope {
    variables: HashMap<String, ()>,
}
impl LocalScope {
    fn new() -> Self {
        LocalScope {
            variables: HashMap::new(),
        }
    }

    fn declare_variable(&mut self, identifier: &str) {
        self.variables.insert(identifier.to_string(), ());
    }

    fn lookup_variable(&self, identifier: &str) -> bool {
        return self.variables.get(identifier).is_some();
    }
}

struct Scopes {
    local: Vec<LocalScope>,
    global: GlobalScope,
}

impl Scopes {
    fn new() -> Self {
        Scopes {
            local: vec![],
            global: GlobalScope::new(),
        }
    }

    fn from_global(global_scope: GlobalScope) -> Self {
        Scopes {
            local: vec![],
            global: global_scope,
        }
    }

    fn enter_scope(&mut self) {
        self.local.push(LocalScope::new());
    }

    fn exit_scope(&mut self) {
        self.local.pop();
    }

    fn declare_variable(&mut self, identifier: &str) {
        if self.is_in_local_scope() {
            self.local.last_mut().unwrap().declare_variable(identifier);
        } else {
            self.global.declare_variable(identifier.to_string());
        }
    }

    fn lookup_variable(&self, identifier: &str) -> bool {
        let inside_local = self.local.iter().rev().any(|scope| scope.lookup_variable(identifier));
        if inside_local {
            return true;
        }
        return self.global.lookup_variable(identifier);
    }

    fn lookup_function(&self, identifier: &str) -> Option<&FunctionSymbol> {
        return self.global.lookup_function(identifier);
    }

    fn is_in_local_scope(&self) -> bool {
        return !self.local.is_empty();
    }
}

struct Resolver {
    scopes: Scopes,
    diagnostics: DiagnosticsVectorCell,
}

impl Resolver {
    fn new(scopes: Scopes, diagnostics: DiagnosticsVectorCell) -> Self {
        Resolver {
            scopes: scopes,
            diagnostics: diagnostics,
        }
    }
}

struct GlobalSymbolResolver {
    global_scope: GlobalScope,
    diagnostics: DiagnosticsVectorCell,
}

impl GlobalSymbolResolver {
    fn new(diagnostics: DiagnosticsVectorCell) -> Self {
        GlobalSymbolResolver {
            global_scope: GlobalScope::new(),
            diagnostics,
        }
    }
}



impl ASTTraverser<'_> for GlobalSymbolResolver {
    fn goto_function_statement(&mut self, func_decl: &ASTFunctionStatement) {
        let params = func_decl.parameters.iter()
            .map(|param| param.identifier.span.literal.clone())
            .collect();
        let literal_span = &func_decl.identifier.span;
        match self.global_scope.declare_function(literal_span.literal.as_str(), &func_decl.body, params) {
            Ok(_) => {}
            Err(_) => {
                self.diagnostics.borrow_mut().report_function_already_declared(
                    &func_decl.identifier
                );
            }
        }
    }

    fn goto_let_statement(&mut self, let_statement: &ASTLetStatement) { }

    fn goto_variable_expression(&mut self, variable_expression: &VariableExpression) { }

    fn goto_number_expression(&mut self, number: &NumberExpression) { }

    fn goto_unary_expression(&mut self, unary_expression: &UnaryExpression) { }

    fn goto_boolean_expression(&mut self, boolean_expression: &crate::syntax_tree::BooleanExpression) {}

    fn goto_error(&mut self, span: &SourceCodeSpan) { }

}

impl ASTTraverser<'_> for Resolver {
    fn goto_function_statement(&mut self, function_statement: &ASTFunctionStatement) {
        self.scopes.enter_scope();
        for param in &function_statement.parameters {
            self.scopes.declare_variable(&param.identifier.span.literal);
        }
        self.goto_statement(&function_statement.body);
        self.scopes.exit_scope();
    }
    
    fn goto_variable_expression(&mut self, variable_expression: &VariableExpression) {
        if !self.scopes.lookup_variable(&variable_expression.identifier.span.literal) {
            self.diagnostics.borrow_mut().report_undeclared_variable(&variable_expression.identifier);
        }
    }

    fn goto_function_call_expression(&mut self, function_call_expression: &crate::syntax_tree::FunctionCallExpression) {
        let function = self.scopes.lookup_function(&function_call_expression.token.span.literal);
        match function {
            Some(func) => {
                if func.parameters.len() != function_call_expression.arguments.len() {
                    self.diagnostics.borrow_mut().report_invalid_arguments(
                        &function_call_expression.token,
                        func.parameters.len(),
                        function_call_expression.arguments.len(),
                    );
                }
            }
            None => {
                self.diagnostics.borrow_mut().report_function_not_declared(
                    &function_call_expression.token,
                );
            }
        }
        for arg in &function_call_expression.arguments {
            self.goto_expression(arg);
        }
    }

    fn goto_block_statement(&mut self, block_statement: &crate::syntax_tree::ASTBlockStatement) {
        self.scopes.enter_scope();
        for statement in &block_statement.statements {
            self.goto_statement(statement);
        }
        self.scopes.exit_scope();
    }
    
    fn goto_if_statement(&mut self, if_statement: &crate::syntax_tree::ASTIfStatement) {
        self.scopes.enter_scope();
        self.goto_expression(&if_statement.condition);
        self.goto_statement(&if_statement.then_branch);
        self.scopes.exit_scope();
        if let Some(else_branch) = &if_statement.else_branch {
            self.scopes.enter_scope();
            self.goto_statement(&else_branch.else_statement);
            self.scopes.exit_scope();
        }
    }

    fn goto_let_statement(&mut self, let_statement: &ASTLetStatement) {
        let identifier = let_statement.identifier.span.literal.clone();
        self.goto_expression(&let_statement.initializer);
        self.scopes.declare_variable(&identifier);
    }
    
    fn goto_number_expression(&mut self, number: &NumberExpression) {}
    
    fn goto_unary_expression(&mut self, unary_expression: &UnaryExpression) {}
    
    fn goto_error(&mut self, span: &SourceCodeSpan) {}

}

pub struct CompilationUnit {
    pub ast: AbstractSyntaxTree,
    pub diagnostics_vector: DiagnosticsVectorCell,
    pub global_scope: GlobalScope,
}

impl CompilationUnit {

    pub fn compile(input: &str) -> Result<CompilationUnit, DiagnosticsVectorCell> {
        let text = SourceCode::new(input.to_string());
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        print!("Tokens: {:#?}\n", tokens);
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
        Self::check_diagnostics(&text, &diagnostics_bag).map_err(|_| Rc::clone(&diagnostics_bag))?;
        let mut global_symbol_resolver = GlobalSymbolResolver::new(Rc::clone(&diagnostics_bag));
        ast.visit(&mut global_symbol_resolver);
        let global_scope = global_symbol_resolver.global_scope;
        let scopes = Scopes::from_global(global_scope);
        let mut resolver = Resolver::new(scopes, Rc::clone(&diagnostics_bag));
        ast.visit(&mut resolver);
        Self::check_diagnostics(&text, &diagnostics_bag).map_err(|_| Rc::clone(&diagnostics_bag))?;
        Ok(CompilationUnit {
            ast, diagnostics_vector: diagnostics_bag, global_scope: resolver.scopes.global,
        })

    }


    pub fn run_if_valid(&self) {
        if self.diagnostics_vector.borrow().diagnostics.len() > 0 {
            return;
        }
        self.run();
    }

    pub fn run(&self) {
        let mut eval = ASTEvaluator::new(&self.global_scope);
        let main_function = self.global_scope.lookup_function("main");
        if let Some(func) = main_function {
            eval.goto_statement(&func.body);
        }
        else {
            self.ast.visit(&mut eval);
        }
        println!("Result: {:?}", eval.last_value);
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