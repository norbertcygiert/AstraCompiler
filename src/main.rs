mod syntax_tree;

fn test_lexer(expression: &str) {
    let input = expression;
    let mut lexer = syntax_tree::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    println!("{:?}", tokens); // Assuming 3 tokens: number, plus, number
}

fn main() {
    test_lexer("7 + 4 - 2 / (3 * 2)");
    println!("Hello, world!");
}
