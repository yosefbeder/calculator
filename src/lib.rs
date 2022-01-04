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
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = vec![];
    let mut current = 0;

    while current < chars.len() {
        if let Some(c) = chars.get(current) {
            if c.is_whitespace() {
                current += 1;
                continue;
            }

            if c.is_numeric() {
                let mut value = String::new();

                while let Some(c) = chars.get(current) {
                    if c.is_numeric() {
                        value.push(*c);
                        current += 1;
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Number(value.parse().unwrap()));
                continue;
            }

            if SPECIAL_CHARACTERS.contains(c) {
                match Token::new(*c) {
                    Ok(token) => {
                        tokens.push(token);
                    }
                    Err(err) => return Err(err),
                }
                current += 1;
                continue;
            }

            return Err(format!("[tokenizer]: Unexpected character {}", c));
        }
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
