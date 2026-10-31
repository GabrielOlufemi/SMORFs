mod contracts;

use contracts::{Category, Finding, Severity};
  
fn main(){
  let finding = Finding {
    severity: Severity::Low,
    category: Category::Style,
    file: String::from("scrapbook/README.txt"),
    line: 14,
    message: String::from("Typo should be updated"), 
  };

  let json = serde_json::to_string_pretty(&finding).unwrap();

  println!("{json}");
  
}