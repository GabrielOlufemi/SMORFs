import json 
import subprocess
from pathlib import Path

CASE_DIR = Path("evals/cases")
BINARY = "target/debug/smorfs"

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
  

def main():
  subprocess.run(["cargo", "build"], check=True)

  for case_dir in sorted(CASE_DIR.iterdir()):
    run_path = run_case(case_dir)

    if run_path is None: 
      print(f"{case_dir.name:<20} ERROR")
    else:
      print(f"{case_dir.name:<20} {run_path}")
      
if __name__ == "__main__":
  main()


  