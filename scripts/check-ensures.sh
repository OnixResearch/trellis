#!/usr/bin/env bash
# Enforce: every pub fn inside a verus! block must have an `ensures` clause.
# A pub fn without ensures has no verified postcondition — it's an unchecked hole.
#
# Exemptions: add `// @verified-logic:no-ensures <reason>` above the pub fn.
set -euo pipefail

cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)"

python - <<'PY'
from pathlib import Path
import re
import sys

FAIL = 0
CHECKED = 0
EXEMPT = 0

verus_start = re.compile(r"^\s*verus!\s*\{")
verus_end = re.compile(r"^\s*\}\s*//\s*verus!?\s*$")
cfg_test = re.compile(r"#\[\s*cfg\s*\(\s*test\s*\)\s*\]")
spec_fn = re.compile(r"^\s*(pub\s+)?(open\s+)?spec\s+fn\b")
proof_fn = re.compile(r"^\s*proof\s+fn\b")
private_fn = re.compile(r"^\s*fn\s+\w")
pub_fn = re.compile(r"^\s*pub\s+fn\b")

for file in sorted(Path("src").glob("*.rs")):
    if file.name == "lib.rs":
        continue

    lines = file.read_text().splitlines()
    in_verus = False
    in_test_mod = False

    for n, line in enumerate(lines):
        if verus_start.match(line):
            in_verus = True

        if cfg_test.search(line):
            in_test_mod = True
        if in_test_mod:
            if verus_end.match(line):
                in_verus = False
            continue

        if not in_verus:
            if verus_end.match(line):
                in_verus = False
            continue

        if spec_fn.match(line) or proof_fn.match(line):
            if verus_end.match(line):
                in_verus = False
            continue

        if private_fn.match(line) and not pub_fn.match(line):
            if verus_end.match(line):
                in_verus = False
            continue

        if pub_fn.match(line):
            CHECKED += 1
            line_num = n + 1

            start = max(0, n - 5)
            is_exempt = any("@verified-logic:no-ensures" in lines[k] for k in range(start, n))
            if is_exempt:
                EXEMPT += 1
                if verus_end.match(line):
                    in_verus = False
                continue

            has_ensures = False

            for fwd in lines[n:]:
                code = fwd.split("//", 1)[0]
                stripped = code.strip()
                if re.search(r"\bensures\b", code):
                    has_ensures = True
                if stripped.startswith("{"):
                    break

            if not has_ensures:
                fn_sig = line.lstrip()
                print(f"MISSING ensures: {file}:{line_num}: {fn_sig}")
                FAIL = 1

        if verus_end.match(line):
            in_verus = False

print()
print(f"Checked {CHECKED} pub fn in verus! blocks, {EXEMPT} exempted")

if FAIL == 0:
    print("✓ all verified pub fns have ensures clauses")
else:
    print()
    print("✗ pub fns in verus! blocks found without ensures")
    print("  Fix: add 'ensures result == *_spec(args)' to the function")
    print("  Or:  add '// @verified-logic:no-ensures <reason>' to exempt")
    sys.exit(1)
PY
