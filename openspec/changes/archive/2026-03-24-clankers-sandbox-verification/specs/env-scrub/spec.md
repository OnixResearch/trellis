# Environment Scrubbing Specification

## Purpose

Defines the correctness properties for environment variable
sanitization before spawning child processes. Extracted from the
`clankers` sandbox policy module. Prevents credential leakage
through the bash tool's child process environment.

## Definitions

- **Environment**: A sequence of `(key, value)` pairs where keys
  and values are byte sequences.
- **Blocklist**: A fixed set of known secret variable names
  (e.g., `ANTHROPIC_API_KEY`, `GITHUB_TOKEN`, `AWS_SECRET_ACCESS_KEY`).
- **Secret suffix**: One of 7 uppercase suffixes that heuristically
  identify secret variables: `_SECRET`, `_TOKEN`, `_PASSWORD`,
  `_CREDENTIALS`, `_API_KEY`, `_APIKEY`, `_PRIVATE_KEY`.
- **Sanitized environment**: The input environment with all
  blocked and suffix-matched variables removed, plus a sandbox
  marker variable injected.

## Requirements

### Requirement: Blocklisted Keys Absent

Every key in the explicit blocklist MUST be absent from the
sanitized output.

For all `k` in `BLOCKLIST`: `k` not in `output_keys(sanitize(env))`.

#### Scenario: AWS key stripped

- GIVEN env contains `("AWS_SECRET_ACCESS_KEY", "secret123")`
- WHEN sanitizing
- THEN the output does not contain key `AWS_SECRET_ACCESS_KEY`

#### Scenario: GitHub token stripped

- GIVEN env contains `("GITHUB_TOKEN", "ghp_abc")`
- WHEN sanitizing
- THEN the output does not contain key `GITHUB_TOKEN`

#### Scenario: SSH auth sock stripped

- GIVEN env contains `("SSH_AUTH_SOCK", "/tmp/agent.123")`
- WHEN sanitizing
- THEN the output does not contain key `SSH_AUTH_SOCK`

### Requirement: Suffix-matched Keys Absent

Every key whose uppercased form ends with a secret suffix MUST
be absent from the sanitized output.

For all `k` where `has_secret_suffix(upper(k))`:
`k` not in `output_keys(sanitize(env))`.

#### Scenario: Custom API key suffix

- GIVEN env contains `("MY_CUSTOM_API_KEY", "key123")`
- WHEN sanitizing
- THEN the output does not contain key `MY_CUSTOM_API_KEY`

#### Scenario: Mixed case suffix

- GIVEN env contains `("my_App_Secret", "s")`
- WHEN sanitizing (uppercased check: `MY_APP_SECRET` ends with `_SECRET`)
- THEN the output does not contain key `my_App_Secret`

#### Scenario: Token suffix

- GIVEN env contains `("DEPLOY_TOKEN", "t")`
- WHEN sanitizing
- THEN the output does not contain key `DEPLOY_TOKEN`

#### Scenario: Password suffix

- GIVEN env contains `("DB_PASSWORD", "p")`
- WHEN sanitizing
- THEN the output does not contain key `DB_PASSWORD`

#### Scenario: Credentials suffix

- GIVEN env contains `("CLOUD_CREDENTIALS", "c")`
- WHEN sanitizing
- THEN the output does not contain key `CLOUD_CREDENTIALS`

#### Scenario: Private key suffix

- GIVEN env contains `("SIGNING_PRIVATE_KEY", "k")`
- WHEN sanitizing
- THEN the output does not contain key `SIGNING_PRIVATE_KEY`

### Requirement: Non-secret Keys Preserved

Every key that is NOT in the blocklist and does NOT match a
secret suffix MUST be present in the sanitized output with its
original value.

For all `(k, v)` in input where `!is_blocked(k)` and
`!has_secret_suffix(upper(k))`:
`(k, v)` is in `sanitize(env)`.

#### Scenario: PATH preserved

- GIVEN env contains `("PATH", "/usr/bin")`
- WHEN sanitizing
- THEN the output contains `("PATH", "/usr/bin")`

#### Scenario: HOME preserved

- GIVEN env contains `("HOME", "/home/user")`
- WHEN sanitizing
- THEN the output contains `("HOME", "/home/user")`

#### Scenario: LANG preserved

- GIVEN env contains `("LANG", "en_US.UTF-8")`
- WHEN sanitizing
- THEN the output contains `("LANG", "en_US.UTF-8")`

### Requirement: Sandbox Marker Present

The sanitized output MUST contain the sandbox marker variable
`("CLANKERS_SANDBOX", "1")`.

#### Scenario: Marker injected

- GIVEN any input environment
- WHEN sanitizing
- THEN the output contains `("CLANKERS_SANDBOX", "1")`

### Requirement: Filter Idempotent

Applying `sanitize` twice MUST produce the same result as
applying it once.

`sanitize(sanitize(env)) == sanitize(env)` for all `env`.

The sandbox marker key `CLANKERS_SANDBOX` does not match any
blocklist entry or secret suffix, so it survives re-filtering.

#### Scenario: Double filter

- GIVEN any input environment
- WHEN sanitizing twice
- THEN the result equals sanitizing once

### Requirement: Suffix Check Is Case-insensitive

The secret suffix check MUST uppercase the key before comparing.
`my_app_secret` and `MY_APP_SECRET` and `My_App_Secret` MUST
all be treated as matching `_SECRET`.

#### Scenario: Lowercase key

- GIVEN env contains `("my_app_secret", "s")`
- WHEN sanitizing
- THEN the key is stripped (uppercased form ends with `_SECRET`)

### Requirement: Empty Environment Yields Marker Only

Sanitizing an empty environment MUST return exactly the sandbox
marker.

#### Scenario: Empty input

- GIVEN an empty environment
- WHEN sanitizing
- THEN the output is `[("CLANKERS_SANDBOX", "1")]`

### Requirement: Blocklist Is Disjoint from Safe Keys

No key in the blocklist matches a common safe variable name
(PATH, HOME, USER, LANG, TERM, SHELL, EDITOR). This is a
structural property of the blocklist itself, not a runtime check.

#### Scenario: PATH not in blocklist

- GIVEN the blocklist
- WHEN checking membership of "PATH"
- THEN PATH is not in the blocklist

### Requirement: Output Size Bounded

The output length MUST be at most `input.len() + 1` (the +1 is
the injected sandbox marker). Filtering can only remove entries.

#### Scenario: All secrets stripped

- GIVEN an env of 10 entries, all secret
- WHEN sanitizing
- THEN the output has exactly 1 entry (the sandbox marker)
