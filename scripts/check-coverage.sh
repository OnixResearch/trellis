#!/usr/bin/env bash
# Enforce: every pub fn in src/ must have an r[impl ...] Tracey annotation.
# This catches unannotated public functions that bypass the verified pattern.
#
# Exemptions: add `// @verified-logic:exempt <reason>` above the pub fn.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

FAIL=0
CHECKED=0
EXEMPT=0

for file in src/*.rs; do
    [[ "$file" == "src/lib.rs" ]] && continue

    # Read file into array for lookback
    mapfile -t lines < "$file"
    total=${#lines[@]}
    in_test_mod=0

    for ((n=0; n<total; n++)); do
        line="${lines[$n]}"

        # Track #[cfg(test)] — stop checking after this
        if [[ "$line" =~ ^[[:space:]]*'#[cfg(test)]' ]]; then
            in_test_mod=1
        fi
        [[ $in_test_mod -eq 1 ]] && continue

        # Skip proof fn and spec fn
        [[ "$line" =~ ^[[:space:]]*(pub[[:space:]]+)?(open[[:space:]]+)?spec[[:space:]]+fn ]] && continue
        [[ "$line" =~ ^[[:space:]]*proof[[:space:]]+fn ]] && continue

        # Skip private fns
        if [[ "$line" =~ ^[[:space:]]*fn[[:space:]] ]] && ! [[ "$line" =~ ^[[:space:]]*pub[[:space:]] ]]; then
            continue
        fi

        # Match pub fn
        if [[ "$line" =~ ^[[:space:]]*pub[[:space:]]+fn[[:space:]] ]]; then
            CHECKED=$((CHECKED + 1))
            line_num=$((n + 1))

            # Look back up to 10 lines for r[impl ...] or @verified-logic:exempt
            has_impl=0
            is_exempt=0
            lookback=10
            start=$((n - lookback))
            [[ $start -lt 0 ]] && start=0

            for ((k=start; k<n; k++)); do
                prev="${lines[$k]}"
                if [[ "$prev" =~ r\[impl[[:space:]] ]]; then
                    has_impl=1
                    break
                fi
                if [[ "$prev" =~ @verified-logic:exempt ]]; then
                    is_exempt=1
                    break
                fi
                # Stop lookback at a blank line or closing brace (different block)
                if [[ -z "${prev// /}" ]] || [[ "$prev" =~ ^[[:space:]]*\} ]]; then
                    # Reset — we crossed a block boundary
                    has_impl=0
                    is_exempt=0
                fi
            done

            if [[ $is_exempt -eq 1 ]]; then
                EXEMPT=$((EXEMPT + 1))
                continue
            fi

            if [[ $has_impl -eq 0 ]]; then
                fn_sig="$(echo "$line" | sed 's/^[[:space:]]*//')"
                echo "MISSING r[impl]: $file:$line_num: $fn_sig"
                FAIL=1
            fi
        fi
    done
done

echo ""
echo "Checked $CHECKED pub fn declarations, $EXEMPT exempted"

if [[ $FAIL -eq 0 ]]; then
    echo "✓ all pub fns have r[impl] annotations"
else
    echo ""
    echo "✗ pub fns found without r[impl] annotations"
    echo "  Fix: add '// r[impl requirement.id]' above the function"
    echo "  Or:  add '// @verified-logic:exempt <reason>' to exempt"
    exit 1
fi
