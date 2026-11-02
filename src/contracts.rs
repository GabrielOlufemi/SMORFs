use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Finding {
  pub severity: Severity,
  pub category: Category,
  pub file : String,
  pub line: u32,
  pub message: String, 
}

#[derive(Serialize, Deserialize)]
pub enum Severity {
  High,
  Medium,
  Low,
}

#[derive(Serialize, Deserialize)]
pub enum Category {
  Security, 
  Style,
  Performance,
  Tests,
  Correctness
}

#[derive(Serialize, Deserialize)]
pub struct Job {
  pub repo_path: String,
  pub source_branch: String,  //branch we intend to merge 
  pub target_branch: String,  //branch receiving the changes
}

#[derive(Serialize, Deserialize)]
pub struct Run {
  pub schema_version: u32,
  pub id: String,
  pub model: String,
  pub diff_path: String,
  // pub job: Job,  -- i'll add back once ready for real branch review 
  pub temperature: f32,
  pub prompt_version: String,
  pub findings: Vec<Finding>,
  pub prompt_tokens: u32,
  pub completion_tokens: u32,
  pub duration_ms: u64
}