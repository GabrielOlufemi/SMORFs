mod contracts;
mod intake;
mod reviewer;
mod editor;
mod junk;

use dotenvy::dotenv;
use contracts::{Category, Finding, Severity};

fn main(){

  dotenv().ok();

  // testing finding struct via serde package
  let finding = Finding {
    severity: Severity::Low,
    category: Category::Style,
    file: String::from("scrapbook/README.txt"),
    line: 14,
    message: String::from("Typo should be updated"), 
  };

  let json = serde_json::to_string_pretty(&finding).unwrap();

  println!("{json}");

  // testing diff froms file fn
  let diff_output = intake::diff_from_file("evals/cases/retry-on-failure/diff.patch").unwrap();

  println!("{}", diff_output);

  // testing junk removal: this case has no junk, so nothing should change
  let clean = intake::remove_junk_files(&diff_output);
  println!("unchanged: {}", clean == diff_output);

  let findings = reviewer::review(&clean).unwrap();
  let findings = editor::filter_findings(&clean, findings);
  println!("{}", serde_json::to_string_pretty(&findings).unwrap());
  
}