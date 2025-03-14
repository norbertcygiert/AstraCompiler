mod syntax_tree;
fn main() {
    let input = "7 + 5";
    let mut lexer = syntax_tree::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token(){
        tokens.push(token)
    }
    println!("{:?}", tokens);
}
