// parse.rs
fn parse_tokens(tokens: &[Token], index: &mut usize) -> Result<Value, TokenParseError> {
    let token = &tokens[*index];
    match token {
        Token::Null => Ok(Value::Null),
        Token::False => Ok(Value::Boolean(false)),
        Token::True => Ok(Value::Boolean(true)),
        Token::Number(number) => todo!(),
        Token::String(string) => todo!(),
        Token::LeftBracket => todo!(),
        Token::LeftBrace => todo!(),
        _ => todo!(),
    }
}

#[derive(Debug, PartialEq)]
enum TokenParseError {}