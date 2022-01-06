mod parser;
mod tokenizer;

pub fn calculate(input: &str) -> Result<i32, String> {
  match tokenizer::tokenize(input) {
    Ok(tokens) => match parser::parse(&tokens) {
      Ok(tree) => Ok(tree.calc()),
      Err(err) => Err(format!("[parser]: {}", err)),
    },
    Err(err) => Err(format!("[tokenizer]: {}", err)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculate_tests() {
    assert_eq!(calculate("-3 + 7 * 2 ^ 2"), Ok(25));
    assert_eq!(calculate("-3 + (7 * 2) ^ 2"), Ok(193));
    assert_eq!(calculate("-(7 * 2)"), Ok(-14));
  }

  #[test]
  fn throws_meaningful_errors() {
    assert_eq!(
      calculate("-3 + 7a * 2 ^ 2"),
      Err(String::from("[tokenizer]: Unexpected character: a"))
    );
    assert_eq!(
      calculate("-3 + (7 * 2 ^ 2"),
      Err(String::from(
        "[parser]: Expected a closing parenthese ')', but Instead, got the end token"
      ))
    );
    assert_eq!(
      calculate("-(7 * )"),
      Err(String::from(
        "[parser]: Expected a number (positive or negative) or an expression wrapped inside parentheses, but Instead, got a closing parenthese ')'"
      ))
    );
  }
}
