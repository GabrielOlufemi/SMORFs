use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Finding {
  #[serde(default)] // so manager wil be able to prefill reviewer field 
  pub reviewer: String,
  pub severity: Severity,
  pub category: Category,
  pub file : String,
  pub line: u32,
  pub message: String, 
}

// derived order follows declaration order: High < Medium < Low, so the most severe sorts first
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
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

// cuz of this structure 
// a pull request on github: https://github.com/<owner>/<repo>/pull/<number>
pub struct PullRequest {
  pub owner: String,
  pub repo: String,
  pub number: u32
}

#[derive(Serialize, Deserialize)]
pub struct Run {
  pub schema_version: u32,
  pub id: String,
  pub model: String,
  pub diff_path: String,
  // pub job: Job, i'll add back once ready for real branch review 
  pub temperature: f32,
  pub tasks: Vec<TaskRecord>, //basically log of agents that ran
  pub findings: Vec<Finding>,
  pub total_prompt_tokens: u32,
  pub total_completion_tokens: u32,
  pub duration_ms: u64
}

// task related p - uncertain
pub struct Task {
  pub reviewer: String,
  pub prompt_path: String,
}

// how a task ended - spawned agent review
#[derive(Serialize, Deserialize)]
pub enum Outcome {
  Success,
  Failure(String),
}

// details of a task
#[derive(Serialize, Deserialize)]
pub struct TaskRecord { 
  pub reviewer: String,
  pub prompt_path: String,
  pub outcome: Outcome,
  pub prompt_tokens: u32,
  pub completion_tokens: u32,
  pub duration_ms: u64,
}