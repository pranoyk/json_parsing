pub fn tokenize(input: String) -> Vec<Token> {
    // replace the function body
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;

    let mut tokens = Vec::new();
    while index < chars.len() {
        let token = make_token(chars[index]);
        tokens.push(token);
        index += 1;
    }

    tokens
}

fn make_token(ch: char) -> Token {
    match ch {
        '[' => Token::LeftBracket,
        ']' => Token::RightBracket,
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        ',' => Token::Comma,
        ':' => Token::Colon,

        _ => todo!("implement other tokens"),
    }
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

        let actual = tokenize(input);

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

        let actual = tokenize(input);

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

        let actual = tokenize(input);

        assert_eq!(actual, expected);
    }
}
