use serde::Serialize;
use crate::contracts::{Finding, PullRequest};

#[derive(Serialize)]
struct ReviewRequest {
  event: String,
  body: String,
  comments: Vec<ReviewComment>
}

#[derive(Serialize)]
struct ReviewComment {
  path: String,
  line: u32,
  side: String,
  body: String
}

pub async fn post_review(pr: &PullRequest, findings: &[Finding]) -> Result<(), Box<dyn std::error::Error>> {
  
  if findings.is_empty() {
    return Ok(())
  }

  let access_token = std::env::var("GITHUB_ACCESS_TOKEN")?;

  let url = format!(
    "https://api.github.com/repos/{}/{}/pulls/{}/reviews",
    pr.owner,
    pr.repo,
    pr.number
  );

  let mut comments = Vec::new();

  for f in findings{
    comments.push(ReviewComment {
      path: f.file.clone(),
      line: f.line,
      side: "RIGHT".to_string(),
      body: format!("**{:?}** ({})\n\n{}", f.severity, f.reviewer, f.message),
    });
  }

    let review = ReviewRequest {
      event: "COMMENT".to_string(),
      body: format!("SMORFs found {} issue(s).", findings.len()),
      comments,
    };

    let client = reqwest::Client::new();

    client
      .post(&url)
      .bearer_auth(&access_token)
      .header("User-Agent", "smorfs")
      .json(&review)
      .send().await?
      .error_for_status()?;

    Ok(())
    
}