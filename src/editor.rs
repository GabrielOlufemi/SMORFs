use std::collections::{HashMap, HashSet};
use crate::intake::{split_by_file, extract_path_from_chunk};
use crate::contracts::Finding;


pub fn filter_findings(diff: &str, findings: Vec<Finding>) -> Vec<Finding> {
  let map = valid_lines(&diff);

  let mut kept = Vec::new();

  for finding in findings {

    let is_valid = match map.get(&finding.file) {
      Some(set) => set.contains(&finding.line),
      None => false,
    };

    if is_valid {
      kept.push(finding);
    } else {
      println!("dropped: {}:{} (not in diff)", finding.file, finding.line);
    }
  }

  kept
} 

fn valid_lines(diff: &str) -> HashMap<String, HashSet<u32>> {
  // make an empty map
  let mut store = HashMap::new();

  for chunk in split_by_file(diff) {
    let Some(path) = extract_path_from_chunk(&chunk) else {continue;};

    let mut set = HashSet::new();

    for line in chunk.lines() {
      if let Some((start, count)) = parse_new_range(line) {
        set.extend(start..start + count);
      }
    }    
    store.insert(path, set);
  }
  store
}

fn parse_new_range(header: &str) -> Option<(u32, u32)> {
  // 1. strip the "@@ " off the front (not a header? return None)
  let rest = header.strip_prefix("@@ ")?;
  // 2. split what's left on spaces, find the piece that starts with '+'
  let new_part = rest.split_whitespace().find(|piece| piece.starts_with('+'))?;
  // 3. drop the '+'           → "27,3"   or   "1"
  let clean_new_part = new_part.strip_prefix('+')?;
  // 4. split on ','           → start = "27", count = "3"   (no comma → count is 1)
  let (start, count) = match clean_new_part.split_once(',') {
    Some((start, count)) => (start, count),
    None => (clean_new_part, "1"),
  };
  
  // 5. turn both into u32 and return Some((start, count))
  let start: u32 = start.parse().ok()?;
  let count: u32 = count.parse().ok()?;

  Some((start, count))
}

// how far apart two findings' lines can be and still count as the same spot
const SAME_SPOT_LINES: u32 = 3;

pub fn merge_duplicates(findings: Vec<Finding>) -> Vec<Finding> {
  // each group holds findings that point at the same spot
  let mut groups: Vec<Vec<Finding>> = Vec::new();

  for finding in findings {
    // find a group with any finding at the same spot as this one
    let existing = groups
      .iter_mut()
      .find(|group| group.iter().any(|kept| same_spot(kept, &finding)));

    match existing {
      Some(group) => group.push(finding),
      None => groups.push(vec![finding]),
    }
  }

  groups.into_iter().map(combine).collect()
}


pub fn rank_by_severity(mut findings: Vec<Finding>) -> Vec<Finding>{

  findings.sort_by_key(|f| f.severity);

  findings
}



fn same_spot(a: &Finding, b: &Finding) -> bool {
  a.file == b.file && a.line.abs_diff(b.line) <= SAME_SPOT_LINES
}

// turns a group into one finding: the most severe one leads, everyone's name and message is kept
fn combine(mut group: Vec<Finding>) -> Finding {
  if group.len() == 1 {
    return group.remove(0);
  }

  // High < Medium < Low, so the most severe comes first. ties keep task order
  group.sort_by_key(|f| f.severity);

  let reviewers: Vec<String> = group.iter().map(|f| f.reviewer.clone()).collect();
  let messages: Vec<String> = group
    .iter()
    .map(|f| format!("[{}] {}", f.reviewer, f.message))
    .collect();

  println!("merged: {}:{} ({})", group[0].file, group[0].line, reviewers.join(", "));

  let mut winner = group.remove(0);
  winner.reviewer = reviewers.join(", ");
  winner.message = messages.join("\n");
  winner
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_new_range() {
    assert_eq!(
      parse_new_range("@@ -27,5 +27,3 @@ fn check()"), Some((27,3))
    );

    assert_eq!(
      parse_new_range("@@ -1 +1 @@"), Some((1,1))
    );

    assert_eq!(
      parse_new_range("@@ -10,2 +9,0 @@"), Some((9, 0))
    );

    assert_eq!(parse_new_range("let x = 5;"), None);
  }

  #[test]
  fn collects_valid_lines_per_file() {
        let diff = "\
diff --git a/src/a.rs b/src/a.rs
@@ -1,2 +1,3 @@
 line
+new
 line
@@ -20,1 +21,2 @@
+x
 y
diff --git a/src/b.rs b/src/b.rs
@@ -5 +5 @@
-old
+new
";

    let lines = valid_lines(diff);

    assert_eq!(lines["src/a.rs"], HashSet::from([1,2,3,21,22]));
    assert_eq!(lines["src/b.rs"], HashSet::from([5]));

  }
}