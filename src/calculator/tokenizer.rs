const SPECIAL_CHARACTERS: [char; 7] = ['(', ')', '+', '-', '*', '/', '^'];

#[derive(Debug, PartialEq)]
pub enum Token {
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

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
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

    return Err(format!("Unexpected character: {}", ch));
  }

  tokens.push(Token::End);

  Ok(tokens)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tokenizes_numbers() {
    assert_eq!(
      tokenize("432").unwrap(),
      vec![Token::Number(432), Token::End],
      "Tokenizes numbers"
    );
  }

  #[test]
  fn tokenizes_parentheses() {
    assert_eq!(
      tokenize("()").unwrap(),
      vec![Token::LeftParenthese, Token::RightParenthese, Token::End],
    );
  }
  #[test]
  fn tokenizes_operators() {
    let input = "-4 * -3";
    let output = tokenize(input).unwrap();
    assert_eq!(output[2], Token::Asterisk);
    assert_eq!(output[0], Token::Dash);
    assert_eq!(output[3], Token::Dash);
  }

  #[test]
  fn tokenizes_a_full_example() {
    assert_eq!(
      tokenize("4 + 3 * 7 - (9 + 8)").unwrap(),
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
    )
  }

  #[test]
  fn throws_meaningful_errors() {
    assert_eq!(tokenize("4a"), Err(String::from("Unexpected character: a")))
  }
}
