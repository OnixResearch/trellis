# Validation Summary

## Commands

### Focused module tests
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_signature`
  - Result: pass (`5 passed; 0 failed`)
  - Transcript: `cargo-test-ucan-signature.txt`

### Full-lib Verus verification
- `./scripts/verus.sh src/lib.rs`
  - Result: pass (`8113 verified, 0 errors`)
  - Transcript: `verus-lib.txt`

### Focused runtime spot checks
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib test_sign_verify_round_trip`
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib test_wrong_key_rejects`
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib test_wrong_message_rejects`
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib test_sign_deterministic`
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib test_public_key_length`
  - Result: all pass

## Scope Notes
- Artifact sync removed the stale `keypair_generate` commitment.
- `SecretKey` is now consistently documented as a 32-byte Ed25519 seed across proposal, design, and spec.
- The change scope is the primitive byte-level sign/verify/public-key-derivation contract; malformed external byte strings remain out of scope.
