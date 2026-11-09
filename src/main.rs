mod contracts;
mod intake;
mod manager;
mod reviewer;
mod editor;
mod store;
mod mailroom;
mod junk;

use contracts::Run;
use dotenvy::dotenv;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::contracts::{Task, Job};


#[tokio::main]
async fn main(){

	dotenv().ok();

	let model = std::env::var("OPENROUTER_MODEL").unwrap();
	let temperature: f32 = std::env::var("OPENROUTER_TEMPERATURE").unwrap().parse().unwrap();


	let args: Vec<String> = std::env::args().collect();

	let (diff_output, diff_label, pr) = match args.len() {
		2 => {
			match intake::parse_pr_url(&args[1]) {
				Some(pr) => {
					let diff = intake::diff_from_pr(&pr).await.unwrap();
					
					let label = format!("{}/{}#{}", pr.owner, pr.repo, pr.number);

					(diff, label, Some(pr))
				},
				None => {
					let diff = intake::diff_from_file(&args[1]).unwrap();
					(diff, args[1].clone(), None)
				}
			}

			// args = [
			//   "target/debug/smorfs",     // args[0]: always there, the program itself
			//   "evals/cases/p95-latency/diff.patch",      // args[1]: the 1 word we typed
			// ]		
		},
		3 | 4 => {
				let job = Job {
					repo_path: args[1].clone(), 
					source_branch: args[2].clone(),
					target_branch: args.get(3).cloned().unwrap_or("main".to_string()), 
				};

				let diff = intake::diff_from_branch(&job).unwrap();
				let label = format!("{} {}...{}", job.repo_path, job.target_branch, job.source_branch);

			(diff, label, None)
		}

		_ => {
			eprintln!("usage:\n  cargo run -- <path/to/diff.patch>\n  cargo run -- <github pr link>\n  cargo run -- <repo path> <branch> [target branch]");
			std::process::exit(1);
		}
	};

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

	let manager_result = manager::run_tasks(&clean, &tasks).await;

	// totals
	let total_prompt_tokens: u32 = manager_result.tasks
		.iter()
		.map(|t| t.prompt_tokens)
		.sum();

	let total_completion_tokens: u32 = manager_result.tasks
		.iter()
		.map(|c| c.completion_tokens)
		.sum();



	let findings = editor::filter_findings(&clean, manager_result.findings);
	let findings = editor::merge_duplicates(findings);

	// ranking according to severity
	let findings = editor::rank_by_severity(findings);

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
		diff_path: diff_label,
		tasks: manager_result.tasks,
		findings,
		total_prompt_tokens,
		total_completion_tokens,
		duration_ms,
	};

	let path = store::save_run(&run).unwrap();
	println!("saved {}", path);

	if let Some(pr) = &pr {
		// TODO: call mailroom::post_review, passing pr and &run.findings
		//       it's async and returns a Result, so .await then .unwrap()

		mailroom::post_review(pr, &run.findings).await.unwrap();
		println!("posted review to {}/{}#{}", pr.owner, pr.repo, pr.number);
	}
}
