use ei_lexer::*;

#[derive(Debug, PartialEq)]
struct UntypedProgram {
    stmts: Vec<()>,
}

fn parse<'src>(stream: TokenStream<'src>) -> UntypedProgram {
    UntypedProgram { stmts: vec![] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_token_stream_returns_empty_ast() {
        let stream = token_stream("");
        let ast = parse(stream);
        let expected = UntypedProgram { stmts: vec![] };
        assert_eq!(expected, ast)
    }
}
