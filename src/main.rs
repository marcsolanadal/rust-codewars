mod kattas;

pub use crate::kattas::{ease_the_stock_broker, word_spinner};

fn main() {
    let phrase = String::from("Welcome travelers! This is Sparta.");
    let spinned_phrase = word_spinner::spin_words(&phrase);

    println!("spinned phrase is: {}", spinned_phrase);

    let multiple_order = "GOOG 300 542.0 B, AAPL 50 145.0 B, CSCO 250.0 29 B, GOOG 200 580.0 S";
    let order_summary = ease_the_stock_broker::balance_statement(&multiple_order);

    println!("order summary: {}", order_summary);
}