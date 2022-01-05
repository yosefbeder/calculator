use super::tokenizer::Token;

/*
    GRAMMAR
        E -> T {("+" | "-") T}
        T -> F {("*" | "/") F}
        F -> P ["^" F]
        P -> v | "(" E ")" | "-" P
*/

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
  Addition,
  Subtraction,
  Division,
  Multiplication,
  Exponentiation,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
  Negative,
}

#[derive(Debug, PartialEq)]
pub enum Node {
  Binary(BinaryOperator, Box<Node>, Box<Node>),
  Unary(UnaryOperator, Box<Node>),
  Single(i32),
}

impl Node {
  pub fn calc(&self) -> i32 {
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

fn parse_e(tokens: &[Token]) -> Result<(Node, &[Token]), &[Token]> {
  let (mut node, mut next_tokens) = parse_t(tokens)?;

  while let Some(token) = next_tokens.iter().next() {
    match token {
      Token::Plus => {
        let (node1, next_tokens_1) = parse_t(&next_tokens[1..])?;
        next_tokens = next_tokens_1;

        node = Node::Binary(BinaryOperator::Addition, Box::new(node), Box::new(node1));
      }
      Token::Dash => {
        let (node1, next_tokens_1) = parse_t(&next_tokens[1..])?;
        next_tokens = next_tokens_1;

        node = Node::Binary(BinaryOperator::Subtraction, Box::new(node), Box::new(node1));
      }
      _ => break,
    }
  }

  Ok((node, next_tokens))
}

fn parse_t(tokens: &[Token]) -> Result<(Node, &[Token]), &[Token]> {
  let (mut node, mut next_tokens) = parse_f(tokens)?;

  while let Some(token) = next_tokens.iter().next() {
    match token {
      Token::Asterisk => {
        let (node1, next_tokens_1) = parse_f(&next_tokens[1..])?;
        next_tokens = next_tokens_1;

        node = Node::Binary(
          BinaryOperator::Multiplication,
          Box::new(node),
          Box::new(node1),
        );
      }
      Token::Slash => {
        let (node1, next_tokens_1) = parse_f(&next_tokens[1..])?;
        next_tokens = next_tokens_1;

        node = Node::Binary(BinaryOperator::Division, Box::new(node), Box::new(node1));
      }
      _ => break,
    }
  }

  Ok((node, next_tokens))
}

fn parse_f(tokens: &[Token]) -> Result<(Node, &[Token]), &[Token]> {
  let (node, next_tokens) = parse_p(tokens)?;

  if let Some(Token::Caret) = next_tokens.iter().next() {
    let (node1, next_tokens) = parse_f(&next_tokens[1..])?;
    Ok((
      Node::Binary(
        BinaryOperator::Exponentiation,
        Box::new(node),
        Box::new(node1),
      ),
      next_tokens,
    ))
  } else {
    Ok((node, next_tokens))
  }
}

fn parse_p(tokens: &[Token]) -> Result<(Node, &[Token]), &[Token]> {
  if let Some(Token::Number(n)) = tokens.iter().next() {
    return Ok((Node::Single(*n), &tokens[1..]));
  }

  if let Some(Token::Dash) = tokens.iter().next() {
    let (node, next_tokens) = parse_p(&tokens[1..])?;

    return Ok((
      Node::Unary(UnaryOperator::Negative, Box::new(node)),
      next_tokens,
    ));
  }

  if let Some(Token::LeftParenthese) = tokens.iter().next() {
    let (node, next_tokens) = parse_e(&tokens[1..])?;
    return Ok((node, expect(Token::RightParenthese, next_tokens)?));
  }

  Err(tokens)
}

fn expect<'a>(_expected_token: Token, tokens: &'a [Token]) -> Result<&'a [Token], &'a [Token]> {
  if let Some(_expected_token) = tokens.iter().next() {
    Ok(&tokens[1..])
  } else {
    Err(tokens)
  }
}

pub fn parse(tokens: &[Token]) -> Result<Node, &[Token]> {
  let (node, next_tokens) = parse_e(tokens)?;
  expect(Token::End, next_tokens)?;
  Ok(node)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn parses_p() {
    let tokens = vec![Token::Number(3), Token::End];
    assert_eq!(
      parse(&tokens),
      Ok(Node::Single(3)),
      "Parses P (single digits)"
    );

    let tokens = vec![Token::Dash, Token::Number(3), Token::End];
    assert_eq!(
      parse(&tokens),
      Ok(Node::Unary(
        UnaryOperator::Negative,
        Box::new(Node::Single(3))
      )),
      "Parses P (negative numbers)"
    );

    let tokens = vec![
      Token::LeftParenthese,
      Token::Number(3),
      Token::RightParenthese,
      Token::End,
    ];
    assert_eq!(
      parse(&tokens),
      Ok(Node::Single(3)),
      "Parses P (expressions wrapped with parentheses)"
    );
  }

  #[test]
  fn parses_f() {
    let tokens = vec![Token::Number(3), Token::Caret, Token::Number(2), Token::End];
    assert_eq!(
      parse(&tokens),
      Ok(Node::Binary(
        BinaryOperator::Exponentiation,
        Box::new(Node::Single(3)),
        Box::new(Node::Single(2))
      )),
      "Parses F (operands are single digits)"
    );

    let tokens = vec![
      Token::Number(3),
      Token::Caret,
      Token::Number(2),
      Token::Caret,
      Token::Number(2),
      Token::End,
    ];
    assert_eq!(
      parse(&tokens),
      Ok(Node::Binary(
        BinaryOperator::Exponentiation,
        Box::new(Node::Single(3)),
        Box::new(Node::Binary(
          BinaryOperator::Exponentiation,
          Box::new(Node::Single(2)),
          Box::new(Node::Single(2))
        )),
      )),
      "Parses F (operands are F too)"
    );
  }

  #[test]
  fn parses_t() {
    let tokens = vec![
      Token::Number(2),
      Token::Asterisk,
      Token::Number(5),
      Token::Slash,
      Token::Number(3),
      Token::End,
    ];

    assert_eq!(
      parse(&tokens),
      Ok(Node::Binary(
        BinaryOperator::Division,
        Box::new(Node::Binary(
          BinaryOperator::Multiplication,
          Box::new(Node::Single(2)),
          Box::new(Node::Single(5)),
        )),
        Box::new(Node::Single(3)),
      )),
      "Parses T (operands aren't single nodes)"
    );
  }

  #[test]
  fn parses_e() {
    let tokens = vec![
      Token::Number(2),
      Token::Plus,
      Token::Number(5),
      Token::Dash,
      Token::Number(3),
      Token::End,
    ];

    assert_eq!(
      parse(&tokens),
      Ok(Node::Binary(
        BinaryOperator::Subtraction,
        Box::new(Node::Binary(
          BinaryOperator::Addition,
          Box::new(Node::Single(2)),
          Box::new(Node::Single(5)),
        )),
        Box::new(Node::Single(3)),
      )),
      "Parses E (operands aren't single nodes)"
    );
  }

  #[test]
  fn parses_a_full_example() {
    // -3 + (7 * 2) ^ 2
    let tokens = vec![
      Token::Dash,
      Token::Number(3),
      Token::Plus,
      Token::LeftParenthese,
      Token::Number(7),
      Token::Asterisk,
      Token::Number(2),
      Token::RightParenthese,
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
        BinaryOperator::Exponentiation,
        Box::new(Node::Binary(
          BinaryOperator::Multiplication,
          Box::new(Node::Single(7)),
          Box::new(Node::Single(2)),
        )),
        Box::new(Node::Single(2)),
      )),
    );

    assert_eq!(parse(&tokens), Ok(ast));
  }
}
