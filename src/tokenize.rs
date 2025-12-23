#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizeError {
    UnfinishedLiteralValue,
}

pub fn tokenize(input: String) -> Result<Vec<Token>, TokenizeError> {
    // replace the function body
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;

    let mut tokens = Vec::new();
    while index < chars.len() {
        let token = make_token(&chars, &mut index)?;
        tokens.push(token);
        index += 1;
    }

    Ok(tokens)
}

fn make_token(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    let ch = chars[*index];
    let token = match ch {
        '[' => Token::LeftBracket,
        ']' => Token::RightBracket,
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        ',' => Token::Comma,
        ':' => Token::Colon,
        'n' => match tokenize_null(chars, index) {
            Ok(token) => token,
            Err(err) => return Err(err),
        },
        't' => todo!("implement `true` token"),
        'f' => todo!("implement `false` token"),

        _ => todo!("implement other tokens"),
    };
    Ok(token)
}


fn tokenize_null(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "null".chars() {
        if expected_char != chars[*index] {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }
    Ok(Token::Null)
}

#[derive(Debug, PartialEq)]
pub enum Token {
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `null`
    Null,
    /// `false`
    False,
    /// `true`
    True,
    /// Any number literal
    Number(f64),
    /// Key of the key/value pair or string value
    String(String),
}

#[cfg(test)]
mod tests {
    use super::{Token, tokenize};

    #[test]
    fn just_comma() {
        let input = String::from(",");
        let expected = [Token::Comma];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn all_punctuation1() {
        let input = String::from("[{]},:");
        let expected = [
            Token::LeftBracket,
            Token::LeftBrace,
            Token::RightBracket,
            Token::RightBrace,
            Token::Comma,
            Token::Colon,
        ];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn all_punctuation2() {
        let input = String::from("[{,:]},:");
        let expected = [
            Token::LeftBracket,
            Token::LeftBrace,
            Token::Comma,
            Token::Colon,
            Token::RightBracket,
            Token::RightBrace,
            Token::Comma,
            Token::Colon,
        ];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_null() {
        let input = String::from("null");
        let expected = [Token::Null];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected)
    }
}
