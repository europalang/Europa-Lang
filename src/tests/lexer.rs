#[cfg(test)]
mod lexer_test {
    use std::fs;

    use crate::lexer::Lexer;

    /// Rule: Only one can ever be not None!
    struct ExpectResult {
        /// Usage: `/* expect token:` Compares based on the `format!` of each `TType`
        tokens: Option<String>,
        error: Option<Vec<String>>,
    }

    fn parse_comment(code: &str) -> ExpectResult {
        let tokens = if code.contains("/* expect token:") {
            let code = code.split_once("/* expect token:").unwrap();
            let code = code.1.trim().split_once("*/").unwrap();
            let tokens = code
                .0
                .trim()
                .split("\n")
                .map(|x| x.trim().to_string())
                .collect::<Vec<_>>()
                .join("\n");
            Some(tokens)
        } else {
            None
        };
        let error = if code.contains("/* expect error:") {
            let code = code.split_once("/* expect error:").unwrap();
            let code = code.1.trim().split_once("*/").unwrap();
            let error = code.0.trim();
            Some(error.split(" ").map(|x| x.to_owned()).collect())
        } else {
            None
        };

        assert!(tokens.is_none() || error.is_none());

        ExpectResult { tokens, error }
    }

    fn test_lexer_file(file: &str) {
        let code = fs::read_to_string(file).unwrap().replace("\r", "");

        let ExpectResult {
            tokens: expect_tokens,
            error: expect_error,
        } = parse_comment(&code);

        let mut lexer = Lexer::new(&code);
        let lex_out = lexer.init();

        if let Some(expect_tokens) = expect_tokens {
            let actual_tokens = lex_out
                .unwrap()
                .iter()
                .map(|tok| format!("{:?}", tok.ttype))
                .collect::<Vec<_>>()
                .join("\n");

            assert_eq!(expect_tokens, actual_tokens)
        } else if let Some(expect_error) = expect_error {
            let error = lex_out.unwrap_err();
            let actual_error = format!("{:?} {:?} {}", error, error.error_type, error.error);

            for snippet in &expect_error {
                assert!(actual_error.contains(snippet));
            }
        }
    }

    #[test]
    fn test_lexer() {
        let programs = fs::read_dir("test/lexer").unwrap();

        for file in programs {
            let path = file.unwrap().path().display().to_string();
            println!("---- {} ----", path);
            test_lexer_file(&path);
        }
    }
}
