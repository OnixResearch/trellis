# Web of Trust Primitives Specification

## Purpose

Defines requirements for a verified model of web-of-trust primitives:
canonical principal IDs, signed trust attestations, activation windows,
exact revocation, chain validity, rooted trust resolution, and a small
threshold policy over distinct introducers. Scope is the mathematical
trust-graph core, not packet formats, keyservers, or public-key
cryptography internals.

## Definitions

- **Principal**: a public-key identity represented by canonical key
  bytes and a derived principal ID.
- **Principal equality**: principal references in attestations and
  revocations carry canonical public-key bytes. Two principal references
  denote the same principal iff the full 32-byte principal IDs derived
  from those bytes are equal. Root membership, same-issuer checks,
  family matching, chain repetition checks, and threshold deduplication
  all use this equality rule.
- **Trust role**: an authenticated role field carried in the signed
  attestation payload. This change gives only one operational value,
  `trust`, encoded with role tag `0x01`, meaning that the attestation
  may contribute a trust edge. Other role-tag values are valid signed
  payload contents but are non-operational in this change.
- **Sequence metadata**: an unsigned 64-bit `sequence_no` carried in
  each attestation, compared by integer equality, and encoded as an
  8-byte little-endian integer in canonical payload bytes.
- **Attestation**: a signed issuer -> subject record carrying issuer,
  subject, trust role, validity window, and `sequence_no`.
- **Trust edge**: an attestation with role `trust` that is active at the
  evaluation time. Only trust edges participate in chain validity,
  rooted resolution, and threshold counting in this change.
- **Attestation family**: the set of attestations sharing the same
  `(issuer, subject, role)` tuple, even if their validity windows or
  sequence metadata differ.
- **Exact-payload target reference**: a revocation target naming one
  attestation by its full canonical signed payload bytes.
- **Family target reference**: a revocation target naming exactly one
  attestation family by `(issuer, subject, role)`.
- **Canonical byte grammar**: variable-length byte strings encode as a
  `u32` little-endian length prefix followed by raw bytes; times and
  `sequence_no` encode as `u64` little-endian integers; role tags and
  target-kind tags encode as single `u8` values. Any variable-length
  field longer than `u32::MAX` bytes is non-canonical and MUST be
  rejected.
- **Canonical attestation payload bytes**: the canonical-byte-grammar
  encoding of `(issuer_bytes, subject_bytes, role_tag, not_before,
  not_after, sequence_no)` in that exact order.
- **Canonical revocation payload bytes**: the canonical-byte-grammar
  encoding of `(revocation_issuer_bytes, target_kind_tag,
  target_encoding, effective_time)` in that exact order, where target
  kind `0x01` means `ExactPayload`, target kind `0x02` means `Family`,
  family targets encode `(issuer_bytes, subject_bytes, role_tag)`, and
  exact-payload targets encode one length-prefixed canonical attestation
  payload byte string.
- **Revocation**: an authenticated record issued by the same principal
  as the targeted attestation or targeted attestation family, and whose
  authenticated payload includes the target reference and effective time.
- **Effective revocation**: a same-issuer authenticated revocation
  whose target matches the attestation and whose effective time is at or
  before the evaluation time.
- **Active attestation**: an attestation whose signature is valid,
  whose evaluation time is inside its validity window, and which is not
  disabled by an effective revocation.
- **Trust root**: a principal supplied by the relying party as an
  initial trusted issuer.
- **Trust chain**: a simple path of active trust edges from a root to a
  subject.

## Requirements

### Requirement: Principal Fingerprint Determinism
r[web-of-trust.principal-fingerprint]

A principal identifier MUST be the full 32-byte BLAKE3 digest of the
principal's canonical public-key bytes. The same canonical bytes MUST
always produce the same identifier, and equality checks MUST use all 32
digest bytes rather than a truncated prefix.

#### Scenario: Same key bytes give same principal ID

- GIVEN one canonical public-key byte string `K`
- WHEN principal ID derivation runs twice on `K`
- THEN both derived principal IDs are equal

#### Scenario: Full digest comparison distinguishes principals

- GIVEN two principal IDs with the same prefix but different full digest bytes
- WHEN principal equality is checked
- THEN the principals are treated as distinct

### Requirement: Attestation Signature Binds Exact Payload
r[web-of-trust.attestation-signature]

An attestation signature MUST authenticate the issuer's canonical
public-key bytes, the subject's canonical public-key bytes, trust role,
validity window, and `sequence_no` exactly, using the canonical
attestation payload-byte grammar defined above. Changing any signed
field MUST invalidate the signature, and inputs whose canonical
byte-string fields exceed `u32::MAX` bytes MUST be rejected as
non-canonical.

#### Scenario: Exact payload verifies

- GIVEN an attestation payload and a matching issuer signature
- WHEN signature verification checks the canonical payload bytes
- THEN the attestation is accepted as authentic

#### Scenario: Tampered subject invalidates signature

- GIVEN a valid signed attestation from issuer `I` to subject `S`
- WHEN the subject field is changed to `S2` without resigning
- THEN signature verification fails

#### Scenario: Overlong identity bytes are rejected as non-canonical

- GIVEN an attestation input whose issuer or subject byte string is longer than `u32::MAX`
- WHEN canonical attestation payload bytes are requested
- THEN payload construction rejects the input as non-canonical

### Requirement: Active Attestation Window
r[web-of-trust.attestation-window]

An attestation MUST be active at time `t` iff its signature is valid,
`t` is inside the half-open window `[not_before, not_after)`, and no
effective revocation targets it at or before `t`.

#### Scenario: In-window unrevoked attestation is active

- GIVEN a valid signed attestation with `not_before <= t < not_after`
- AND no effective revocation targets it at time `t`
- WHEN activity is checked at time `t`
- THEN the attestation is active

#### Scenario: Expired or future attestation is inactive

- GIVEN a valid signed attestation
- WHEN activity is checked at `t < not_before` or `t >= not_after`
- THEN the attestation is inactive

### Requirement: Revocation Is Exact and Monotone
r[web-of-trust.revocation]

A revocation record MUST be authenticated over its issuer, target
reference, and effective time using the canonical revocation
payload-byte grammar defined above. Tampering with issuer, target
reference, or effective time MUST invalidate verification, and inputs
whose canonical byte-string fields exceed `u32::MAX` bytes MUST be
rejected as non-canonical. A revocation record MUST be issued by the
same principal as the targeted attestation or targeted attestation
family. A revocation record MUST carry either an exact-payload target
reference or a family target reference. An exact-payload target
identifies attestation content by canonical attestation payload bytes;
if multiple records carry identical canonical attestation payload bytes,
they are the same exact-payload target and the revocation disables all
such copies. A family target MUST disable exactly the attestations in
the named `(issuer, subject, role)` family. Once effective, the
revocation MUST remain effective for all later times.

#### Scenario: Exact-payload target disables one attestation

- GIVEN attestation `A` and a same-issuer authenticated revocation whose exact-payload target matches `A`
- WHEN evaluation time reaches the revocation time
- THEN `A` is inactive from that time onward

#### Scenario: Exact-payload target disables identical payload copies

- GIVEN two attestation records `A1` and `A2` with identical canonical attestation payload bytes
- WHEN a same-issuer authenticated exact-payload revocation for that payload becomes effective
- THEN both `A1` and `A2` are inactive from that time onward

#### Scenario: Family target disables reissued attestations in one family

- GIVEN two attestations `A1` and `A2` with the same `(issuer, subject, role)`
- AND different validity windows or `sequence_no` values
- WHEN a same-issuer authenticated family-target revocation for that `(issuer, subject, role)` becomes effective
- THEN both `A1` and `A2` are inactive from that time onward

#### Scenario: Tampered effective time invalidates revocation verification

- GIVEN a valid authenticated revocation for attestation `A`
- WHEN the effective-time field is changed without resigning
- THEN revocation verification fails

#### Scenario: Overlong revocation fields are rejected as non-canonical

- GIVEN a revocation input whose issuer byte string or length-prefixed exact-payload target byte string is longer than `u32::MAX`
- WHEN canonical revocation payload bytes are requested
- THEN payload construction rejects the input as non-canonical

#### Scenario: Wrong-issuer revocation does not disable the target

- GIVEN attestation `A`
- AND a revocation from some principal other than `A`'s issuer whose target text matches `A`
- WHEN activity for `A` is checked after that revocation time
- THEN `A` is unaffected because the revocation is not same-issuer authenticated

#### Scenario: Unrelated attestation stays unaffected

- GIVEN attestations `A` and `B` with a revocation targeting only `A` or only `A`'s family
- WHEN activity for `B` is checked after the revocation time
- THEN `B` is unaffected by that revocation

### Requirement: Trust Chains Compose Active Trust Edges
r[web-of-trust.chain-validity]

A trust chain MUST be valid only when every attestation in the chain is
an active trust edge at the evaluation time, the first issuer is a
trust root, each attestation's subject is the next attestation's issuer,
and no principal repeats in the chain.

#### Scenario: Active root-to-subject path is valid

- GIVEN a trust root `R`, an active trust edge `R -> A`, and an active trust edge `A -> B`
- WHEN chain validity is checked for `[R -> A, A -> B]`
- THEN the chain is valid

#### Scenario: Cycle makes chain invalid

- GIVEN active trust edges forming `R -> A -> R`
- WHEN chain validity is checked
- THEN the chain is rejected because a principal repeats

#### Scenario: Active non-`trust` attestation does not form a trust edge

- GIVEN a valid active attestation whose role tag is not `0x01`
- WHEN chain validity is checked using that attestation as an edge
- THEN the chain is rejected because the attestation is non-operational in this change

### Requirement: Rooted Trust Resolution Is Depth Bounded
r[web-of-trust.rooted-resolution]

For a fixed trust-root set `Roots` and fixed evaluation time `t`, a
rooted-trust decision for subject `S` under natural-number depth bound
`d` MUST return trusted iff either:
- `S` is itself in `Roots` and the witness is the empty chain, or
- there exists a valid trust chain at time `t` from some root in
  `Roots` to `S` of length at most `d`

#### Scenario: Root resolves at depth zero

- GIVEN trust root `R`
- WHEN rooted trust is checked for subject `R` with depth bound `0`
- THEN `R` resolves as trusted via the empty chain

#### Scenario: Direct root certification resolves at depth one

- GIVEN trust root `R` and an active trust edge `R -> S`
- WHEN rooted trust is checked for subject `S` with depth bound `1`
- THEN `S` resolves as trusted

#### Scenario: Existing valid chain must succeed

- GIVEN a valid chain `R -> A -> S`
- WHEN rooted trust is checked for subject `S` with depth bound `2`
- THEN `S` resolves as trusted

#### Scenario: Too-small depth bound rejects longer path

- GIVEN a valid chain `R -> A -> B -> S`
- WHEN rooted trust is checked for `S` with depth bound `2`
- THEN `S` does not resolve as trusted under that bound

### Requirement: Threshold Policy Counts Distinct Introducers
r[web-of-trust.threshold]

A threshold policy is parameterized by a natural `k`, an evaluation
time `t`, and a natural-number depth bound `d`. The policy MUST reject
when `k = 0`. For `k > 0`, it MUST accept subject `S` iff there exist at
least `k` distinct introducers `I` such that:
- `d > 0`
- `I` itself resolves as trusted at time `t` under bound `d - 1`
- `I` has an active direct trust edge to `S` at time `t`
- appending that final direct edge yields a valid simple trust chain to
  `S`, so repeated-principal final hops MUST NOT count

Multiple active attestations from the same introducer to the same
subject MUST contribute at most one count. When `d = 0`, the threshold
policy MUST reject every subject because no final trust-edge hop fits
inside the bound. Threshold policy does not special-case root subjects;
a root subject is accepted only if the same `k`-introducer rule is met.

#### Scenario: Distinct trust roots satisfy threshold at depth one

- GIVEN trust roots `A` and `B`
- AND active trust edges `A -> S` and `B -> S`
- WHEN a `k = 2` threshold policy is evaluated for subject `S` at depth `1`
- THEN the policy accepts `S`

#### Scenario: Zero threshold rejects

- GIVEN any subject `S`
- WHEN a `k = 0` threshold policy is evaluated for `S`
- THEN the policy rejects `S`

#### Scenario: Depth zero rejects subject

- GIVEN trust root `R` and an active trust edge `R -> S`
- WHEN a `k = 1` threshold policy is evaluated for subject `S` at depth `0`
- THEN the policy rejects `S`

#### Scenario: Root subject still needs introducers under threshold

- GIVEN subject `R` that is itself a trust root
- AND no qualifying introducer edges into `R`
- WHEN a `k = 1` threshold policy is evaluated for subject `R`
- THEN the policy rejects `R`

#### Scenario: Too-small depth bound rejects indirect introducer

- GIVEN trust root `R`, an active trust edge `R -> A`, and an active trust edge `A -> S`
- WHEN a `k = 1` threshold policy is evaluated for subject `S` at depth `1`
- THEN the policy rejects `S` because `A` is not rooted-trusted under bound `0`

#### Scenario: Repeated-principal final hop does not count

- GIVEN a rooted-trusted introducer candidate `I`
- AND an active direct trust edge whose final hop would repeat a principal already on the witness chain or would use `I = S`
- WHEN threshold counting is evaluated
- THEN that candidate does not contribute to the threshold count

#### Scenario: Duplicate attestations do not double count

- GIVEN rooted-trusted introducer `A`
- AND two active trust edges from `A` to `S`
- WHEN a `k = 2` threshold policy is evaluated for subject `S`
- THEN the policy rejects `S`

## Cross-Module Properties

### Requirement: Revocation Cuts Derived Trust
r[properties.web-of-trust.revocation-cuts-trust]

For fixed trust roots and fixed evaluation times, if a successful
rooted-trust witness chain `C` for subject `S` includes attestation `A`,
then once `A` is effectively revoked, that same witness chain `C` MUST
no longer justify trust for `S` at later times. This requirement is
witness-local and does not forbid a different valid chain from still
trusting `S`.

### Requirement: Resolution Is Monotone in Depth
r[properties.web-of-trust.depth-monotone]

For a fixed trust-root set and fixed evaluation time, if a subject
resolves as trusted under depth bound `d`, then it MUST also resolve as
trusted under every depth bound `d' >= d`.

### Requirement: Threshold Deduplicates Introducers
r[properties.web-of-trust.threshold-dedup]

For fixed trust roots, evaluation time, depth bound, threshold `k`, and
subject `S`, once one active direct trust edge from introducer `I` to
`S` has already been counted, any later active direct trust edge from
that same introducer to `S` MUST leave both the remaining qualifying
distinct-introducer count and the final threshold accept/reject result
unchanged. Equivalently, replacing multiple such same-introducer final
hops with a single counted edge MUST not change the threshold outcome.
