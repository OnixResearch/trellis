# Repo sync

Date: 2026-04-16

Pinned pre-change baseline revision: `decef04b0379` (`ruqnowsyupkn`, parent of the first choreography checkpoint)

Compared the baseline against:

- `src/lib.rs`
- `docs/requirements.md`
- `README.md`
- `openspec/specs/`

Commands and results:

```bash
$ jj file show -r decef04b0379 src/lib.rs | rg -n 'choreography_|pub mod properties;'
509:pub mod properties;

$ jj file show -r decef04b0379 docs/requirements.md | rg -n 'choreography\.|properties\.choreography-'
# no matches

$ jj file show -r decef04b0379 README.md | rg -n 'choreography\.|properties\.choreography-'
# no matches

$ for f in $(find openspec/specs -name spec.md | sort); do
    jj file show -r decef04b0379 "$f" 2>/dev/null | rg -n 'choreography\.|properties\.choreography-'
  done
# no matches
```

Conclusion:

- Baseline already had `pub mod properties;` in `src/lib.rs`.
- Baseline had no shipped module name starting with `choreography_`.
- Baseline had no shipped requirement ID starting with `choreography.`.
- Baseline had no shipped requirement ID starting with `properties.choreography-`.
- Current tree now adds `pub mod choreography_global;`, `pub mod choreography_local;`, `pub mod choreography_projection;`, and `pub mod choreography_step;` in `src/lib.rs`.
