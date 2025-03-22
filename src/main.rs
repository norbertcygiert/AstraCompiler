use syntax_tree::lexer::Lexer;
use syntax_tree::parser::Parser;
use syntax_tree::evaluator::Evaluator;
use syntax_tree::syntax_tree::ActualST;
mod syntax_tree;

fn test_lexer(i: &str) {
    let mut lexer = Lexer::new(i);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    
    println!();
    //let mut parser = Parser::create_from_input(input); //this also works
    let mut parser = Parser::new(tokens);
    let mut ast = ActualST::new();
    while let Some(statement) = parser.next_statement() {  
        ast.push_statement(statement);
    }
    println!("{:#?}", ast); //pretty-print

    let mut eval = Evaluator::new();
    ast.find(&mut eval);
    println!("Statement result: {:?}", eval.last_value);

}

fn main() {
    //test_lexer("7 - (30 + 7) * 8 - 2");
    test_lexer("10 * 10 - 10");
}
/*
OUTPUT FOR test_lexer("( 7 + 4 ) * 32"):
    352 (correct)

OUTPUT FOR test_lexer("7 - (30 + 7) * 8 / 2"):  inserting a minus instead of slash at the end results in weird statement parsing
    -141 (correct)

OUTPUT FOR test_lexer("10 * 10 - (15 - 5)"):
    10 (incorrect)

OUTPUT FOR test_lexer("10 * 10 - 10"):
    10 (incorrect)
*/