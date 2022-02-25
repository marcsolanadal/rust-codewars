/*
Clients place orders to a stockbroker as strings. The order can be simple or multiple or empty.

Type of a simple order: Quote/white-space/Quantity/white-space/Price/white-space/Status
where Quote is formed of non-whitespace character, Quantity is an int, Price a double
(with mandatory decimal point "." ), Status is represented by the letter B (buy) or the letter S (sell).

Example:
"GOOG 300 542.0 B"

A multiple order is the concatenation of simple orders with a comma between each.

Example:
"ZNGA 1300 2.66 B, CLH15.NYM 50 56.32 B, OWW 1000 11.623 B, OGG 20 580.1 B"

or

"ZNGA 1300 2.66 B,CLH15.NYM 50 56.32 B,OWW 1000 11.623 B,OGG 20 580.1 B"

To ease the stockbroker your task is to produce a string of type

"Buy: b Sell: s" where b and s are 'double' formatted with no decimal, b representing the
total price of bought stocks and s the total price of sold stocks.

Example:
"Buy: 294990 Sell: 0"

Unfortunately sometimes clients make mistakes. When you find mistakes in orders, you must
pinpoint these badly formed orders and produce a string of type:

"Buy: b Sell: s; Badly formed nb: badly-formed 1st simple order ;badly-formed nth simple order ;"

where nb is the number of badly formed simple orders, b representing the total price of bought
stocks with correct simple order and s the total price of sold stocks with correct simple order.

Examples:
"Buy: 263 Sell: 11802; Badly formed 2: CLH16.NYM 50 56 S ;OWW 1000 11 S ;"

"Buy: 100 Sell: 56041; Badly formed 1: ZNGA 1300 2.66 ;"

Notes:
If the order is empty, Buy is 0 and Sell is 0 hence the return is: "Buy: 0 Sell: 0".
Due to Codewars whitespace differences will not always show up in test results.
With Golang (and maybe others) you can use a format with "%.0f" for "Buy" and "Sell".
*/

/*
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Status {
  B,
  S,
}

impl FromStr for Status {
  fn from_str(s: &str) -> Result<Self, Self::Err> {

    let coords: Vec<&str> = s
      .trim_matches(|p| p == '(' || p == ')')
      .split(',')
      .collect();

    let x_fromstr = coords[0].parse::<i32>()?;
    let y_fromstr = coords[1].parse::<i32>()?;

    Ok(Point {
      x: x_fromstr,
      y: y_fromstr,
    })
  }
}
*/

use std::result;
use std::string::ParseError;

type Result<T> = result::Result<T, ParseError>;

struct SimpleOrder {
  quote: String,
  quantity: i32,
  price: f64,
  status: String,
}

impl SimpleOrder {
  pub fn create(order_string: &str) -> SimpleOrder {
    let order: Vec<&str> = order_string.split_whitespace().collect();

    // I must handle the Result instead of unwrapping.
    SimpleOrder {
      quote: order[0].to_string(),
      quantity: order[1].parse::<i32>().unwrap(),
      price: order[2].parse::<f64>().unwrap(),
      status: order[3].to_string(),
    }
  }
}

struct MultipleOrder {
  orders: Vec<SimpleOrder>,
}

impl MultipleOrder {
  pub fn create(order_string: &str) -> MultipleOrder {
    MultipleOrder {
      orders: order_string
        .split(", ")
        .map(|order| SimpleOrder::create(order))
        .collect(),
    }
  }
}

pub fn balance_statement(multiple_order: &str) -> &str {
  multiple_order
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn simple_order_create() {
    let order_string = "GOOG 300 542.0 B";
    let order = SimpleOrder::create(order_string);

    assert_eq!(order.quote, "GOOG");
    assert_eq!(order.quantity, 300);
    assert_eq!(order.price, 542.0);
    assert_eq!(order.status, "B");
  }

  #[test]
  #[ignore]
  fn simple_order_malformed() {
    let order_string = "GOOG 542.0 300 L";
    let order = SimpleOrder::create(order_string);
  }

  #[test]
  fn multiple_order_create() {
    let order_string = "ZNGA 1300 2.66 B, CLH15.NYM 50 56.32 B, OWW 1000 11.623 B, OGG 20 580.1 B";
    let multiple_order = MultipleOrder::create(order_string);

    assert_eq!(multiple_order.orders[1].quote, "CLH15.NYM");
    assert_eq!(multiple_order.orders[2].price, 11.623);
    assert_eq!(multiple_order.orders[3].status, "B");
    assert_eq!(multiple_order.orders.len(), 4);
  }

  fn dotest(lst: &str, exp: &str) -> () {
    println!("lst: {:?};", lst);
    let ans = balance_statement(lst);
    println!("actual:\n{:?};", ans);
    println!("expect:\n{:?};", exp);
    println!("{};", ans == exp);
    assert_eq!(ans, exp);
    println!("{};", "-");
  }

  #[test]
  #[ignore]
  fn basic_tests() {
    let mut l = "GOOG 300 542.0 B, AAPL 50 145.0 B, CSCO 250.0 29 B, GOOG 200 580.0 S";
    let mut sol = "Buy: 169850 Sell: 116000; Badly formed 1: CSCO 250.0 29 B ;";
    dotest(l, sol);

    l = "GOOG 90 160.45 B, JPMC 67 12.8 S, MYSPACE 24.0 210 B, CITI 50 450 B, CSCO 100 55.5 S";
    sol = "Buy: 14440 Sell: 6408; Badly formed 2: MYSPACE 24.0 210 B ;CITI 50 450 B ;";
    dotest(l, sol);

    l = "ZNGA 1300 2.66 B, CLH15.NYM 50 56.32 B, OWW 1000 11.623 B, OGG 20 580.1 B";
    sol = "Buy: 29499 Sell: 0";
    dotest(l, sol);
  }
}
