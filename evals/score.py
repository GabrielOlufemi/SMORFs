import json 
import subprocess
from pathlib import Path

CASES_DIR = Path("evals/cases")
BINARY = "target/debug/smorfs"
LINE_TOLERANCE = 3

def run_case(case_dir):
  patch = case_dir / "diff.patch"

  result = subprocess.run(
    [BINARY, str(patch)],
    capture_output=True,
    text=True
  )

  if result.returncode != 0:
    print(result.stderr)
    return None
  for line in result.stdout.splitlines():
    if line.startswith("saved "):
      return line.removeprefix("saved ")

  return None


# checks if line finding gap is significant
def matches(finding, bug):
  return finding["file"] == bug["file"] and abs(finding["line"] - bug["line"]) <= LINE_TOLERANCE


def score_case(run_path, expected_path):
  run = json.loads(Path(run_path).read_text())

  expected = json.loads(Path(expected_path).read_text())

  findings = run["findings"]
  bugs = expected["bugs"]

  bugs_found = 0
  for bug in bugs:
    if any(matches(f, bug) for f in findings):
      bugs_found += 1

  findings_hit = 0
  for f in findings:
    if any(matches(f, bug) for bug in bugs):
      findings_hit += 1

  return len(bugs), bugs_found, len(findings), findings_hit
  
  
      

def main():
  subprocess.run(["cargo", "build"], check=True)

  total_bugs = total_found = total_findings = total_hit = errors = 0

  print(f"{'case':<20} {'bugs found':>10} {'findings hit':>13}")

  for case_dir in sorted(CASES_DIR.iterdir()):
    run_path = run_case(case_dir)

    if run_path is None:
      print(f"{case_dir.name:<20} ERROR")
      errors += 1
      continue

    bugs, found, findings, hit = score_case(run_path, case_dir / "expected.json")

    print(f"{case_dir.name:<20} {found:>4} / {bugs:<4} {hit:>6} / {findings:<4}")

    total_bugs += bugs
    total_found += found
    total_findings += findings
    total_hit += hit

  recall = total_found / total_bugs if total_bugs else 0
  precision = total_hit / total_findings if total_findings else 0

  print()
  print(f"recall:    {total_found}/{total_bugs} = {recall:.0%}")
  print(f"precision: {total_hit}/{total_findings} = {precision:.0%}")
  print(f"errors:    {errors}")
if __name__ == "__main__":
  main()


  