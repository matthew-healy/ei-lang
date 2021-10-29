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
    Identifier { name: Token },
    Literal { l: Literal },
}

#[derive(Debug, PartialEq)]
enum Literal {
    String(String),
}

fn parse<'src>(mut stream: TokenStream<'src>) -> UntypedProgram {
    let mut program = UntypedProgram { stmts: Vec::new() };

    let nxt = stream.next();

    if let None = nxt {
        return program;
    }

    let nxt = nxt.unwrap();

    let stmt = match nxt.kind {
        TokenKind::String(s) => Stmt::Expr {
            e: Expr::Literal {
                l: Literal::String(s),
            },
        },
        TokenKind::Identifier => Stmt::Expr {
            e: Expr::Identifier { name: nxt },
        },
        _ => Stmt::Expr {
            e: Expr::Identifier {
                name: Token::identifier("Unknown"),
            },
        },
    };

    program.stmts.push(stmt);
    program
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::test_with_parameters;

    #[test]
    fn empty_token_stream_returns_empty_ast() {
        let ast = parse(token_stream(""));
        let expected = UntypedProgram { stmts: vec![] };
        assert_eq!(expected, ast)
    }

    #[test_with_parameters(
        [ input              , literal        ]
        [ "\"Hallo, Welt!\"" , "Hallo, Welt!" ]
        [ "\"Goodbye\""      , "Goodbye"      ]
    )]
    fn single_string_literal_returns_string_expr_stmt(raw: &str, literal: &str) {
        let ast = parse(token_stream(raw));
        let expected = UntypedProgram {
            stmts: vec![Stmt::Expr {
                e: Expr::Literal {
                    l: Literal::String(literal.to_string()),
                },
            }],
        };
        assert_eq!(expected, ast)
    }

    #[test]
    fn raw_identifier() {
        let ast = parse(token_stream("some_ident"));
        let expected = UntypedProgram {
            stmts: vec![Stmt::Expr {
                e: Expr::Identifier {
                    name: Token::identifier("some_ident"),
                },
            }],
        };
        assert_eq!(expected, ast)
    }
}
