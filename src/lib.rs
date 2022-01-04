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

/*
    GRAMMAR
        E -> T {("+" | "-") T}
        T -> F {("*" | "/") F}
        F -> P ["^" F]
        P -> v | "(" E ")" | "-" P
*/

// each one is supposed to return the node it parsed along with the rest of the tokens (Node, &[Token])

#[derive(Debug, PartialEq)]
enum BinaryOperator {
    Addition,
    Subtraction,
    Division,
    Multiplication,
    Exponentiation,
}

#[derive(Debug, PartialEq)]
enum UnaryOperator {
    Negative,
}

#[derive(Debug, PartialEq)]
enum Node {
    Binary(BinaryOperator, Box<Node>, Box<Node>),
    Unary(UnaryOperator, Box<Node>),
    Single(i32),
}

impl Node {
    fn calc(&self) -> i32 {
        match self {
            Node::Single(n) => return *n,
            Node::Unary(op, node) => match op {
                UnaryOperator::Negative => {
                    return node.calc() * -1;
                }
            },
            Node::Binary(op, node0, node1) => match op {
                BinaryOperator::Addition => {
                    return node0.calc() + node1.calc();
                }
                BinaryOperator::Subtraction => {
                    return node0.calc() - node1.calc();
                }
                BinaryOperator::Division => {
                    return node0.calc() / node1.calc();
                }
                BinaryOperator::Multiplication => {
                    return node0.calc() * node1.calc();
                }
                BinaryOperator::Exponentiation => {
                    return node0.calc().pow(node1.calc().try_into().unwrap());
                }
            },
        }
    }
}

fn parse_E(tokens: &[Token]) -> Result<(Node, &[Token]), &[Token]> {
    let (t0, next_tokens) = parse_T(tokens)?;
    let mut last_tokens = next_tokens;
    let mut t = t0;

    while let Some(token) = last_tokens.iter().next() {
        match token {
            Token::Plus => {
                let (t1, next_tokens) = parse_T(&last_tokens[1..])?;
                last_tokens = next_tokens;

                t = Node::Binary(BinaryOperator::Addition, Box::new(t), Box::new(t1));
            }
            Token::Dash => {
                let (t1, next_tokens) = parse_T(&last_tokens[1..])?;
                last_tokens = next_tokens;

                t = Node::Binary(BinaryOperator::Subtraction, Box::new(t), Box::new(t1));
            }
            _ => break,
        }
    }

    Ok((t, last_tokens))
}

fn parse_T(tokens: &[Token]) -> Result<(Node, &[Token]), &[Token]> {
    let (t0, next_tokens) = parse_F(tokens)?;
    let mut last_tokens = next_tokens;
    let mut t = t0;

    while let Some(token) = last_tokens.iter().next() {
        match token {
            Token::Asterisk => {
                let (t1, next_tokens) = parse_F(&last_tokens[1..])?;
                last_tokens = next_tokens;

                t = Node::Binary(BinaryOperator::Multiplication, Box::new(t), Box::new(t1));
            }
            Token::Slash => {
                let (t1, next_tokens) = parse_F(&last_tokens[1..])?;
                last_tokens = next_tokens;

                t = Node::Binary(BinaryOperator::Division, Box::new(t), Box::new(t1));
            }
            _ => break,
        }
    }

    Ok((t, last_tokens))
}

fn parse_F(tokens: &[Token]) -> Result<(Node, &[Token]), &[Token]> {
    let (t0, next_tokens) = parse_P(tokens)?;

    if let Some(Token::Caret) = next_tokens.iter().next() {
        let (t1, next_tokens) = parse_F(&next_tokens[1..])?;
        Ok((
            Node::Binary(BinaryOperator::Exponentiation, Box::new(t0), Box::new(t1)),
            next_tokens,
        ))
    } else {
        Ok((t0, next_tokens))
    }
}

fn parse_P(tokens: &[Token]) -> Result<(Node, &[Token]), &[Token]> {
    if let Some(Token::Number(n)) = tokens.iter().next() {
        return Ok((Node::Single(*n), &tokens[1..]));
    }

    if let Some(Token::Dash) = tokens.iter().next() {
        return Ok((
            Node::Unary(UnaryOperator::Negative, Box::new(parse_P(&tokens[1..])?.0)),
            &tokens[2..],
        ));
    }

    Err(tokens)
}

fn expect<'a>(expected_token: &Token, tokens: &'a [Token]) -> Result<&'a [Token], &'a [Token]> {
    if let Some(expected_token) = tokens.iter().next() {
        Ok(&tokens[1..])
    } else {
        Err(tokens)
    }
}

fn parser(tokens: &[Token]) -> Result<Node, &[Token]> {
    let (t, last_tokens) = parse_E(tokens)?;
    expect(&Token::End, last_tokens)?;
    Ok(t)
}

pub fn calc(input: &str) -> i32 {
    let tokens = tokenizer(input).unwrap();
    let root_node = parser(&tokens).unwrap();

    return root_node.calc();
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

    #[test]
    fn parser_components_tests() {
        let tokens = vec![Token::Dash, Token::Number(3)];

        assert_eq!(
            parse_P(&tokens),
            Ok((
                Node::Unary(UnaryOperator::Negative, Box::new(Node::Single(3))),
                &tokens[2..]
            )),
            "Parses single digits and negative numbers"
        );

        let tokens = vec![
            Token::Number(3),
            Token::Caret,
            Token::Dash,
            Token::Number(2),
        ];

        assert_eq!(
            parse_F(&tokens),
            Ok((
                Node::Binary(
                    BinaryOperator::Exponentiation,
                    Box::new(Node::Single(3),),
                    Box::new(Node::Unary(
                        UnaryOperator::Negative,
                        Box::new(Node::Single(2))
                    ))
                ),
                &tokens[4..]
            ),),
        );

        let tokens = vec![Token::Number(3), Token::Asterisk, Token::Number(5)];

        assert_eq!(
            parse_T(&tokens),
            Ok((
                Node::Binary(
                    BinaryOperator::Multiplication,
                    Box::new(Node::Single(3)),
                    Box::new(Node::Single(5)),
                ),
                &tokens[3..]
            ),),
        );

        let tokens = vec![
            Token::Number(2),
            Token::Plus,
            Token::Number(3),
            Token::Asterisk,
            Token::Number(5),
        ];

        assert_eq!(
            parse_E(&tokens),
            Ok((
                Node::Binary(
                    BinaryOperator::Addition,
                    Box::new(Node::Single(2)),
                    Box::new(Node::Binary(
                        BinaryOperator::Multiplication,
                        Box::new(Node::Single(3)),
                        Box::new(Node::Single(5)),
                    ))
                ),
                &tokens[5..]
            ),),
        )
    }

    #[test]
    fn parser_tests() {
        // -3 + 7 * 2 ^ 2

        let tokens = vec![
            Token::Dash,
            Token::Number(3),
            Token::Plus,
            Token::Number(7),
            Token::Asterisk,
            Token::Number(2),
            Token::Caret,
            Token::Number(2),
            Token::End,
        ];
        let ast = Node::Binary(
            BinaryOperator::Addition,
            Box::new(Node::Unary(
                UnaryOperator::Negative,
                Box::new(Node::Single(3)),
            )),
            Box::new(Node::Binary(
                BinaryOperator::Multiplication,
                Box::new(Node::Single(7)),
                Box::new(Node::Binary(
                    BinaryOperator::Exponentiation,
                    Box::new(Node::Single(2)),
                    Box::new(Node::Single(2)),
                )),
            )),
        );

        assert_eq!(parser(&tokens), Ok(ast));
    }

    #[test]
    fn calc_tests() {
        assert_eq!(calc("-3 + 7 * 2 ^ 2"), 25);
    }
}
