/*
In a factory a printer prints labels for boxes. The printer uses colors which,
for the sake of simplicity, are named with letters from a to z (except letters
u, w, x or z that are for errors).

The colors used by the printer are recorded in a control string.
For example a control string would be aaabbbbhaijjjm meaning that the printer
used three times color a, four times color b, one time color h then one time
color a... and so on.

Sometimes there are problems: lack of colors, technical malfunction and a control
string is produced e.g. uuaaaxbbbbyyhwawiwjjjwwxym where errors are reported with
letters u, w, x or z.

You have to write a function hist which given a string will output the errors as a
string representing a histogram of the encountered errors.

Format of the output string:

letter (error letters are sorted in alphabetical order) in a field of 2
characters, a white space, number of error for that letter in a field of 6, as
many "*" as the number of errors for that letter and "\r" (or "\n" depending on the langauge).

The string has a length greater or equal to one and contains only letters from a to z.

Examples
s="uuaaaxbbbbyyhwawiwjjjwwxym"
hist(s) => "u  2     **\rw  5     *****\rx  2     **"
or with dots to see white spaces:

hist(s) => "u..2.....**\rw..5.....*****\rx..2.....**"
s="uuaaaxbbbbyyhwawiwjjjwwxymzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz"
hist(s) => "u..2.....**\rw..5.....*****\rx..2.....**\rz..31....*******************************"
*/

/*
PREFERRED SOLUTIONS

fn hist(s: &str) -> String {
    ["u", "w", "x", "z"]
        .iter()
        .fold(vec![], |mut acc, error| {
          let count = s.matches(error).count();
          if count > 0 {
              acc.push(format!("{:2} {:<6}{}", error, count, "*".repeat(count)))
          }
          acc
        })
        .join("\r")
}

static ERRORS: &'static [char] = &['u', 'w', 'x', 'z'];
fn hist(s: &str) -> String {
    s
        .chars()
        .filter(|c| ERRORS.iter().any(|x| x == c))
        .sorted()
        .group_by(|e| e.clone())
        .into_iter()
        .map(|(e, g)| (e, g.count()))
        .map(|(e, c)| format!("{}  {}     {}", e, c, "*".repeat(c)))
        .join("\r")
}

fn hist(s: &str) -> String {
    let r = ["u","w","x","z"]; let mut res = "".to_string();
    for rr in r.iter() {
        let cnt = s.matches(rr).count();
        if cnt > 0 {
            res = res + &format!("{:2} {:<6}", rr, cnt);
            res = res + &"*".repeat(cnt);
            res = res + "\r";
        }
    }
    if res.len() != 0 {return res[0..res.len() - 1].to_string()}
    return res;
}
*/

fn generate_error_string(code: char, count: usize) -> String {
  format!("{0:<2} {2:<6}{1:*<2$}", code, '*', count)
}

fn count_char_in_string(search_char: char, string: &str) -> Option<usize> {
  let mut count = 0;
  for current_char in string.chars() {
    if current_char == search_char {
      count += 1;
    }
  }

  match count > 0 {
    true => Some(count),
    false => None,
  }
}

pub fn histogram(printer_log: &str) -> String {
  let error_codes = vec!['u', 'w', 'x', 'z'];
  let mut error_strings: Vec<String> = vec![];

  for code in error_codes.iter() {
    match count_char_in_string(*code, printer_log) {
      Some(n) => error_strings.push(generate_error_string(*code, n)),
      None => (),
    }
  }

  error_strings.join("\n")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_count_number_of_ocurrences() {
    assert_eq!(count_char_in_string('l', "lolerpoperlleeeel").unwrap(), 5);
    assert_eq!(count_char_in_string('l', "fesfsdfdf"), Option::None);
  }

  fn dotest(s: &str, exp: &str) -> () {
    println!("s:{:?}", s);
    let ans = histogram(s);
    println!("actual: {:?}", ans);
    println!("expect: {:?}", exp);
    println!("{}", ans == exp);
    assert_eq!(ans, exp);
    println!("{}", "-");
  }

  #[test]
  fn basic_tests() {
    dotest(
      "tpwaemuqxdmwqbqrjbeosjnejqorxdozsxnrgpgqkeihqwybzyymqeazfkyiucesxwutgszbenzvgxibxrlvmzihcb",
      "u  3     ***\nw  4     ****\nx  6     ******\nz  6     ******",
    );
    dotest(
      "aaifzlnderpeurcuqjqeywdq",
      "u  2     **\nw  1     *\nz  1     *",
    );
  }
}
