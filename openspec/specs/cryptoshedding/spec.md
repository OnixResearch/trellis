# Cryptoshedding Specification

## Purpose

Defines requirements for verified cryptoshedding (crypto-shredding) over
key trees, covering chain recoverability, shed completeness, shed
isolation, subtree shedding, key rotation, and idempotent destruction.

## Requirements

### Requirement: Tree Well-Formedness

The parent relation MUST form a finite forest: each node has at most one
parent, and there are no cycles. A root is a node with no parent.

r[cryptoshedding.tree-well-formedness]

#### Scenario: No cycles

- GIVEN a key tree with nodes A→B→C
- WHEN C's parent is set to A (creating cycle A→B→C→A)
- THEN the tree is not well-formed

#### Scenario: Single parent

- GIVEN a key tree where node L has parent I
- WHEN L is also assigned parent J (J ≠ I)
- THEN the tree is not well-formed (parent-indexed representation
  prevents this structurally)

#### Scenario: Valid forest

- GIVEN two independent chains L1→I1→R1 and L2→R2
- AND no cycles exist
- THEN the tree is well-formed

### Requirement: Shed Definition

Shedding key K produces a new keystore equal to the old keystore with K
removed. The tree structure is unchanged.

r[cryptoshedding.shed-definition]

#### Scenario: Shed present key

- GIVEN keystore containing K
- WHEN K is shed
- THEN the new keystore equals the old keystore minus K
- AND the tree is unchanged

#### Scenario: Shed absent key

- GIVEN keystore not containing K
- WHEN K is shed
- THEN the keystore is unchanged

### Requirement: Chain Recoverability

Data is encrypted under leaf nodes only (internal nodes protect other
keys, not data directly). A datum encrypted under leaf key L is
recoverable if and only if every key on the chain from L to its root is
present in the keystore.

r[cryptoshedding.chain-recoverability]

#### Scenario: Full chain present

- GIVEN a key tree with root R, intermediate I, and leaf L (L→I→R)
- AND all of R, I, L are in the keystore
- WHEN recoverability of a datum under L is checked
- THEN the datum is recoverable

#### Scenario: Intermediate missing

- GIVEN the same chain L→I→R
- AND I is not in the keystore
- WHEN recoverability is checked
- THEN the datum is not recoverable

#### Scenario: Leaf missing

- GIVEN the same chain L→I→R
- AND L is not in the keystore
- WHEN recoverability is checked
- THEN the datum is not recoverable

### Requirement: Shed Completeness

Destroying key K MUST render irrecoverable every datum whose leaf-to-root
chain passes through K.

r[cryptoshedding.shed-completeness]

#### Scenario: Shed intermediate

- GIVEN chain L→I→R with all keys present
- WHEN I is shed (removed from keystore)
- THEN the datum under L is no longer recoverable

#### Scenario: Shed root

- GIVEN chain L→I→R with all keys present
- WHEN R is shed
- THEN the datum under L is no longer recoverable

### Requirement: Shed Isolation

Destroying key K MUST NOT affect the recoverability of any datum whose
leaf-to-root chain does not pass through K.

r[cryptoshedding.shed-isolation]

#### Scenario: Sibling subtree unaffected

- GIVEN two chains L1→I1→R and L2→I2→R
- AND all keys present
- WHEN I1 is shed
- THEN the datum under L2 remains recoverable

#### Scenario: Separate tree unaffected

- GIVEN two independent trees rooted at R1 and R2
- AND all keys present
- WHEN R1 is shed
- THEN all data under R2 remain recoverable

### Requirement: Subtree Shedding

Destroying an internal key K MUST shed the entire subtree rooted at K:
every datum bound to a leaf whose chain passes through K or any
descendant of K becomes irrecoverable. (Derived lemma from
chain-recoverability and shed-completeness; verified as an explicit
proof artifact.)

r[cryptoshedding.subtree-shedding]

#### Scenario: Three-level subtree

- GIVEN root R, internal I, leaves L1 and L2 with L1→I→R and L2→I→R
- WHEN I is shed
- THEN both L1 and L2 data are irrecoverable

### Requirement: Rotation Correctness

Rotation is an atomic re-parent: `tree'[L] = Some(P_new)`, all other
entries unchanged. P_new MUST already exist in the tree. Cross-tree
rotation (P_new in a different forest root) is in scope. For a valid
rotation, if L previously had parent P_old, the new parent chain MUST
NOT pass through P_old (re-parenting under a descendant of P_old is out
of scope because it keeps the old key on the new chain). After rotating
leaf L from old parent P_old to new parent P_new: the old parent's
destruction does not affect L's recoverability, and L is recoverable iff
the new chain (L→P_new→...→root) is fully present.

r[cryptoshedding.rotation-correctness]

#### Scenario: Old parent shed after rotation

- GIVEN chain L→P_old→R, rotated to L→P_new→R
- WHEN P_old is shed
- THEN the datum under L is still recoverable (if P_new and R present)

#### Scenario: New parent shed after rotation

- GIVEN chain L→P_old→R, rotated to L→P_new→R
- WHEN P_new is shed
- THEN the datum under L is not recoverable

### Requirement: Idempotent Destruction

Destroying an already-destroyed key is a no-op: the keystore and the
recoverability of all data are unchanged.

r[cryptoshedding.idempotent-destruction]

#### Scenario: Double shed

- GIVEN key K already shed
- WHEN K is shed again
- THEN the keystore is unchanged
- AND recoverability of all data is unchanged
