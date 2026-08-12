## Why

Cryptoshedding (also called crypto-shredding or chainshedding) is a data
deletion technique: encrypt data under a key, then destroy the key to
render the ciphertext irrecoverable. When deletion from storage is
expensive, unreliable, or impossible (replicated systems, append-only
logs, backups, WORM media), cryptoshedding gives a provable deletion
guarantee that depends only on key destruction, not on media erasure.

Key chaining extends this: data is encrypted under a leaf key, which is
itself encrypted under a parent key, forming a tree. Destroying any
ancestor key sheds all descendants. This models retention policies
(per-tenant, per-time-bucket, per-regulation) as subtree deletions.

Data is bound to leaf keys only; internal keys protect other keys, not
data directly. The key tree's parent relation must form a finite acyclic
forest (each node has at most one parent, no cycles).

The correctness properties are:
- **Shedding completeness**: destroying a key renders all data encrypted
  (transitively) under that key irrecoverable.
- **Shedding precision**: destroying a key does NOT affect data encrypted
  under unrelated keys (different subtree, different chain).
- **Chain integrity**: a datum is recoverable if and only if every key on
  its chain from root to leaf is present.
- **Rotation safety**: re-encrypting data under a new key preserves
  recoverability and does not create a second path to the plaintext
  through the old key; valid rotation excludes re-parenting under a
  descendant of the old parent.

## What Changes

- **cryptoshedding**: A new module modeling key trees, encryption chains,
  and shed/recover operations with verified isolation and completeness.

  Proves:
  - **Chain recoverability**: data is recoverable iff every key on its
    root-to-leaf path is present in the keystore.
  - **Shed completeness**: destroying key K removes recoverability for
    every datum whose chain passes through K.
  - **Shed precision (isolation)**: destroying key K does not affect
    recoverability of data whose chain does not pass through K.
  - **Subtree shedding**: destroying an internal node sheds the entire
    subtree rooted at that node.
  - **Rotation correctness**: after re-encrypting a datum's leaf under a
    new parent, the old parent's destruction does not shed the datum, and
    the new parent's presence suffices for recovery.
  - **Idempotent destruction**: destroying an already-destroyed key is a
    no-op on the recoverability of all data.

## Capabilities

This module is pure logic over abstract key trees and presence sets. It
does not model specific ciphers, key derivation functions, or storage
backends. Data are represented only by their bound leaf key; there is no
separate datum object or `encrypted_under(datum, key)` relation in scope.

## Verification Strategy

All properties proved in Verus over `Seq`/`Set`-based key trees with
recursive spec helpers for chain membership, reachability, and
recoverability. Exec wrappers use `Vec`-based key trees with loop-based
chain walks. Property tests cover random tree shapes, multi-shed
sequences, and rotation scenarios.
