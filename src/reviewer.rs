
use serde::{Serialize, Deserialize};
use crate::contracts::Finding;

// what we send to openrouter
#[derive(Serialize)]
struct ChatRequest {
  model: String,
  temperature: f32,
  messages: Vec<RequestMessage>
}

#[derive(Serialize)]
struct RequestMessage {
  role: String,
  content: String,
}


// openrouter response
#[derive(Deserialize)]
struct ChatResponse {
  choices: Vec<Choice>,
  usage: Usage,
}

#[derive(Deserialize)]
struct Choice {
  message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
  content: String,
}

#[derive(Deserialize)]
struct Usage {
  prompt_tokens: u32,
  completion_tokens: u32,
}

// to aid us storing details upon functon end
pub struct ReviewResult {
  pub findings: Vec<Finding>,
  pub prompt_tokens: u32,
  pub completion_tokens: u32,
}

// what model wrote within Content (findings appaz)
#[derive(Deserialize)]
struct ReviewOutput {
  findings: Vec<Finding>
}

pub fn review(diff: &str, prompt_path: &str) -> Result<ReviewResult, Box<dyn std::error::Error>> {
  
  // retrieving env params
  let api_key = std::env::var("OPENROUTER_API_KEY")?;
  let model = std::env::var("OPENROUTER_MODEL")?;
  let temperature: f32 = std::env::var("OPENROUTER_TEMPERATURE")?.parse()?;

  // // loading prompt from generalist.md 
  // let prompt = std::fs::read_to_string("prompts/generalist.md")?;

  // loading from prompt_path
  let prompt = std::fs::read_to_string(prompt_path)
    .map_err(|e| format!("couldn't read prompt {prompt_path} ({e})"))?;

  // build chat request
  let request = ChatRequest {
    model: model.clone(),
    temperature: temperature,
    messages: vec![
      RequestMessage {
        role: "system".to_string(),
        content: prompt,
      }, 
      RequestMessage {
        role: "user".to_string(),
        content: diff.to_string(),
      },
    ],
  };

  // client p
  let client = reqwest::blocking::Client::builder()
    .timeout(std::time::Duration::from_secs(120))
    .build()?;

  let response: ChatResponse = client
    .post("https://openrouter.ai/api/v1/chat/completions")
    .bearer_auth(&api_key)
    .json(&request)
    .send()?
    .error_for_status()?
    .json()?;

  // json conntent response
  let content = response
    .choices
    .into_iter()
    .next()
    .ok_or("model returned no choices")?
    .message
    .content;

  let json_text = content
    .trim()
    .trim_start_matches("```json")
    .trim_start_matches("```")
    .trim_end_matches("```")
    .trim();

  let output: ReviewOutput = serde_json::from_str(json_text).map_err(|e| format!("model reply wasn't valid JSON ({e}):\n{content}"))?;

  Ok(ReviewResult {
    findings:  output.findings,
    prompt_tokens: response.usage.prompt_tokens,
    completion_tokens: response.usage.completion_tokens,
  })
  
}