#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )
    Dot,          // .
    Comma,        // ,
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

    Identifier, // [_a-zA-Z][_a-zA-Z0-9]*

    String(String), // \".*\"

    Unknown, // anything else
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    // TODO: make this a pointer/ref to a symbol table entry.
    pub lexeme: String,
}

impl Token {
    pub fn identifier<S: Into<String>>(s: S) -> Token {
        Token {
            kind: TokenKind::Identifier,
            lexeme: s.into(),
        }
    }
}
