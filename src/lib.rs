#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    BinaryOperator(char),
    UnaryOperator(char),
    Parentheses(char),
    End,
}

fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = vec![];
    let mut current = 0;

    while current < chars.len() {
        if let Some(c) = chars.get(current) {
            // whitespaces
            if c.is_whitespace() {
                current += 1;
                continue;
            }

            // numbers
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

            // parentheses
            if *c == '(' || *c == ')' {
                tokens.push(Token::Parentheses(*c));
                current += 1;
                continue;
            }

            // - operator
            if *c == '-' {
                match tokens.iter().last() {
                    Some(value) => match value {
                        Token::Number(_) => tokens.push(Token::BinaryOperator(*c)),
                        _ => tokens.push(Token::UnaryOperator(*c)),
                    },
                    None => tokens.push(Token::UnaryOperator(*c)),
                }
                current += 1;
                continue;
            }

            // binary operators
            if ['+', '*', '/', '^'].contains(c) {
                tokens.push(Token::BinaryOperator(*c));
                current += 1;
                continue;
            }

            return Err(format!("[tokenizer]: Unexpected character {}", c));
        }
    }

    tokens.push(Token::End);

    Ok(tokens)
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
            vec![Token::Parentheses('('), Token::Parentheses(')'), Token::End],
            "Tokenizes parentheses"
        );

        let input = "-4 * -3";
        let output = tokenizer(input).unwrap();

        assert_eq!(
            output[2],
            Token::BinaryOperator('*'),
            "Tokenizes binary operators"
        );
        assert_eq!(
            output[0],
            Token::UnaryOperator('-'),
            "Tokenizes unary operators at the start of the input"
        );
        assert_eq!(
            output[3],
            Token::UnaryOperator('-'),
            "Tokenizes unary operators in the middle of the input"
        );

        assert_eq!(
            tokenizer("4 + 3 * 7 - (9 + 8)").unwrap(),
            vec![
                Token::Number(4),
                Token::BinaryOperator('+'),
                Token::Number(3),
                Token::BinaryOperator('*'),
                Token::Number(7),
                Token::BinaryOperator('-'),
                Token::Parentheses('('),
                Token::Number(9),
                Token::BinaryOperator('+'),
                Token::Number(8),
                Token::Parentheses(')'),
                Token::End,
            ],
            "Tokenizes a full input"
        )
    }
}
