use phf::phf_map;
use std::{iter::Peekable, str::Chars};
use token::*;

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

pub fn token_stream<'src>(src: &'src str) -> TokenStream<'src> {
    TokenStream {
        raw: src,
        src: src.chars().peekable(),
        current_token_size: 0,
        current_token_start: 0,
    }
}

pub struct TokenStream<'src> {
    raw: &'src str,
    src: Peekable<Chars<'src>>,
    current_token_size: usize,
    current_token_start: usize,
}

impl<'src> Iterator for TokenStream<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token_kind().map(|kind| Token {
            kind,
            lexeme: self.lexeme().to_string(),
        })
    }
}

impl<'src> TokenStream<'src> {
    fn next_token_kind(&mut self) -> Option<TokenKind> {
        self.prepare_for_next_token();
        self.skip_whitespace();

        let next = self.src.next();
        self.current_token_size += 1;

        next.map(|nxt| {
            use TokenKind::*;

            match nxt {
                '{' => LeftBrace,
                '}' => RightBrace,
                '(' => LeftParen,
                ')' => RightParen,
                '.' => Dot,
                ',' => Comma,
                ':' => Colon,
                ';' => SemiColon,
                '!' if self.consume('=') => BangEqual,
                '!' => Bang,
                '+' => Plus,
                '-' if self.consume('>') => RightArrow,
                '-' => Minus,
                '/' => Slash,
                '*' => Star,
                '=' if self.consume('=') => EqualEqual,
                '=' => Equal,
                '>' if self.consume('=') => GreaterEqual,
                '>' => Greater,
                '<' if self.consume('=') => LessEqual,
                '<' => Less,
                '&' if self.consume('&') => And,
                '|' if self.consume('|') => Or,
                '"' => self.consume_string(),
                c if can_start_identifier(c) => self.consume_keyword_or_identifier(),
                _ => Unknown,
            }
        })
    }

    fn prepare_for_next_token(&mut self) {
        self.current_token_start = self.current_token_start + self.current_token_size;
        self.current_token_size = 0;
    }

    // Assumes that we've read a complete token & wish to skip
    // all whitespace until the next token. As such it bumps
    // current_token_start on each skipped char.
    fn skip_whitespace(&mut self) {
        while self.src.peek().map_or(false, |c| c.is_whitespace()) {
            self.src.next();
            self.current_token_start += 1;
        }
    }

    // Assumes we have already read a '"' and then
    // keeps reading until it finds another '"'.
    fn consume_string(&mut self) -> TokenKind {
        self.consume_until_match('"');
        self.consume('"');
        TokenKind::String(self.lexeme().trim_matches('"').to_string())
    }

    fn consume_keyword_or_identifier(&mut self) -> TokenKind {
        self.consume_until(cannot_be_used_in_identifier);
        KEYWORDS
            .get(self.lexeme())
            .cloned()
            .unwrap_or(TokenKind::Identifier)
    }

    fn consume(&mut self, c: char) -> bool {
        if self.src.peek() == Some(&c) {
            self.src.next();
            self.current_token_size += 1;
            true
        } else {
            false
        }
    }

    fn lexeme(&self) -> &'src str {
        let token_end = self.current_token_start + self.current_token_size;
        &self.raw[self.current_token_start..token_end]
    }

    fn consume_until(&mut self, should_stop: impl Fn(char) -> bool) {
        let is_done = |nxt: Option<&char>| nxt.is_none() || should_stop(*nxt.unwrap());
        while !is_done(self.src.peek()) {
            self.src.next();
            self.current_token_size += 1;
        }
    }

    fn consume_until_match(&mut self, sought: char) {
        self.consume_until(|c| c == sought)
    }
}

fn cannot_be_used_in_identifier(c: char) -> bool {
    !(can_start_identifier(c) || c.is_digit(10))
}

fn can_start_identifier(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_with_parameters::*;

    #[test]
    fn empty_source_returns_no_tokens() {
        let maybe_token = token_stream("").next();
        assert_eq!(maybe_token, None);
    }

    #[test]
    fn whitespace_is_skipped() {
        use TokenKind::*;
        let input = "{    + \n  }\n   -";
        let tokens = token_stream(input).map(|t| t.kind).collect::<Vec<_>>();
        let expected = vec![LeftBrace, Plus, RightBrace, Minus];
        assert_eq!(expected, tokens);
    }

    #[test_with_parameters(
        [ input,       expected                ]
        [ "{"        , TokenKind::LeftBrace    ]
        [ "}"        , TokenKind::RightBrace   ]
        [ "("        , TokenKind::LeftParen    ]
        [ ")"        , TokenKind::RightParen   ]
        [ "."        , TokenKind::Dot          ]
        [ ","        , TokenKind::Comma        ]
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
            Some(token) => util::assert_eq_tokens(
                &Token {
                    kind: expected,
                    lexeme: input.to_string(),
                },
                &token,
                input,
            ),
            None => panic!("No token returned for input {:?}", input),
        };
    }

    #[test_with_parameters(
        [ input        , expected_ident ]
        [ "a"          , "a"            ]
        [ "eggs"       , "eggs"         ]
        [ "_name"      , "_name"        ]
        [ "d1m12"      , "d1m12"        ]
        [ "hello_world", "hello_world"  ]
    )]
    fn captures_identifier_lexemes(input: &str, expected: &str) {
        let maybe_token = token_stream(input).next();
        match maybe_token {
            Some(token) => util::assert_eq_tokens(&util::ident_token(expected), &token, input),
            None => panic!("No token returned for input {:?}", input),
        }
    }

    #[test_with_parameters(
        [ input        , expected_literal ]
        [ "\"\""       , ""               ]
        [ "\"a\""      , "a"              ]
        [ "\"abacus\"" , "abacus"         ]
    )]
    fn can_lex_string_literals(input: &str, expected: &str) {
        let maybe_token = token_stream(input).next();
        match maybe_token {
            Some(token) => {
                util::assert_eq_tokens(&util::string_token(expected, input), &token, input)
            }
            None => panic!("No token returned for input {:?}", input),
        }
    }

    #[test]
    fn correctly_skips_whitespace() {
        let input = "\"b\", \"c\"";
        let tokens: Vec<Token> = token_stream(input).collect();
        let expected = vec![
            Token {
                kind: TokenKind::String("b".into()),
                lexeme: "\"b\"".into(),
            },
            Token {
                kind: TokenKind::Comma,
                lexeme: ",".into(),
            },
            Token {
                kind: TokenKind::String("c".into()),
                lexeme: "\"c\"".into(),
            },
        ];
        assert_eq!(expected, tokens);
    }

    mod util {
        use super::super::*;

        pub(crate) fn assert_eq_tokens(expected: &Token, actual: &Token, input: &str) {
            assert_eq!(
                expected, actual,
                "Lexed incorrect token {:?} for input {:?}. Expected: {:?}.",
                actual, input, expected
            )
        }

        pub(crate) fn ident_token(lexeme: &str) -> Token {
            Token {
                kind: TokenKind::Identifier,
                lexeme: lexeme.to_string(),
            }
        }

        pub(crate) fn string_token<S: Into<String>>(value: S, lexeme: &str) -> Token {
            Token {
                kind: TokenKind::String(value.into()),
                lexeme: lexeme.to_string(),
            }
        }
    }
}
