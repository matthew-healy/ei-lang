#[derive(Debug, PartialEq)]
enum Token {
    LeftBrace, // {
    RightBrace, // }
}

struct TokenStream<I> {
    src: I,
}

impl <I: Iterator<Item=char>> Iterator for TokenStream<I> {
   type Item = Token;

   fn next(&mut self) -> Option<Token> {
       match self.src.next() {
           Some(c) => parse_token(c),
           None => None
       }
   }
}

fn parse_token(c: char) -> Option<Token> {
    match c {
        '{' => Some(Token::LeftBrace),
        '}' => Some(Token::RightBrace),
        _ => None
    }
}

fn token_stream<I>(src: I) -> TokenStream<I> where I: Iterator<Item=char> {
    TokenStream { src }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_source_returns_no_tokens() {
        let maybe_token = token_stream("".chars()).next();
        assert_eq!(maybe_token, None);
    }

    #[test]
    fn can_lex_single_character_tokens() {
        let tests: [(&str, Token); 2] = [
            ("{", Token::LeftBrace),
            ("}", Token::RightBrace),
        ];
        for test in &tests {
           let input = test.0;
           let expected = &test.1;

           let maybe_token = token_stream(input.chars()).next();
           match maybe_token {
               Some(token) => assert_eq!(expected, &token, "Lexed incorrect token {:?} for input {:?}.", expected, token),
               None => panic!("No token returned for input {:?}", input),
           };
        }
    }
}
