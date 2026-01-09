use crate::Value;
use crate::tokenize::Token;

// parse.rs
fn parse_tokens(tokens: &[Token], index: &mut usize) -> ParseResult {
    let token = &tokens[*index];
    match token {
        Token::Null => Ok(Value::Null),
        Token::False => Ok(Value::Boolean(false)),
        Token::True => Ok(Value::Boolean(true)),
        Token::Number(number) => Ok(Value::Number(*number)),
        Token::String(string) => parse_string(string),
        Token::LeftBracket => todo!(),
        Token::LeftBrace => todo!(),
        _ => todo!(),
    }
}

fn parse_string(input: &str) -> ParseResult {
    let mut output = String::with_capacity(input.len());

    let mut is_escaping = false;
    let mut chars = input.chars();
    while let Some(next_char) = chars.next() {
        if is_escaping {
            todo!("implement");
            is_escaping = false;
        } else if next_char == '\\' {
            is_escaping = true;
        } else {
            output.push(next_char);
        }
    }

    Ok(Value::String(output))
}

type ParseResult = Result<Value, TokenParseError>;

#[derive(Debug, PartialEq)]
enum TokenParseError {}

#[cfg(test)]
mod tests {
    use crate::Value;
    use crate::tokenize::Token;

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

    #[test]
    fn parses_string_no_escapes() {
        let input = [Token::String("hello world".into())];
        let expected = Value::String("hello world".into());

        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_string_non_ascii() {
        let input = [Token::String(String::from("olá_こんにちは_नमस्ते_привіт"))];
        let expected = Value::String(String::from("olá_こんにちは_नमस्ते_привіт"));

        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_string_with_emoji() {
        let input = [Token::String(String::from("hello 💩 world"))];
        let expected = Value::String(String::from("hello 💩 world"));

        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }
}
