#[cfg(test)]
mod lexer_test {
    use std::fs;

    use crate::lexer::Lexer;

    struct ExpectResult {
        tokens: String,
    }

    fn parse_comment(code: &str) -> ExpectResult {
        let code = code.split_once("/*").unwrap();
        let code = code.1.trim().split_once("*/").unwrap();
        let tokens = code
            .0
            .trim()
            .split("\n")
            .map(|x| x.trim().to_string())
            .collect::<Vec<_>>()
            .join("\n");

        ExpectResult { tokens }
    }

    #[test]
    fn test_ident() {
        let code = fs::read_to_string("test/lexer/ident.eo").unwrap();
        let ExpectResult {
            tokens: expect_tokens,
        } = parse_comment(&code);
        let mut lexer = Lexer::new(&code);
        let lex_out = lexer.init().unwrap();
        let actual_tokens = lex_out
            .iter()
            .map(|tok| format!("{:?}", tok.ttype))
            .collect::<Vec<_>>()
            .join("\n");

        println!("{actual_tokens:?}");
        assert_eq!(expect_tokens, actual_tokens);
    }
}
