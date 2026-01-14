use crate::Value;
use crate::tokenize::Token;

fn parse_tokens(tokens: &[Token], index: &mut usize) -> ParseResult {
    let token = &tokens[*index];
    match token {
        Token::Null => Ok(Value::Null),
        Token::False => Ok(Value::Boolean(false)),
        Token::True => Ok(Value::Boolean(true)),
        Token::Number(number) => Ok(Value::Number(*number)),
        Token::String(string) => parse_string(string),
        Token::LeftBracket => parse_array(tokens, index),
        Token::LeftBrace => todo!(),
        _ => todo!(),
    }
}

fn parse_array(tokens: &[Token], index: &mut usize) -> ParseResult {
    let mut array: Vec<Value> = Vec::new();
    loop {
        // consume the previous LeftBracket or Comma token
        *index += 1;
         if tokens[*index] == Token::RightBracket {
            break;
        }
        let value = parse_tokens(tokens, index)?;
        array.push(value);

        *index += 1;
        let token = &tokens[*index];
        match token {
            Token::Comma => {}
            Token::RightBracket => break,
            _ => return Err(TokenParseError::ExpectedComma),
        }
    }
    // consume the RightBracket token
    *index += 1;

    Ok(Value::Array(array))
}

fn parse_string(input: &str) -> ParseResult {
    let mut output = String::with_capacity(input.len());

    let mut is_escaping = false;
    let mut chars = input.chars();
    while let Some(next_char) = chars.next() {
        if is_escaping {
            match next_char {
                '"' => output.push('"'),
                '\\' => output.push('\\'),
                // `\b` (backspace) is a valid escape in JSON, but not Rust
                'b' => output.push('\u{8}'),
                // `\f` (formfeed) is a valid escape in JSON, but not Rust
                'f' => output.push('\u{12}'),
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                'u' => {
                    let mut sum = 0;
                    for i in 0..4 {
                        let next_char = chars.next().ok_or(TokenParseError::UnfinishedEscape)?;
                        let digit = next_char
                            .to_digit(16)
                            .ok_or(TokenParseError::InvalidHexValue)?;
                        sum += (16u32).pow(3 - i) * digit;
                    }
                    let unescaped_char =
                        char::from_u32(sum).ok_or(TokenParseError::InvalidCodePointValue)?;
                    output.push(unescaped_char);
                }
                // any other character *may* be escaped, ex. `\q` just push that letter `q`
                _ => output.push(next_char),
            }
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
enum TokenParseError {
    /// An escape sequence was started without 4 hexadecimal digits afterwards
    UnfinishedEscape,
    /// A character in an escape sequence was not valid hexadecimal
    InvalidHexValue,
    /// Invalid unicode value
    InvalidCodePointValue,

    ExpectedComma,
}

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

    #[test]
    fn parses_string_unescape_backslash() {
        let input = [Token::String(r#"hello\\world"#.into())];
        let expected = Value::String(r#"hello\world"#.into());

        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_array_one_element() {
        // [true]
        let input = [Token::LeftBracket, Token::True, Token::RightBracket];
        let expected = Value::Array(vec![Value::Boolean(true)]);

        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_array_two_elements() {
        // [null, 16]
        let input = [
            Token::LeftBracket,
            Token::Null,
            Token::Comma,
            Token::Number(16.0),
            Token::RightBracket,
        ];
        let expected = Value::Array(vec![Value::Null, Value::Number(16.0)]);

        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_empty_array() {
        // []
        let input = [Token::LeftBracket, Token::RightBracket];
        let expected = Value::Array(vec![]);

        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }
}
