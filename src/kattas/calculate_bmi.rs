/*
Write function bmi that calculates body mass index (bmi = weight / height2).

if bmi <= 18.5 return "Underweight"
if bmi <= 25.0 return "Normal"
if bmi <= 30.0 return "Overweight"
if bmi > 30 return "Obese"
*/

// PREFERRED SOLUTION
pub fn bmi(weight: u32, height: f32) -> &'static str {
  let index = weight as f32 / height.powi(2);
  match index {
    _ if index <= 18.5 => "Underweight",
    _ if index <= 25.0 => "Normal",
    _ if index <= 30.0 => "Overweight",
    _ => "Obese",
  }
}

// To avoid the warnings we need to use guards instead to float ranges
/*
// MY SOLUTION
pub fn bmi(weight: u32, height: f32) -> &'static str {
  let bmi = weight as f32 / (height * height);
  match bmi {
    0.0..=18.4 => "Underweight",
    18.5..=24.9 => "Normal",
    25.0..=29.9 => "Overweight",
    _ => "Obese",
  }
}
*/

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_tests() {
    assert_eq!(bmi(50, 1.80), "Underweight");
    assert_eq!(bmi(80, 1.80), "Normal");
    assert_eq!(bmi(90, 1.80), "Overweight");
    assert_eq!(bmi(110, 1.80), "Obese");
  }
}
