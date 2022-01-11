use ei_lexer::*;
use std::iter::Peekable;

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
    FunctionApplication { callee: Box<Expr> },
}

#[derive(Debug, PartialEq)]
enum Literal {
    String(String),
}

impl Literal {
    fn new<T: Into<Literal>>(t: T) -> Literal {
        t.into()
    }
}

impl<T: Into<String>> From<T> for Literal {
    fn from(s: T) -> Literal {
        Literal::String(s.into())
    }
}

fn parse<'src>(stream: TokenStream<'src>) -> UntypedProgram {
    let mut parser = Parser::new(stream);
    parser.parse_program()
}

struct Parser<T> {
    tokens: T,
}

impl<T: Iterator<Item = Token>> Parser<Peekable<T>> {
    fn new(tokens: T) -> Parser<Peekable<T>> {
        let tokens = tokens.peekable();
        Parser { tokens }
    }

    fn parse_program(&mut self) -> UntypedProgram {
        let mut stmts = Vec::new();

        let ps = self.parse_stmt();
        if let Some(s) = ps {
            stmts.push(s);
        }

        UntypedProgram { stmts }
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        if self.tokens.peek().is_none() {
            return None;
        }

        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        let expression = self.expression();

        self.match_single(TokenKind::SemiColon)
            .and_then(|_t| expression.map(|e| Stmt::Expr { e }))
    }

    fn expression(&mut self) -> Option<Expr> {
        match self.primary() {
            Some(e) if self.match_single(TokenKind::LeftParen).is_some() => {
                if self.match_single(TokenKind::RightParen).is_some() {
                    Some(Expr::FunctionApplication {
                        callee: Box::new(e),
                    })
                } else {
                    panic!("Unmatched parens")
                }
            }
            Some(e) => Some(e),
            None => None,
        }
    }

    fn primary(&mut self) -> Option<Expr> {
        self.tokens.next().and_then(|t| match t.kind {
            TokenKind::String(s) => Some(Expr::Literal { l: Literal::new(s) }),
            TokenKind::Identifier => Some(Expr::Identifier { name: t }),
            _ => None,
        })
    }

    fn match_single(&mut self, t: TokenKind) -> Option<Token> {
        if self.check_next(t) {
            self.tokens.next()
        } else {
            None
        }
    }

    fn check_next(&mut self, t: TokenKind) -> bool {
        self.tokens.peek().map(|nxt| nxt.kind == t).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_with_parameters::*;

    #[test]
    fn empty_token_stream_returns_empty_ast() {
        let ast = parse(token_stream(""));
        let expected = UntypedProgram { stmts: vec![] };
        assert_eq!(expected, ast)
    }

    #[test_with_parameters(
        [ input              , literal        ]
        [ "\"Hallo, Welt!\";" , "Hallo, Welt!" ]
        [ "\"Goodbye\";"      , "Goodbye"      ]
    )]
    fn single_string_literal_returns_string_expr_stmt(raw: &str, literal: &str) {
        let ast = parse(token_stream(raw));
        let expected = UntypedProgram {
            stmts: vec![Stmt::Expr {
                e: Expr::Literal {
                    l: Literal::new(literal),
                },
            }],
        };
        assert_eq!(expected, ast)
    }

    #[test]
    fn raw_identifier() {
        let ast = parse(token_stream("some_ident;"));
        let expected = UntypedProgram {
            stmts: vec![Stmt::Expr {
                e: Expr::Identifier {
                    name: Token::identifier("some_ident"),
                },
            }],
        };
        assert_eq!(expected, ast)
    }

    #[test]
    fn function_application_no_args() {
        let ast = parse(token_stream("do_something();"));
        let expected = UntypedProgram {
            stmts: vec![Stmt::Expr {
                e: Expr::FunctionApplication {
                    callee: Box::new(Expr::Identifier {
                        name: Token::identifier("do_something"),
                    }),
                },
            }],
        };
        assert_eq!(expected, ast)
    }
}
