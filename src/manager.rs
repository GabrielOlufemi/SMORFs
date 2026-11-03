// technically papa smorf - the orchestrator i suppose
use futures::future::join_all;
use crate::contracts::{Finding, Task, TaskRecord,Outcome};
use crate::reviewer;

pub struct ManagerResult {
  pub findings: Vec<Finding>,
  pub tasks: Vec<TaskRecord>,
}


pub async fn run_tasks(diff: &str, tasks: &[Task]) -> ManagerResult{
  let mut findings = Vec::new();
  let mut records = Vec::new();

  // phase 1
  let futures = tasks.iter().map(|task| async move{
    // timing our task
    let start = std::time::Instant::now();
    let task_outcome = reviewer::review(diff, &task.prompt_path).await;
    let duration_ms = start.elapsed().as_millis() as u64;

    (task_outcome, duration_ms)
  });

  // phase 2
  let results = join_all(futures).await;

  // phase 3 
  for (task, (task_outcome,duration_ms)) in tasks.iter().zip(results) {
    // match cuz result gives us either ok or err
    match task_outcome{
      Ok(mut result) => {
        for f in result.findings.iter_mut() {
          f.reviewer = task.reviewer.clone();
        }

        records.push(TaskRecord {
          reviewer: task.reviewer.clone(),
          prompt_path: task.prompt_path.clone(),
          outcome: Outcome::Success,
          prompt_tokens: result.prompt_tokens,
          completion_tokens: result.completion_tokens,
          duration_ms,
        });

        findings.extend(result.findings)
      },
      Err(e) => {
        eprintln!("{} failed: {}", task.reviewer, e);

        records.push(TaskRecord {
          reviewer: task.reviewer.clone(),
          prompt_path: task.prompt_path.clone(),
          outcome: Outcome::Failure(e.to_string()),
          prompt_tokens: 0,
          completion_tokens: 0,
          duration_ms,
        });
      }
    }
  }

  ManagerResult { findings, tasks: records }

}
