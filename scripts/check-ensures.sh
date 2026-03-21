#!/usr/bin/env bash
# Enforce: every pub fn inside a verus! block must have an `ensures` clause.
# A pub fn without ensures has no verified postcondition — it's an unchecked hole.
#
# Exemptions: add `// @verified-logic:no-ensures <reason>` above the pub fn.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

FAIL=0
CHECKED=0
EXEMPT=0

for file in src/*.rs; do
    [[ "$file" == "src/lib.rs" ]] && continue

    mapfile -t lines < "$file"
    total=${#lines[@]}
    in_verus=0
    in_test_mod=0

    for ((n=0; n<total; n++)); do
        line="${lines[$n]}"

        # Track verus! block boundaries
        [[ "$line" =~ ^verus![[:space:]]*\{ ]] && in_verus=1
        [[ "$line" =~ ^\}[[:space:]]*//.verus ]] && in_verus=0

        # Track #[cfg(test)]
        [[ "$line" =~ '#[cfg(test)]' ]] && in_test_mod=1
        [[ $in_test_mod -eq 1 ]] && continue

        # Only check inside verus! blocks
        [[ $in_verus -eq 0 ]] && continue

        # Skip spec fn and proof fn
        [[ "$line" =~ ^[[:space:]]*(pub[[:space:]]+)?(open[[:space:]]+)?spec[[:space:]]+fn ]] && continue
        [[ "$line" =~ ^[[:space:]]*proof[[:space:]]+fn ]] && continue

        # Skip private helper fns
        if [[ "$line" =~ ^[[:space:]]*fn[[:space:]] ]] && ! [[ "$line" =~ ^[[:space:]]*pub[[:space:]] ]]; then
            continue
        fi

        # Match pub fn inside verus!
        if [[ "$line" =~ ^[[:space:]]*pub[[:space:]]+fn[[:space:]] ]]; then
            CHECKED=$((CHECKED + 1))
            line_num=$((n + 1))

            # Check for exemption in previous 5 lines
            is_exempt=0
            lookback=5
            start=$((n - lookback))
            [[ $start -lt 0 ]] && start=0
            for ((k=start; k<n; k++)); do
                [[ "${lines[$k]}" =~ @verified-logic:no-ensures ]] && is_exempt=1
            done

            if [[ $is_exempt -eq 1 ]]; then
                EXEMPT=$((EXEMPT + 1))
                continue
            fi

            # Scan forward for `ensures` before the opening `{`
            has_ensures=0
            for ((k=n; k<n+15 && k<total; k++)); do
                fwd="${lines[$k]}"
                [[ "$fwd" =~ ^[[:space:]]*ensures ]] && has_ensures=1 && break
                # Stop at opening brace of function body
                if [[ $k -gt $n ]] && [[ "$fwd" =~ ^\{ ]]; then
                    break
                fi
            done

            if [[ $has_ensures -eq 0 ]]; then
                fn_sig="$(echo "$line" | sed 's/^[[:space:]]*//')"
                echo "MISSING ensures: $file:$line_num: $fn_sig"
                FAIL=1
            fi
        fi
    done
done

echo ""
echo "Checked $CHECKED pub fn in verus! blocks, $EXEMPT exempted"

if [[ $FAIL -eq 0 ]]; then
    echo "✓ all verified pub fns have ensures clauses"
else
    echo ""
    echo "✗ pub fns in verus! blocks found without ensures"
    echo "  Fix: add 'ensures result == *_spec(args)' to the function"
    echo "  Or:  add '// @verified-logic:no-ensures <reason>' to exempt"
    exit 1
fi
