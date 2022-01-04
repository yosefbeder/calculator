const SPECIAL_CHARACTERS: [char; 7] = ['(', ')', '+', '-', '*', '/', '^'];

#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Plus,
    Dash,
    Asterisk,
    Slash,
    Caret,
    LeftParenthese,
    RightParenthese,
    End,
}

impl Token {
    fn new(c: char) -> Result<Self, String> {
        if c == '(' {
            return Ok(Token::LeftParenthese);
        };
        if c == ')' {
            return Ok(Token::RightParenthese);
        };
        if c == '*' {
            return Ok(Token::Asterisk);
        };
        if c == '/' {
            return Ok(Token::Slash);
        };
        if c == '^' {
            return Ok(Token::Caret);
        };
        if c == '+' {
            return Ok(Token::Plus);
        };
        if c == '-' {
            return Ok(Token::Dash);
        };

        Err(format!("Failed to create a token from {}", c))
    }
}

fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
    let mut left_input = input;
    let mut tokens = vec![];

    while let Some(ch) = left_input.chars().next() {
        if ch.is_whitespace() {
            left_input = &left_input[ch.len_utf8()..];
            continue;
        }

        if ch.is_numeric() {
            let mut value = String::new();

            while let Some(ch) = left_input.chars().next() {
                if ch.is_numeric() {
                    value.push(ch);
                    left_input = &left_input[ch.len_utf8()..];
                } else {
                    break;
                }
            }

            tokens.push(Token::Number(value.parse().unwrap()));
            continue;
        }

        if SPECIAL_CHARACTERS.contains(&ch) {
            match Token::new(ch) {
                Ok(token) => {
                    tokens.push(token);
                }
                Err(err) => return Err(err),
            }
            left_input = &left_input[ch.len_utf8()..];
            continue;
        }

        return Err(format!("[tokenizer]: Unexpected character {}", ch));
    }

    tokens.push(Token::End);

    Ok(tokens)
}

pub fn calc(input: &str) {
    let tokens = tokenizer(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_tests() {
        assert_eq!(
            tokenizer("432").unwrap(),
            vec![Token::Number(432), Token::End],
            "Tokenizes numbers"
        );

        assert_eq!(
            tokenizer("()").unwrap(),
            vec![Token::LeftParenthese, Token::RightParenthese, Token::End],
            "Tokenizes parentheses"
        );

        let input = "-4 * -3";
        let output = tokenizer(input).unwrap();

        assert_eq!(output[2], Token::Asterisk, "Tokenizes binary operators");
        assert_eq!(
            output[0],
            Token::Dash,
            "Tokenizes unary operators at the start of the input"
        );
        assert_eq!(
            output[3],
            Token::Dash,
            "Tokenizes unary operators in the middle of the input"
        );

        assert_eq!(
            tokenizer("4 + 3 * 7 - (9 + 8)").unwrap(),
            vec![
                Token::Number(4),
                Token::Plus,
                Token::Number(3),
                Token::Asterisk,
                Token::Number(7),
                Token::Dash,
                Token::LeftParenthese,
                Token::Number(9),
                Token::Plus,
                Token::Number(8),
                Token::RightParenthese,
                Token::End,
            ],
            "Tokenizes a full input"
        )
    }
}
