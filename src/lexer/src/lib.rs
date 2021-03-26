#[derive(Debug, PartialEq)]
enum TokenKind {
    LeftBrace,  // {
    RightBrace, // }
    LeftParen,  // (
    RightParen, // )
    Dot,        // .
    Colon,      // :
    SemiColon,  // ;
    Bang,       // !
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Equal,      // =
    Greater,    // >
    Less,       // <
}

struct TokenStream<I> {
    src: I,
}

impl <I: Iterator<Item=char>> Iterator for TokenStream<I> {
   type Item = TokenKind;

   fn next(&mut self) -> Option<TokenKind> {
       match self.src.next() {
           Some(c) => parse_token(c),
           None => None
       }
   }
}

fn parse_token(c: char) -> Option<TokenKind> {
    use TokenKind::*;
    match c {
        '{' => Some(LeftBrace),
        '}' => Some(RightBrace),
        '(' => Some(LeftParen),
        ')' => Some(RightParen),
        '.' => Some(Dot),
        ':' => Some(Colon),
        ';' => Some(SemiColon),
        '!' => Some(Bang),
        '+' => Some(Plus),
        '-' => Some(Minus),
        '/' => Some(Slash),
        '*' => Some(Star),
        '=' => Some(Equal),
        '>' => Some(Greater),
        '<' => Some(Less),
        _ => None
    }
}

fn token_stream<I>(src: I) -> TokenStream<I> where I: Iterator<Item=char> {
    TokenStream { src }
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
        [ input, expected              ]
        [ "{"  , TokenKind::LeftBrace  ]
        [ "}"  , TokenKind::RightBrace ]
        [ "("  , TokenKind::LeftParen  ]
        [ ")"  , TokenKind::RightParen ]
        [ "."  , TokenKind::Dot        ]
        [ ":"  , TokenKind::Colon      ]
        [ ";"  , TokenKind::SemiColon  ]
        [ "!"  , TokenKind::Bang       ]
        [ "+"  , TokenKind::Plus       ]
        [ "-"  , TokenKind::Minus      ]
        [ "*"  , TokenKind::Star       ]
        [ "/"  , TokenKind::Slash      ]
        [ "="  , TokenKind::Equal      ]
        [ ">"  , TokenKind::Greater    ]
        [ "<"  , TokenKind::Less       ]
    )]
    fn can_lex_single_char_tokens(input: &str, expected: TokenKind) {
        let maybe_token = token_stream(input.chars()).next();
        match maybe_token {
            Some(token) => assert_eq!(expected, token, "Lexed incorrect token {:?} for input {:?}.", expected, input),
            None => panic!("No token returned for input {:?}", input),
        };
    }
}
