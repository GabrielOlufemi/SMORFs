use crate::junk::is_junk;
use std::process::Command;
use crate::contracts::{Job, PullRequest};

pub fn parse_pr_url(url: &str) -> Option<PullRequest> {

    let path = url.strip_prefix("https://github.com/")?;

    let pieces: Vec<&str> = path.split('/').collect(); 

    if pieces.len() < 4 || pieces[2] != "pull" {
        return None
    }

    let number: u32 = pieces[3].parse().ok()?;
    
    let result = PullRequest {
        owner: pieces[0].to_string(),
        repo: pieces[1].to_string(),
        number,
    };
        
    return Some(result)
    
}


pub async fn diff_from_pr(pr: &PullRequest) -> Result<String, Box <dyn std::error::Error>> {

    let access_token = std::env::var("GITHUB_ACCESS_TOKEN")?;

    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        pr.owner,
        pr.repo,
        pr.number
    );

    let client = reqwest::Client::new();

    let diff = client
        .get(&url)
        .bearer_auth(&access_token)
        .header("Accept", "application/vnd.github.diff")
        .header("User-Agent", "smorfs")
        .send().await?
        .error_for_status()?
        .text().await?;

    Ok(diff)

}

pub fn diff_from_branch(job: &Job) -> Result<String, Box<dyn std::error::Error>> {

    let cmd_string = format!("{}...{}", job.target_branch, job.source_branch);

    let output = Command::new("git")
        .args(["-C", job.repo_path.as_str(), "diff", cmd_string.as_str()])
        .output()?;

    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git diff failed: {}", message.trim()).into());
    }

    let diff = String::from_utf8(output.stdout)?;
    Ok(diff)
 
}

pub fn diff_from_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

pub fn split_by_file(diff: &str) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    let mut current = String::new();

    for line in diff.lines() {
        if line.starts_with("diff --git ") && !current.is_empty() {
            chunks.push(std::mem::take(&mut current));
        }
        current.push_str(line);
        current.push('\n');
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

pub fn extract_path_from_chunk(chunk: &str) -> Option<String> {
    let header = chunk.lines().next()?;
    let rest = header.strip_prefix("diff --git ")?;
    let (_, path) = rest.rsplit_once(" b/")?;

    Some(path.to_string())
}

pub fn remove_junk_files(diff: &str) -> String {
    let mut kept = String::new();

    for chunk in split_by_file(diff) {
        if let Some(path) = extract_path_from_chunk(&chunk) {
            if is_junk(&path) {
                continue;
            }
        }
        kept.push_str(&chunk);
    }
    kept
}
