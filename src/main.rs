mod syntax_tree;
use syntax_tree::lexer::Lexer;
use syntax_tree::parser::Parser;
fn test_lexer(i: &str) {
    let input = i;
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    //println!("{:?}", tokens);
    println!();
    let mut parser = Parser::create_from_tokens(tokens);
    while let Some(statement) = parser.next_statement() {
        println!("{:?}", statement);
    }

}

fn main() {
    test_lexer("7454");
}
