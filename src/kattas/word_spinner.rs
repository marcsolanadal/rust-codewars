/*
Write a function that takes in a string of one or more words, and returns the same string,
but with all five or more letter words reversed (Just like the name of this Kata).
Strings passed in will consist of only letters and spaces.
Spaces will be included only when more than one word is present.

Examples:
    spinWords( "Hey fellow warriors" ) => returns "Hey wollef sroirraw"
    spinWords( "This is a test") => returns "This is a test"
    spinWords( "This is another test" )=> returns "This is rehtona test"
*/

/*
fn spin_words(words: &str) -> String {
    words.split_ascii_whitespace().map(|word| match word.len() >= 5 {
        true => word.chars().rev().collect(),
        false => word.to_string()
    }).collect::<Vec<String>>().join(" ")
}
*/

pub fn spin_words(phrase: &str) -> String {
  let mut words = vec![];

  for word in phrase.split_whitespace() {
    if word.len() >= 5 {
      words.push(reverse_word(&word));
    } else {
      words.push(word.to_string());
    }
  }
  words.join(" ")
}

fn reverse_word(word: &str) -> String {
  let char_list = word.chars();
  let mut new_word = String::from("");

  for c in char_list.rev() {
    new_word.push_str(&c.to_string());
    // print!("{}", c);
  }

  new_word
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn examples() {
    assert_eq!(spin_words("Welcome"), "emocleW");
    assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
    assert_eq!(spin_words("This is a test"), "This is a test");
    assert_eq!(spin_words("This is another test"), "This is rehtona test");
    assert_eq!(
      spin_words("You are almost to the last test"),
      "You are tsomla to the last test"
    );
    assert_eq!(
      spin_words("Just kidding there is still one more"),
      "Just gniddik ereht is llits one more"
    );
    assert_eq!(
      spin_words("Seriously this is the last one"),
      "ylsuoireS this is the last one"
    );
  }
}
