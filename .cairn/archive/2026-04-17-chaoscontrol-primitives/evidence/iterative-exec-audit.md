# Iterative-Exec Audit

```
$ python3 scripts/check-iterative-exec.py src/chaoscontrol_scheduler.rs src/chaoscontrol_fault_schedule.rs
result: PASS
```

- `src/chaoscontrol_scheduler.rs`: 17 exec functions, 0 self-edges, 0 cycles
- `src/chaoscontrol_fault_schedule.rs`: 15 exec functions, 0 self-edges, 0 cycles

All intra-module exec helper calls use bare local function names.
