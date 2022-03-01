/*
Your task is to sort a given string. Each word in the string will contain a single number.
This number is the position the word should have in the result.

Note: Numbers can be from 1 to 9. So 1 will be the first word (not 0).

If the input string is empty, return an empty string.
The words in the input String will only contain valid consecutive numbers.

Examples
"is2 Thi1s T4est 3a"  -->  "Thi1s is2 3a T4est"
"4of Fo1r pe6ople g3ood th5e the2"  -->  "Fo1r the2 g3ood 4of th5e pe6ople"
""  -->  ""
*/

/*
// PREFERRED SOLUTION
fn order(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}
*/

fn get_word_index_tuple(word: &str) -> Option<(usize, String)> {
  for ch in word.chars() {
    if ch.is_digit(10) {
      return Some((ch.to_digit(10).unwrap() as usize, word.to_string()));
    }
  }
  None
}

pub fn order(sentence: &str) -> String {
  if sentence == "" {
    return String::from("");
  }

  let words_with_indexes = sentence
    .split(" ")
    .map(|word| get_word_index_tuple(word).unwrap())
    .collect::<Vec<_>>();

  let mut result = vec![""; words_with_indexes.len()];
  for (i, word) in &words_with_indexes {
    result[*i - 1] = &word;
    println!("i: {}, w: {}", i, word)
  }

  result.join(" ")
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn returns_expected() {
    assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
    assert_eq!(order(""), "");
  }
}
