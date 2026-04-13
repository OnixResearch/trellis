$ rg -n "verification results::" openspec/changes/archive/2026-04-13-liveness-analysis/evidence/verus-lib.txt | tail -n 1
2065:verification results:: 6380 verified, 0 errors

$ rg -n "test result: ok\. 3474 passed" openspec/changes/archive/2026-04-13-liveness-analysis/evidence/cargo-test.txt | tail -n 1
3483:test result: ok. 3474 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.20s

$ cat openspec/changes/archive/2026-04-13-liveness-analysis/evidence/tracey-uncovered.txt
verified-logic/rust: 0 uncovered out of 1387 rules

---
→ Run `tracey query rule <rule-id>`
→ Run `tracey query uncovered --prefix <prefix>`


$ cat openspec/changes/archive/2026-04-13-liveness-analysis/evidence/tracey-untested.txt
verified-logic/rust: 0 untested (impl but no verify) out of 1387 rules

---
→ Run `tracey query rule <rule-id>`
→ Run `tracey query untested --prefix <prefix>`


$ python - <<"PY"
from pathlib import Path
mods = sum(1 for line in Path("src/lib.rs").read_text().splitlines() if line.startswith("pub mod "))
print(f"modules={mods}")
PY
modules=296

$ rg -n "#\[verifier::external_body\]" src | wc -l
15

$ python - <<"PY"
from pathlib import Path
reqs = sum(1 for line in Path("docs/requirements.md").read_text().splitlines() if line.startswith("r["))
print(f"raw_requirement_lines={reqs}")
PY
raw_requirement_lines=1410
