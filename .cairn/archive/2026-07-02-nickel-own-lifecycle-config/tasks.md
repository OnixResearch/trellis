## Phase 1: Nickel lifecycle config source

- [x] [serial] Add a typed Nickel lifecycle config source that represents the current Trellis `cairn/config.yaml` project metadata, domains, context, schema, and proposal/task rules. r[verification-policy.nickel-owned-lifecycle-config]
- [x] [depends:lifecycle-source] Define contracts for non-empty project fields, non-empty domains, supported schema values, non-empty context, and non-empty rule strings. r[verification-policy.nickel-owned-lifecycle-config]

## Phase 2: Generated compatibility artifacts

- [x] [depends:lifecycle-source] Generate compatibility outputs for `cairn/config.yaml` and `openspec/config.yaml` from the Nickel source without changing lifecycle semantics. r[verification-policy.nickel-owned-lifecycle-config]
- [x] [depends:generated-config] Add a freshness checker that typechecks Nickel, exports the expected compatibility representation, and rejects stale generated config artifacts. r[verification-policy.nickel-owned-lifecycle-config]

## Phase 3: Positive and negative fixtures

- [x] [depends:freshness-checker] Add valid lifecycle config fixtures and invalid fixtures for missing metadata, empty domains, unsupported schema, malformed rules, and unsafe generated paths. r[verification-policy.lifecycle-config-negative-fixtures]
- [x] [depends:fixtures] Cover the fixture checker with positive and negative self-tests. r[verification-policy.lifecycle-config-negative-fixtures]

## Phase 4: Integration and validation

- [x] [depends:fixtures] Integrate lifecycle config freshness into the repo verification wrapper or the smallest existing policy-check command that runs under full verification. r[verification-policy.nickel-owned-lifecycle-config]
- [x] [depends:integration] Run Nickel config checks, fixture checks, Cairn validation/gates, and the repo verification wrapper; archive command output before completing the change. r[verification-policy.lifecycle-config-negative-fixtures]
