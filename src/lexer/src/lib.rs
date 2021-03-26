use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum TokenKind {
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )
    Dot,          // .
    Colon,        // :
    SemiColon,    // ;
    Bang,         // !
    BangEqual,    // !=
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    Less,         // <
    LessEqual,    // <=
    GreaterEqual, // >=
    RightArrow,   // ->
    And,          // &&
    Or,           // ||
}

struct TokenStream<I: Iterator<Item=char>> {
    src: Peekable<I>,
}

impl <I: Iterator<Item=char>> Iterator for TokenStream<I> {
    type Item = TokenKind;

    fn next(&mut self) -> Option<TokenKind> {
        self.next_token_kind()
    }
}

impl <I: Iterator<Item=char>> TokenStream<I> {
    fn next_token_kind(&mut self) -> Option<TokenKind> {
        use TokenKind::*;
        let next = self.src.next();
        match next {
            Some('{') => Some(LeftBrace),
            Some('}') => Some(RightBrace),
            Some('(') => Some(LeftParen),
            Some(')') => Some(RightParen),
            Some('.') => Some(Dot),
            Some(':') => Some(Colon),
            Some(';') => Some(SemiColon),
            Some('!') if self.consume('=') => Some(BangEqual),
            Some('!') => Some(Bang),
            Some('+') => Some(Plus),
            Some('-') if self.consume('>') => Some(RightArrow),
            Some('-') => Some(Minus),
            Some('/') => Some(Slash),
            Some('*') => Some(Star),
            Some('=') if self.consume('=') => Some(EqualEqual),
            Some('=') => Some(Equal),
            Some('>') if self.consume('=') => Some(GreaterEqual),
            Some('>') => Some(Greater),
            Some('<') if self.consume('=') => Some(LessEqual),
            Some('<') => Some(Less),
            Some('&') if self.consume('&') => Some(And),
            Some('|') if self.consume('|') => Some(Or),
            _ => None
        }
    }

    fn consume(&mut self, c: char) -> bool {
        if self.src.peek() == Some(&c) {
           self.src.next();
           true
        } else { false }
    }
}

fn token_stream<I: Iterator<Item=char>>(src: I) -> TokenStream<I> {
    TokenStream { src: src.peekable() }
}

#[cfg(test)]
mod tests {
    use test_utils::test_with_parameters;
    use super::*;

    #[test]
    fn empty_source_returns_no_tokens() {
        let maybe_token = token_stream("".chars()).next();
        assert_eq!(maybe_token, None);
    }

    #[test_with_parameters(
        [ input, expected                ]
        [ "{"  , TokenKind::LeftBrace    ]
        [ "}"  , TokenKind::RightBrace   ]
        [ "("  , TokenKind::LeftParen    ]
        [ ")"  , TokenKind::RightParen   ]
        [ "."  , TokenKind::Dot          ]
        [ ":"  , TokenKind::Colon        ]
        [ ";"  , TokenKind::SemiColon    ]
        [ "!"  , TokenKind::Bang         ]
        [ "+"  , TokenKind::Plus         ]
        [ "-"  , TokenKind::Minus        ]
        [ "*"  , TokenKind::Star         ]
        [ "/"  , TokenKind::Slash        ]
        [ "="  , TokenKind::Equal        ]
        [ ">"  , TokenKind::Greater      ]
        [ "<"  , TokenKind::Less         ]
        [ "->" , TokenKind::RightArrow   ]
        [ "<=" , TokenKind::LessEqual    ]
        [ ">=" , TokenKind::GreaterEqual ]
        [ "==" , TokenKind::EqualEqual   ]
        [ "!=" , TokenKind::BangEqual    ]
        [ "&&" , TokenKind::And          ]
        [ "||" , TokenKind::Or           ]
    )]
    fn can_lex_static_tokens(input: &str, expected: TokenKind) {
        let maybe_token = token_stream(input.chars()).next();
        match maybe_token {
            Some(token) => assert_eq!(
                expected, token,
                "Lexed incorrect token {:?} for input {:?}.", expected, input
            ),
            None => panic!("No token returned for input {:?}", input),
        };
    }
}
