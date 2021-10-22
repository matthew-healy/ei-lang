use std::{iter::Peekable, str::Chars};

use phf::phf_map;

static KEYWORDS: phf::Map<&str, TokenKind> = phf_map! {
    "let"       => TokenKind::Let,
    "mut"       => TokenKind::Mut,
    "fn"        => TokenKind::Fn,
    "enum"      => TokenKind::Enum,
    "record"    => TokenKind::Record,
    "interface" => TokenKind::Interface,
    "impl"      => TokenKind::Impl,
    "check"     => TokenKind::Check,
    "match"     => TokenKind::Match,
};

#[derive(Clone, Debug, PartialEq)]
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

    Let,       // let
    Mut,       // mut
    Fn,        // fn
    Enum,      // enum
    Record,    // record
    Interface, // interface
    Impl,      // impl
    Check,     // check
    Match,     // match

    Identifier, // [a-zA-Z][_a-zA-Z0-9]*
}

#[derive(Debug, PartialEq)]
struct Token<'src> {
    kind: TokenKind,
    lexeme: &'src str,
}

fn token_stream<'src>(src: &'src str) -> TokenStream<'src> {
    TokenStream {
        raw: src,
        src: src.chars().peekable(),
        current_token_size: 0,
        current_token_start: 0,
    }
}

struct TokenStream<'src> {
    raw: &'src str,
    src: Peekable<Chars<'src>>,
    current_token_size: usize,
    current_token_start: usize,
}

impl <'src> Iterator for TokenStream<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token_kind()
            .map(|kind| Token { kind, lexeme: self.lexeme() })
    }
}

impl <'src> TokenStream<'src> {
    fn next_token_kind(&mut self) -> Option<TokenKind> {
        use TokenKind::*;

        self.current_token_start = self.current_token_start + self.current_token_size;
        self.current_token_size = 0;

        let next = self.src.next();
        self.current_token_size += 1;

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
            Some(c) if can_start_identifier(c) => {
                self.advance_until(|c| !can_be_used_in_identifier(c));
                let kind = KEYWORDS.get(self.lexeme())
                    .cloned()
                    .unwrap_or(Identifier);
                Some(kind)
            },
            _ => None
        }
    }

    fn consume(&mut self, c: char) -> bool {
        if self.src.peek() == Some(&c) {
           self.src.next();
           self.current_token_size += 1;
           true
        } else { false }
    }

    fn lexeme(&self) -> &'src str {
        let token_end = self.current_token_start + self.current_token_size;
        &self.raw[self.current_token_start..token_end]
    }

    fn advance_until(
        &mut self,
        should_stop: impl Fn(char) -> bool
    ) {
        let is_done = |nxt: Option<&char>| nxt.is_none() || should_stop(*nxt.unwrap());
        while !is_done(self.src.peek()) {
            self.src.next();
            self.current_token_size += 1;
        }
    }
}

fn can_be_used_in_identifier(c: char) -> bool {
    can_start_identifier(c) || c.is_digit(10)
}

fn can_start_identifier(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

#[cfg(test)]
mod tests {
    use test_utils::test_with_parameters;
    use super::*;

    #[test]
    fn empty_source_returns_no_tokens() {
        let maybe_token = token_stream("").next();
        assert_eq!(maybe_token, None);
    }

    #[test_with_parameters(
        [ input,       expected                ]
        [ "{"        , TokenKind::LeftBrace    ]
        [ "}"        , TokenKind::RightBrace   ]
        [ "("        , TokenKind::LeftParen    ]
        [ ")"        , TokenKind::RightParen   ]
        [ "."        , TokenKind::Dot          ]
        [ ":"        , TokenKind::Colon        ]
        [ ";"        , TokenKind::SemiColon    ]
        [ "!"        , TokenKind::Bang         ]
        [ "+"        , TokenKind::Plus         ]
        [ "-"        , TokenKind::Minus        ]
        [ "*"        , TokenKind::Star         ]
        [ "/"        , TokenKind::Slash        ]
        [ "="        , TokenKind::Equal        ]
        [ ">"        , TokenKind::Greater      ]
        [ "<"        , TokenKind::Less         ]
        [ "->"       , TokenKind::RightArrow   ]
        [ "<="       , TokenKind::LessEqual    ]
        [ ">="       , TokenKind::GreaterEqual ]
        [ "=="       , TokenKind::EqualEqual   ]
        [ "!="       , TokenKind::BangEqual    ]
        [ "&&"       , TokenKind::And          ]
        [ "||"       , TokenKind::Or           ]
        [ "let"      , TokenKind::Let          ]
        [ "mut"      , TokenKind::Mut          ]
        [ "fn"       , TokenKind::Fn           ]
        [ "enum"     , TokenKind::Enum         ]
        [ "record"   , TokenKind::Record       ]
        [ "interface", TokenKind::Interface    ]
        [ "impl"     , TokenKind::Impl         ]
        [ "check"    , TokenKind::Check        ]
        [ "match"    , TokenKind::Match        ]
    )]
    fn can_lex_static_tokens(input: &str, expected: TokenKind) {
        let maybe_token = token_stream(input).next();
        match maybe_token {
            Some(token) => assert_eq!(
                Token { kind: expected, lexeme: input }, token,
                "Lexed incorrect token {:?} for input {:?}.", token.kind, input
            ),
            None => panic!("No token returned for input {:?}", input),
        };
    }

    #[test_with_parameters(
        [ input  , expected_ident ]
        [ "a"    , "a"            ]
        [ "eggs" , "eggs"         ]
        [ "_name", "_name"        ]
        [ "d1m12", "d1m12"        ]
    )]
    fn captures_identifier_lexemes(input: &str, expected: &str) {
        let maybe_token = token_stream(input).next();
        match maybe_token {
            Some(token) => assert_eq!(
                ident_token(expected), token,
                "Lexed incorrect token {:?} for input {:?}", expected, input
            ),
            None => panic!("No token returned for input {:?}", input),
        }
    }

    fn ident_token(lexeme: &str) -> Token {
        Token { kind: TokenKind::Identifier, lexeme }
    }
}
