mod syntax_tree;

use syntax_tree::lexer::Lexer;
use syntax_tree::parser::Parser;
use syntax_tree::evaluator::Evaluator;
use syntax_tree::traverser::ActualST;
fn test_lexer(i: &str) {
    let input = i;
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    
    println!();
    //let mut parser = Parser::create_from_input(input); //this also works
    let mut parser = Parser::new(tokens);
    let mut ast = ActualST::new();
    while let Some(statement) = parser.next_statement() { 
        println!("{:#?}",statement); //Pretty print
        ast.push_statement(statement);
    }
    
    let mut eval = Evaluator::new();
    ast.find(&mut eval);
    println!("Statement result: {:?}", eval.last_value.unwrap()); //print the result of 

}

fn main() {
    test_lexer("( 7 + 4 ) * 32");
}
/*
OUTPUT FOR test_lexer("( 7 + 4 ) * 32"):
    Some(352) (not uwrapped) (correct)
*/