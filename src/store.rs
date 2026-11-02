use crate::contracts::Run;
  
pub fn save_run(run: &Run) -> Result<String, Box<dyn std::error::Error>> {
  let json = serde_json::to_string_pretty(run)?;

  // creates run folder if non existent
  std::fs::create_dir_all("runs")?;

  let path = format!("runs/{}.json", run.id);
  std::fs::write(&path, json)?;

  Ok(path)
}























