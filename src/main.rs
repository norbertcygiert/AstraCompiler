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
    let mut parser = Parser::new(tokens);
    while let Some(statement) = parser.next_statement() {
        println!("{:#?}",statement); //Pretty print
    }

}

fn main() {
    test_lexer("7+4*3");
}
/*
OUTPUT FOR test_lexer("7+4*3"):

StStatement {
    kind: EXPRESSION(
        StExpression {
            kind: BINARY(
                StBinaryExpression {
                    left: StExpression {
                        kind: NUMBER(
                            StNumeralExpression {
                                value: 7,
                            },
                        ),
                    },
                    operator: StBinaryOperator {
                        kind: ADD,
                        token: Token {
                            kind: PLUS,
                            span: TokenSpan {
                                start: 1,
                                end: 2,
                                content: "+",
                            },
                        },
                    },
                    right: StExpression {
                        kind: BINARY(
                            StBinaryExpression {
                                left: StExpression {
                                    kind: NUMBER(
                                        StNumeralExpression {
                                            value: 4,
                                        },
                                    ),
                                },
                                operator: StBinaryOperator {
                                    kind: MULTIPLY,
                                    token: Token {
                                        kind: STAR,
                                        span: TokenSpan {
                                            start: 3,
                                            end: 4,
                                            content: "*",
                                        },
                                    },
                                },
                                right: StExpression {
                                    kind: NUMBER(
                                        StNumeralExpression {
                                            value: 3,
                                        },
                                    ),
                                },
                            },
                        ),
                    },
                },
            ),
        },
    ),
}
*/