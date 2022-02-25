/*
Complete the method that takes a boolean value and return a "Yes" string
for true, or a "No" string for false.
*/

// Prefered solution
/*
fn bool_to_word(value: bool) -> &'static str {
    match value {
        true => "Yes",
        false => "No",
    }
}
*/

pub fn bool_to_word(value: bool) -> &'static str {
  if value == true {
    "Yes"
  } else {
    "No"
  }
}

#[cfg(test)]
mod tests {
  use super::bool_to_word;

  #[test]
  fn example_tests() {
    assert_eq!(bool_to_word(true), "Yes");
    assert_eq!(bool_to_word(false), "No");
  }
}
