/*
You are going to be given a word. Your job is to return the middle character of the word.
If the word's length is odd, return the middle character.
If the word's length is even, return the middle 2 characters.

# Examples:
Kata.getMiddle("test") should return "es"
Kata.getMiddle("testing") should return "t"
Kata.getMiddle("middle") should return "dd"
Kata.getMiddle("A") should return "A"

#Input
A word (string) of length 0 < str < 1000.
You do not need to test for this.
This is only here to tell you that you do not need to worry about your solution timing out.

#Output
The middle character(s) of the word represented as a string.
*/

/*
// PREFERRED SOLUTION
fn get_middle(s:&str) -> &str {
    &s[(s.len()-1)/2..s.len()/2+1]
}
*/

pub fn get_middle(s: &str) -> &str {
  let half = s.len() / 2;
  match s.len() % 2 {
    0 => &s[half - 1..half + 1],
    _ => &s[half..half + 1],
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_tests() {
    assert_eq!(get_middle("test"), "es");
    assert_eq!(get_middle("testing"), "t");
    assert_eq!(get_middle("middle"), "dd");
    assert_eq!(get_middle("A"), "A");
    assert_eq!(get_middle("of"), "of");
  }
}
