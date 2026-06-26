# Rust Primitives Specification

## Purpose

Specify finite, mechanically checked Rust primitives whose executable admission helpers refine explicit Verus specs. Async-adjacent primitives avoid modeling callbacks, OS wake delivery, scheduler fairness, or destructor side effects; circuit primitives stop at deterministic constraint admission and do not claim proving-system soundness.

## Requirements

### Requirement: Async Task State Lifecycle

The system MUST expose a finite Rust async task-state primitive with explicit pending, ready, completed, cancelled, and terminal states. Admission MUST reject transitions that mutate a terminal completed or cancelled task, and executable helpers MUST refine the spec predicate rather than encoding a separate runtime policy.

r[rust-primitives.async-task-state]

#### Scenario: Ready task can complete

- GIVEN an admitted task in the ready state
- WHEN completion admission is evaluated
- THEN the successor state is completed and marked terminal

#### Scenario: Completed task rejects later cancellation

- GIVEN an admitted task in the completed terminal state
- WHEN cancellation admission is evaluated
- THEN the transition is rejected and the completed state is unchanged

### Requirement: Task Waker Slot

The system MUST expose a finite task-waker slot primitive for empty, registered, notified, and taken states. Waker registration MUST replace only the current slot generation, wake MUST be idempotent for an already-notified generation, and taking a notification MUST consume at most one pending notification.

r[rust-primitives.task-waker]

#### Scenario: Wake after registration is observable

- GIVEN an empty waker slot that registers generation `g`
- WHEN wake admission is evaluated for generation `g`
- THEN the slot records a pending notification for `g`

#### Scenario: Repeated wake is idempotent

- GIVEN a waker slot already notified for generation `g`
- WHEN wake admission is evaluated again for generation `g`
- THEN the slot remains notified without increasing the number of consumable notifications

#### Scenario: Replaced waker ignores stale generation

- GIVEN a waker slot registered at generation `g2` after replacing generation `g1`
- WHEN wake admission is evaluated for stale generation `g1`
- THEN the stale wake is rejected or leaves the `g2` slot unchanged

### Requirement: No Lost Wake Bridge

The system MUST define a no-lost-wake bridge between task readiness and the waker slot: if readiness is admitted after a generation is registered, then either the slot is notified for that generation or the task is already observed as ready by the consumer. The bridge MUST NOT claim callback execution, scheduler fairness, or OS wake delivery.

r[rust-primitives.no-lost-wake]

#### Scenario: Registered waiter observes later readiness

- GIVEN a pending task with a registered waker generation
- WHEN readiness is admitted after the registration
- THEN the model records either a notification for that generation or an already-ready observation

#### Scenario: Ready-before-register does not require wake

- GIVEN a task that is already ready before a waker is registered
- WHEN registration admission is evaluated
- THEN the model may accept registration without requiring a new wake notification

### Requirement: Oneshot Channel State

The system MUST expose a single-slot oneshot channel primitive with open, sent, received, closed, and cancelled states. Sending MUST succeed at most once, receiving MUST consume at most one sent value, closing MUST reject future sends, and cancellation MUST preserve the no-duplicate-receive invariant.

r[rust-primitives.oneshot-channel]

#### Scenario: Send then receive consumes one value

- GIVEN an open oneshot channel
- WHEN send admission succeeds and receive admission is evaluated afterward
- THEN the channel transitions to received and no second receive can consume the same value

#### Scenario: Closed channel rejects send

- GIVEN a closed oneshot channel
- WHEN send admission is evaluated
- THEN the send is rejected and the channel remains closed

#### Scenario: Cancelled receiver prevents duplicate delivery

- GIVEN a oneshot channel whose receiver side is cancelled before receiving
- WHEN later receive admission is evaluated
- THEN no value is delivered twice and the cancelled terminal state is preserved

### Requirement: Oneshot Readiness Notification

The system MUST compose oneshot send admission with task readiness and waker-slot notification. A successful send to an open channel with a registered receiver waker MUST make the receiver ready and satisfy the no-lost-wake bridge; a failed send MUST NOT create receiver readiness evidence.

r[rust-primitives.oneshot-readiness]

#### Scenario: Successful send wakes registered receiver

- GIVEN an open oneshot channel with a registered receiver waker
- WHEN send admission succeeds
- THEN receiver readiness is admitted and the registered generation is notified or already observed ready

#### Scenario: Failed send does not wake receiver

- GIVEN a closed or already-sent oneshot channel
- WHEN send admission is rejected
- THEN no new receiver readiness or wake evidence is produced by that rejected send

### Requirement: Drop Guard Exactly-Once Cleanup

The system MUST expose a Rust RAII-style drop-guard primitive with armed, disarmed, cleaned, and terminal states. Cleanup admission MUST run at most once for an armed guard, disarm MUST prevent later cleanup, and all terminal states MUST be stable under repeated cleanup attempts.

r[rust-primitives.drop-guard]

#### Scenario: Armed guard cleans once

- GIVEN an armed guard
- WHEN cleanup admission is evaluated
- THEN the guard transitions to cleaned and later cleanup attempts are rejected or no-op terminal transitions

#### Scenario: Disarmed guard skips cleanup

- GIVEN an armed guard that is explicitly disarmed
- WHEN cleanup admission is evaluated afterward
- THEN cleanup is not admitted and the disarmed terminal state is preserved

### Requirement: Rust Primitive Verification Coverage

The system MUST provide `r[impl]` and `r[verify]` coverage for every public helper introduced by this change. Tests MUST include positive and negative cases for each primitive, and proof functions MUST connect executable helper results to the corresponding spec predicates before README/catalog entries claim the module is verified.

r[rust-primitives.verification-coverage]

#### Scenario: Public helper has traceable requirement coverage

- GIVEN a public function in one of the Rust async primitive modules
- WHEN repo-wide verification coverage checks are run
- THEN the function has explicit `r[impl]` coverage and matching requirement text

#### Scenario: Negative transition test is required

- GIVEN a primitive that rejects at least one invalid state transition
- WHEN focused tests for that module are reviewed
- THEN at least one negative test covers the rejected transition behavior

### Requirement: R1CS Row Shape

The system MUST expose a finite single-row R1CS envelope with explicit row length, witness length, modulus, three coefficient slots for each of `A`, `B`, and `C`, and three witness-value slots. Row and witness admission MUST reject zero lengths, lengths above the finite slot count, moduli outside the bounded field range, and active coefficient or witness values outside the supported element bound.

r[rust-primitives.r1cs-row-shape]

#### Scenario: Valid row and witness shapes are admitted

- GIVEN a row and witness with matching length in `1..=3`, valid modulus, and bounded active elements
- WHEN row and witness admission are evaluated
- THEN both shapes are admitted

#### Scenario: Length mismatch is rejected by satisfaction

- GIVEN an individually valid row and witness with different active lengths
- WHEN row satisfaction is evaluated
- THEN the satisfaction helper rejects the pair

### Requirement: R1CS Dot Product

The system MUST compute a bounded modular dot product for an admitted row coefficient vector and witness over the active length. The executable dot-product helper MUST refine the spec expression and MUST NOT use unchecked arithmetic outside the finite bounds guaranteed by admission.

r[rust-primitives.r1cs-dot-product]

#### Scenario: Active slots contribute to modular dot product

- GIVEN active coefficient and witness slots within the admitted bounds
- WHEN the dot product is computed
- THEN the result equals the active pairwise sum reduced by the row modulus

#### Scenario: Inactive slots do not affect shorter rows

- GIVEN an admitted row length shorter than the physical slot count
- WHEN inactive coefficient or witness slots differ
- THEN only active slots contribute to the dot-product result

### Requirement: R1CS Row Satisfaction

The system MUST expose a one-row R1CS satisfaction helper that returns true exactly when row and witness shapes are admitted, lengths match, and `<A,w> * <B,w> == <C,w>` modulo the row modulus. The helper MUST reject malformed shapes, mismatched lengths, and unsatisfied equations without claiming multi-row constraint-system or proving-system soundness.

r[rust-primitives.r1cs-row-satisfaction]

#### Scenario: Satisfied row is admitted

- GIVEN an admitted row and matching witness whose modular dot products satisfy the equation
- WHEN row satisfaction is evaluated
- THEN the helper returns true

#### Scenario: Unsatisfied row is rejected

- GIVEN an admitted row and matching witness whose modular dot products do not satisfy the equation
- WHEN row satisfaction is evaluated
- THEN the helper returns false

### Requirement: R1CS Verification Coverage

The R1CS primitive MUST provide explicit `r[impl]` and `r[verify]` coverage for every public helper introduced by this change, plus positive and negative runtime tests before README/catalog claims are made.

r[rust-primitives.r1cs-verification-coverage]

#### Scenario: Public helper has traceable coverage

- GIVEN a public helper in `r1cs_constraint`
- WHEN repo-wide coverage checks are run
- THEN the helper has explicit requirement coverage and corresponding proof or test evidence

### Requirement: R1CS System Shape

The system MUST expose a finite multi-row R1CS envelope with explicit system length, shared modulus, and three physical row slots. System admission MUST reject zero lengths, lengths above the finite row-slot count, invalid system moduli, invalid active rows, and active rows whose modulus differs from the system modulus. Inactive rows MUST NOT affect system-shape admission.

r[rust-primitives.r1cs-system-shape]

#### Scenario: Valid active rows are admitted

- GIVEN a system with length in `1..=3`, a valid modulus, and active rows using that modulus
- WHEN system-shape admission is evaluated
- THEN the system shape is admitted

#### Scenario: Modulus mismatch rejects active row

- GIVEN an otherwise valid system with an active row over a different modulus
- WHEN system-shape admission is evaluated
- THEN the system is rejected

### Requirement: R1CS System Satisfaction

The system MUST expose a satisfaction helper that returns true exactly when the system shape is admitted, the witness is admitted, and every active row is satisfied by the same witness. The helper MUST reject malformed witnesses, failed active rows, and malformed active rows without claiming proving-system soundness.

r[rust-primitives.r1cs-system-satisfaction]

#### Scenario: All active rows satisfied

- GIVEN an admitted system and admitted witness where every active row satisfies the R1CS equation
- WHEN system satisfaction is evaluated
- THEN the helper returns true

#### Scenario: One failed active row rejects the system

- GIVEN an admitted system and admitted witness where one active row fails the R1CS equation
- WHEN system satisfaction is evaluated
- THEN the helper returns false

### Requirement: R1CS System Verification Coverage

The R1CS system primitive MUST provide explicit `r[impl]` and `r[verify]` coverage for every public helper introduced by this change, plus positive and negative runtime tests before README/catalog claims are made.

r[rust-primitives.r1cs-system-verification-coverage]

#### Scenario: Public helper has traceable coverage

- GIVEN a public helper in `r1cs_system`
- WHEN repo-wide coverage checks are run
- THEN the helper has explicit requirement coverage and corresponding proof or test evidence

### Requirement: R1CS Public Input Binding Shape

The system MUST expose a finite public-input binding envelope with explicit active length, witness-index slots, and public-value slots. Binding-shape admission MUST accept only active lengths within the bounded slot count, active indices inside the admitted witness prefix, admitted public values, and an admitted modulus. Inactive binding slots MUST NOT affect binding-shape admission.

r[rust-primitives.r1cs-public-input-binding-shape]

#### Scenario: Matching public inputs are admitted

- GIVEN admitted public bindings whose active indices are inside the witness prefix and whose active public values match the indexed witness values modulo the modulus
- WHEN public-input binding admission is evaluated
- THEN the binding helper returns true

#### Scenario: Out-of-range binding index is rejected

- GIVEN an active public binding whose witness index is outside the active witness prefix
- WHEN public-input binding admission is evaluated
- THEN the binding helper returns false

### Requirement: R1CS Public Statement Satisfaction

The system MUST expose a public statement satisfaction helper that returns true exactly when the R1CS system is satisfied by the witness and the public-input bindings are satisfied against that same witness and system modulus. The helper MUST reject malformed systems, malformed witnesses, unsatisfied systems, mismatched public values, and out-of-range public-input bindings without claiming proving-system soundness.

r[rust-primitives.r1cs-public-statement-satisfaction]

#### Scenario: Satisfied system with matching public inputs is admitted

- GIVEN an admitted R1CS system, an admitted witness satisfying every active row, and public bindings matching selected witness slots
- WHEN public statement satisfaction is evaluated
- THEN the helper returns true

#### Scenario: Mismatched public value rejects statement

- GIVEN an otherwise satisfied R1CS system and admitted witness with one active public value different from the bound witness slot modulo the system modulus
- WHEN public statement satisfaction is evaluated
- THEN the helper returns false

### Requirement: R1CS Public Input Verification Coverage

The R1CS public-input primitive MUST provide explicit `r[impl]` and `r[verify]` coverage for every public helper introduced by this change, plus positive and negative runtime tests before README/catalog claims are made.

r[rust-primitives.r1cs-public-input-verification-coverage]

#### Scenario: Public helper has traceable coverage

- GIVEN a public helper in `r1cs_public_inputs`
- WHEN repo-wide coverage checks are run
- THEN the helper has explicit requirement coverage and corresponding proof or test evidence

### Requirement: R1CS Gate Shape

A bounded R1CS gate adapter MUST admit only supported finite gate kinds, valid witness lengths, valid moduli, in-range active witness indices, and bounded constant values.

r[rust-primitives.r1cs-gate-shape]

#### Scenario: Malformed gate rejection

- GIVEN a gate with an unsupported shape, out-of-range witness index, invalid witness length, invalid modulus, or overlarge constant value
- WHEN gate admission is checked
- THEN the gate MUST be rejected before satisfaction is claimed

### Requirement: R1CS Gate Row Encoding

A bounded R1CS gate adapter MUST encode multiplication, addition, and constant gates into a single R1CS row with the same witness length and modulus as the admitted gate.

r[rust-primitives.r1cs-gate-row-encoding]

#### Scenario: Multiplication row encoding

- GIVEN an admitted multiplication gate over witness indices x, y, and z
- WHEN the gate is encoded to a row
- THEN the row MUST enforce `w[x] * w[y] = w[z]` through the existing R1CS row satisfaction predicate

#### Scenario: Addition and constant row encoding

- GIVEN an admitted addition or constant gate and a witness whose slot zero is the constant-one wire
- WHEN the gate is encoded to a row
- THEN addition MUST enforce `(w[x] + w[y]) * 1 = w[z]` and constant MUST enforce `w[x] * 1 = c` through the existing R1CS row satisfaction predicate

### Requirement: R1CS Gate Satisfaction

A bounded R1CS gate satisfaction helper MUST return true exactly when the gate is admitted, the witness is admitted with the same active length, any required constant-one wire is present, and the encoded R1CS row is satisfied.

r[rust-primitives.r1cs-gate-satisfaction]

#### Scenario: Gate satisfaction and rejection

- GIVEN admitted multiplication, addition, or constant gates and a witness
- WHEN the gate satisfaction helper is evaluated
- THEN matching witnesses MUST be accepted and mismatched witness values MUST be rejected

### Requirement: R1CS Gate Verification Coverage

The R1CS gate primitive MUST include verification and runtime coverage for gate admission, row encoding, satisfaction, malformed gate rejection, and mismatched witness rejection.

r[rust-primitives.r1cs-gate-verification-coverage]

#### Scenario: Coverage remains catalogued

- GIVEN the R1CS gate primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the gate requirements MUST have implementation and verification references

### Requirement: R1CS Boolean Shape

A bounded R1CS boolean primitive MUST admit only valid witness lengths, valid moduli whose `modulus - 1` coefficient fits the bounded row model, and a nonzero boolean witness index inside the active witness prefix.

r[rust-primitives.r1cs-boolean-shape]

#### Scenario: Malformed boolean shape rejection

- GIVEN an invalid witness length, invalid modulus, overlarge `modulus - 1` coefficient, zero boolean index, or out-of-range boolean index
- WHEN boolean shape admission is checked
- THEN the shape MUST be rejected before satisfaction is claimed

### Requirement: R1CS Boolean Row Encoding

A bounded R1CS boolean primitive MUST encode booleanity as a single R1CS row equivalent to `b * (1 - b) = 0` using the existing constant-one witness slot convention.

r[rust-primitives.r1cs-boolean-row-encoding]

#### Scenario: Boolean row encoding

- GIVEN an admitted boolean index and modulus
- WHEN the booleanity row is encoded
- THEN the row MUST use the active witness length and modulus and MUST be valid under the existing R1CS row-shape predicate

### Requirement: R1CS Boolean Satisfaction

A bounded R1CS boolean satisfaction helper MUST return true exactly when the boolean shape is admitted, the witness is admitted with the same active length, witness slot zero is one, and the encoded row is satisfied.

r[rust-primitives.r1cs-boolean-satisfaction]

#### Scenario: Boolean values accepted and non-boolean values rejected

- GIVEN admitted boolean shapes and witnesses
- WHEN boolean satisfaction is evaluated
- THEN witness values 0 and 1 at the boolean slot MUST be accepted and non-boolean values MUST be rejected

### Requirement: R1CS Boolean Verification Coverage

The R1CS boolean primitive MUST include implementation and verification coverage for shape admission, row encoding, satisfaction, malformed shape rejection, missing one-wire rejection, and non-boolean witness rejection.

r[rust-primitives.r1cs-boolean-verification-coverage]

#### Scenario: Boolean primitive coverage remains catalogued

- GIVEN the R1CS boolean primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the boolean requirements MUST have implementation and verification references
### Requirement: R1CS Bit Decomposition Shape

A bounded R1CS bit-decomposition primitive MUST admit only valid witness lengths, valid boolean-compatible moduli, a bounded value below four, and two distinct nonzero bit witness indices inside the active witness prefix.

r[rust-primitives.r1cs-bit-decomposition-shape]

#### Scenario: Malformed bit-decomposition shape rejection

- GIVEN an invalid witness length, invalid boolean-compatible modulus, value outside the two-bit range, zero bit index, out-of-range bit index, or duplicate bit indices
- WHEN bit-decomposition shape admission is checked
- THEN the shape MUST be rejected before satisfaction is claimed

### Requirement: R1CS Bit Boolean Composition

A bounded R1CS bit-decomposition primitive MUST require each active bit witness slot to satisfy the existing R1CS boolean primitive under the same active witness length and modulus.

r[rust-primitives.r1cs-bit-decomposition-boolean-composition]

#### Scenario: Non-boolean bit rejection

- GIVEN an admitted bit-decomposition shape and a witness with a non-boolean value in either bit slot
- WHEN bit-decomposition satisfaction is evaluated
- THEN satisfaction MUST be rejected

### Requirement: R1CS Bit Recomposition Satisfaction

A bounded R1CS bit-decomposition satisfaction helper MUST return true exactly when the shape and witness are admitted, both bit slots are boolean, witness slot zero is one, and `value = bit0 + 2 * bit1`.

r[rust-primitives.r1cs-bit-decomposition-satisfaction]

#### Scenario: Recomposition accepts matching bits and rejects mismatches

- GIVEN an admitted bit-decomposition shape and boolean bit witness slots
- WHEN bit-decomposition satisfaction is evaluated
- THEN matching two-bit recompositions MUST be accepted and mismatched recompositions MUST be rejected

### Requirement: R1CS Bit Decomposition Verification Coverage

The R1CS bit-decomposition primitive MUST include implementation and verification coverage for shape admission, boolean-bit composition, recomposition satisfaction, malformed shape rejection, non-boolean bit rejection, and mismatched value rejection.

r[rust-primitives.r1cs-bit-decomposition-verification-coverage]

#### Scenario: Bit-decomposition primitive coverage remains catalogued

- GIVEN the R1CS bit-decomposition primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the bit-decomposition requirements MUST have implementation and verification references

### Requirement: R1CS Lookup Shape

A bounded R1CS lookup-membership primitive MUST admit only valid witness lengths, a nonzero lookup witness index inside the active witness prefix, a table length from one to three, and bounded R1CS elements in every active table entry.

r[rust-primitives.r1cs-lookup-shape]

#### Scenario: Malformed lookup shape rejection

- GIVEN an invalid witness length, zero lookup index, out-of-range lookup index, zero table length, table length above three, or overlarge active table element
- WHEN lookup shape admission is checked
- THEN the shape MUST be rejected before membership satisfaction is claimed

### Requirement: R1CS Lookup Active Table Membership

A bounded R1CS lookup-membership primitive MUST compare the selected witness-slot value only against active table entries and MUST ignore inactive table entries.

r[rust-primitives.r1cs-lookup-active-membership]

#### Scenario: Active table entries determine membership

- GIVEN an admitted lookup shape, an admitted witness, and a selected witness value
- WHEN lookup membership satisfaction is evaluated
- THEN satisfaction MUST hold exactly when the selected value equals at least one active table entry

### Requirement: R1CS Lookup Satisfaction

A bounded R1CS lookup-membership satisfaction helper MUST return true exactly when the lookup shape and witness are admitted, the witness length matches the lookup length, and the selected witness value is a member of the active table prefix.

r[rust-primitives.r1cs-lookup-satisfaction]

#### Scenario: Lookup accepts members and rejects non-members

- GIVEN an admitted lookup shape and admitted witness
- WHEN lookup satisfaction is evaluated
- THEN active-table member values MUST be accepted and non-member values MUST be rejected

### Requirement: R1CS Lookup Verification Coverage

The R1CS lookup-membership primitive MUST include implementation and verification coverage for shape admission, active table membership, satisfaction, malformed shape rejection, malformed witness rejection, inactive-entry ignoring, and non-member rejection.

r[rust-primitives.r1cs-lookup-verification-coverage]

#### Scenario: Lookup primitive coverage remains catalogued

- GIVEN the R1CS lookup-membership primitive is present
- WHEN repo-wide requirement coverage checks run
- THEN the lookup requirements MUST have implementation and verification references
