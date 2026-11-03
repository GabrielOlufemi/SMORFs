mod contracts;
mod intake;
mod manager;
mod reviewer;
mod editor;
mod store;
mod junk;

use contracts::Run;
use dotenvy::dotenv;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::contracts::Task;

fn main(){

  dotenv().ok();

	let model = std::env::var("OPENROUTER_MODEL").unwrap();
	let temperature: f32 = std::env::var("OPENROUTER_TEMPERATURE").unwrap().parse().unwrap();

  let diff_path = std::env::args()
    .nth(1)
    .expect("usage: cargo run --<path/to/diff.patch>");

  let diff_output = intake::diff_from_file(&diff_path).unwrap();

  let clean = intake::remove_junk_files(&diff_output);

  // the 5 smorfs
  let tasks = vec![
    Task {
      // grouchy smorf
      reviewer: "tests".to_string(),
      prompt_path: "prompts/tests.md".to_string(),
    },

    Task {
      // brainy smorf
      reviewer: "correctness".to_string(),
      prompt_path: "prompts/correctness.md".to_string(),
    },

    Task {
      // hefty smorf
      reviewer: "performance".to_string(),
      prompt_path: "prompts/performance.md".to_string(),
    },

    Task {
      // hadny smorf
      reviewer: "security".to_string(),
      prompt_path: "prompts/security.md".to_string(),
    },

    Task {
      // vanity smorf
      reviewer: "style".to_string(),
      prompt_path: "prompts/style.md".to_string(),
    },
  ];

  let started = std::time::Instant::now();

  let manager_result = manager::run_tasks(&clean, &tasks);

  // totals
  let total_prompt_tokens: u32 = manager_result.tasks
    .iter()
    .map(|t| t.prompt_tokens)
    .sum();

  let total_completion_tokens: u32 = manager_result.tasks
    .iter()
    .map(|c| c.completion_tokens)
    .sum();

  // building our run


  let findings = editor::filter_findings(&clean, manager_result.findings);

  let duration_ms = started.elapsed().as_millis() as u64;

  let id = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis()
    .to_string();


  let run = Run {
    schema_version: 2,
    id,
    model,
    temperature,
    diff_path,
    tasks: manager_result.tasks,
    findings,
    total_prompt_tokens,
    total_completion_tokens,
    duration_ms,
  };

  let path = store::save_run(&run).unwrap();
  println!("saved {}", path);

  // println!("{}", serde_json::to_string_pretty(&run.findings).unwrap());

}
