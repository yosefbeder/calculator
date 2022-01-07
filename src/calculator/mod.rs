mod parser;
mod tokenizer;

pub fn calculate(input: &str) -> Result<f64, String> {
  match tokenizer::tokenize(input) {
    Ok(tokens) => match parser::parse(&tokens) {
      Ok(tree) => Ok(tree.calculate()),
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
    assert_eq!(calculate("-3 + 7 * 2 ^ 2"), Ok(25.0));
    assert_eq!(calculate("-3 + (7 * 2) ^ 2"), Ok(193.0));
    assert_eq!(calculate("-(7 * 2)"), Ok(-14.0));
    assert_eq!(calculate("1 / 2"), Ok(0.5));
    assert_eq!(calculate("-3 + 7 * 2 ^ 2 / 60"), Ok(-2.533333333333333));
    assert_eq!(
      calculate("70 - 23 * 32 ^ 2 / 90 + (5 / 3) ^ 2"),
      Ok(-188.9111111111111)
    );
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
