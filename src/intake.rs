use crate::junk::is_junk;

pub fn diff_from_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn split_by_file(diff: &str) -> Vec<String> {
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

fn extract_path_from_chunk(chunk: &str) -> Option<String> {
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
