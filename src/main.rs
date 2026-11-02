mod contracts;
mod intake;
mod reviewer;
mod editor;
mod store;
mod junk;

use contracts::Run;
use dotenvy::dotenv;
use std::time::{SystemTime, UNIX_EPOCH};

fn main(){

  dotenv().ok();

  let diff_path = "evals/cases/retry-on-failure/diff.patch";
  let diff_output = intake::diff_from_file(diff_path).unwrap();

  let clean = intake::remove_junk_files(&diff_output);

  let started = std::time::Instant::now();
  let result = reviewer::review(&clean).unwrap();
  let findings = editor::filter_findings(&clean, result.findings);

  let duration_ms = started.elapsed().as_millis() as u64;

  let id = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis()
    .to_string();


  let run = Run {
    schema_version: 1,
    id,
    model: result.model,
    temperature: result.temperature,
    diff_path: diff_path.to_string(),
    prompt_version: "generalist-v1".to_string(),
    findings,
    prompt_tokens: result.prompt_tokens,
    completion_tokens: result.completion_tokens,
    duration_ms,
  };

  let path = store::save_run(&run).unwrap();
  println!("saved {}", path);

  // println!("{}", serde_json::to_string_pretty(&run.findings).unwrap());

}
