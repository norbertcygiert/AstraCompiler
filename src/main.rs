use diagnostics::DiagnosticsVecCell;
use syntax_tree::lexer::Lexer;
use syntax_tree::parser::Parser;
use syntax_tree::evaluator::Evaluator;
use syntax_tree::syntax_tree::ActualST;
mod syntax_tree;
mod diagnostics;
mod text;
fn test_lexer(i: &str) {
    let mut lexer = Lexer::new(i);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    let diag = DiagnosticsVecCell::new();
    let mut parser = Parser::new(tokens, diag);
    let mut ast = ActualST::new();
    while let Some(statement) = parser.next_statement() {  
        ast.push_statement(statement);
    }
    //println!("{:#?}", ast);
    

    let mut eval = Evaluator::new();
    ast.find(&mut eval);
    println!("Statement result: {:?}", eval.last_value);

}

fn main() {
    test_lexer("10 * 10 - 10");
}