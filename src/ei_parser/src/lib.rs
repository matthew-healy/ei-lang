use ei_lexer::*;

#[derive(Debug, PartialEq)]
struct UntypedProgram {
    stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
enum Stmt {
    Expr { e: Expr },
}

#[derive(Debug, PartialEq)]
enum Expr {
    Literal { l: Literal },
}

#[derive(Debug, PartialEq)]
enum Literal {
    String(String),
}

fn parse<'src>(mut stream: TokenStream<'src>) -> UntypedProgram {
    match stream.next().map(|t| t.kind) {
        Some(TokenKind::String(s)) => UntypedProgram {
            stmts: vec![Stmt::Expr {
                e: Expr::Literal {
                    l: Literal::String(s),
                },
            }],
        },
        _ => UntypedProgram { stmts: vec![] },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::test_with_parameters;

    #[test]
    fn empty_token_stream_returns_empty_ast() {
        let stream = token_stream("");
        let ast = parse(stream);
        let expected = UntypedProgram { stmts: vec![] };
        assert_eq!(expected, ast)
    }

    #[test_with_parameters(
        [ input              , literal        ]
        [ "\"Hallo, Welt!\"" , "Hallo, Welt!" ]
        [ "\"Goodbye\""      , "Goodbye"      ]
    )]
    fn single_string_literal_returns_string_expr_stmt(raw: &str, literal: &str) {
        let stream = token_stream(raw);
        let ast = parse(stream);
        let expected = UntypedProgram {
            stmts: vec![Stmt::Expr {
                e: Expr::Literal {
                    l: Literal::String(literal.to_string()),
                },
            }],
        };
        assert_eq!(expected, ast)
    }
}
