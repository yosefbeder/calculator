mod parser;
mod tokenizer;

pub fn calculate(input: &str) -> i32 {
  let tokens = tokenizer::tokenize(input).unwrap();
  let tree = parser::parse(&tokens).unwrap();

  return tree.calc();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculate_tests() {
    assert_eq!(calculate("-3 + 7 * 2 ^ 2"), 25);
    assert_eq!(calculate("-3 + (7 * 2) ^ 2"), 193);
    assert_eq!(calculate("-(7 * 2)"), -14);
  }
}
