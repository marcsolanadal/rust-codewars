/*
You are given an array (which will have a length of at least 3, but could be very large) containing integers.
The array is either entirely comprised of odd integers or entirely comprised of even integers except for a single integer N.

Write a method that takes the array as an argument and returns this "outlier" N.

Examples
[2, 4, 0, 100, 4, 11, 2602, 36]
Should return: 11 (the only odd number)

[160, 3, 1719, 19, 11, 13, -21]
Should return: 160 (the only even number)
*/

/*
// PREFERRED SOLUTION
fn find_outlier(values: &[i32]) -> i32 {
    let sum: i32 = values.iter()
        .take(3)
        .map(|n| n.abs() % 2)
        .sum();

    let m = if sum == 1 || sum == 0 { 1 } else { 0 };

    values
        .iter()
        .find(|n| n.abs() % 2 == m)
        .map(|n| *n)
        .unwrap_or(0)
}
*/

#[derive(PartialEq, Debug)]
enum Parity {
  Odd,
  Even,
}

fn get_array_parity(values: &[i32]) -> Parity {
  match i32::abs(values[0] % 2) + i32::abs(values[1] % 2) + i32::abs(values[2] % 2) {
    0 | 1 => Parity::Even,
    _ => Parity::Odd,
  }
}

pub fn find_outlier(values: &[i32]) -> i32 {
  *values
    .iter()
    .find(|&n| match get_array_parity(&values) {
      Parity::Even => i32::abs(*n % 2) == 1,
      Parity::Odd => i32::abs(*n % 2) == 0,
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_array_parity() {
    assert_eq!(get_array_parity(&[0, 2, 4]), Parity::Even);
    assert_eq!(get_array_parity(&[1, 2, 4]), Parity::Even);
    assert_eq!(get_array_parity(&[1, 3, 4]), Parity::Odd);
    assert_eq!(get_array_parity(&[1, 3, 5]), Parity::Odd);
    assert_eq!(get_array_parity(&[2, -6, 8]), Parity::Even);
  }

  #[test]
  fn check_negative_number() {
    assert_eq!(get_array_parity(&[-1, 2, 4]), Parity::Even);
  }

  #[test]
  fn basic_test() {
    let t1 = [2, 6, 8, -10, 3];
    let t2 = [
      206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
    ];
    let t3 = [std::i32::MAX, 0, 1];
    assert_eq!(3, find_outlier(&t1));
    assert_eq!(206847684, find_outlier(&t2));
    assert_eq!(0, find_outlier(&t3));
  }
}
