use crate::Value;
use crate::tokenize::Token;

// parse.rs
fn parse_tokens(tokens: &[Token], index: &mut usize) -> Result<Value, TokenParseError> {
    let token = &tokens[*index];
    match token {
        Token::Null => Ok(Value::Null),
        Token::False => Ok(Value::Boolean(false)),
        Token::True => Ok(Value::Boolean(true)),
        Token::Number(number) => Ok(Value::Number(*number)),
        Token::String(string) => todo!(),
        Token::LeftBracket => todo!(),
        Token::LeftBrace => todo!(),
        _ => todo!(),
    }
}

#[derive(Debug, PartialEq)]
enum TokenParseError {}

#[cfg(test)]
mod tests {
    use crate::tokenize::Token;
    use crate::Value;

    use super::parse_tokens;

    #[test]
    fn parses_null() {
        let input = [Token::Null];
        let expected = Value::Null;

        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_true() {
        let input = [Token::True];
        let expected = Value::Boolean(true);

        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_false() {
        let input = [Token::False];
        let expected = Value::Boolean(false);

        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_number() {
        let input = [Token::Number(10.0)];
        let expected = Value::Number(10.0);

        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }
}