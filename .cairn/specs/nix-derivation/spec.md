# Nix Derivation Primitives Specification

## Purpose

Defines requirements for machine-checked implementations of the Nix
derivation layer: the algorithms that compute store paths from derivation
descriptions. Covers base32 encoding, hash truncation, fingerprint
construction, ATerm serialization, and store path composition.

All operations use `Seq<u8>` for byte strings, matching the existing
nix module conventions.

## Requirements

### Nix Base32 Encoding

Nix uses a non-standard base32 alphabet: `0123456789abcdfghijklmnpqrsvwxyz`.
This omits `e`, `o`, `t`, `u` to avoid English words in store paths. The
bit-packing order is reversed compared to RFC 4648.

### Requirement: Base32 Alphabet
r[nix-base32.alphabet]

The encoding alphabet MUST be exactly the 32 characters
`0123456789abcdfghijklmnpqrsvwxyz`, indexed 0–31. Each character MUST
map to a unique 5-bit value. The alphabet MUST NOT contain `e`, `o`,
`t`, or `u`.

### Requirement: Base32 Encode
r[nix-base32.encode]

encode(bytes) MUST produce a character sequence where each output
character represents 5 bits of input, packed from the last byte toward
the first (Nix's reversed order). The output length MUST be
`ceil(input_len * 8 / 5)`. Encoding an empty sequence MUST return
an empty sequence.

#### Scenario: Known encoding
- GIVEN the byte sequence [0x1f]
- WHEN encoded
- THEN the output matches the Nix reference implementation

#### Scenario: Round-trip
- GIVEN any byte sequence b
- WHEN encode then decode is applied
- THEN decode(encode(b)) == b

### Requirement: Base32 Decode
r[nix-base32.decode]

decode(chars) MUST invert encode: unpack 5-bit values from the character
sequence back into bytes. MUST return None for characters outside the
alphabet. For valid encodings, decode(encode(b)) MUST equal b.
decode of an empty sequence MUST return an empty sequence.

### Requirement: Base32 Bijectivity
r[nix-base32.bijectivity]

encode MUST be injective: distinct byte sequences MUST produce distinct
encodings. decode MUST be a left inverse of encode:
`decode(encode(x)) == Some(x)` for all byte sequences x. encode MUST
be a left inverse of decode on valid encodings:
`encode(decode(y).unwrap()) == y` for all valid encoded strings y.

### Requirement: Base32 Output Length
r[nix-base32.output-length]

encode output length MUST equal `(input_len * 8 + 4) / 5` (ceiling
division of bits by 5). This is deterministic — same input length always
produces same output length.

### Hash Truncation

### Requirement: Compress Hash
r[nix-hash-truncate.compress]

compress_hash(hash, target_len) MUST reduce a byte sequence to exactly
`target_len` bytes by XOR-folding: `output[i % target_len] ^= hash[i]`
for each byte in hash. The output MUST be initialized to all zeros
before folding. When hash.len() <= target_len, the output MUST equal
hash zero-padded to target_len.

#### Scenario: Identity case
- GIVEN a hash of length 20 and target_len 20
- WHEN compressed
- THEN output equals the input (XOR with zero padding is identity)

#### Scenario: Fold case
- GIVEN a 32-byte SHA-256 hash and target_len 20
- WHEN compressed
- THEN bytes 0–19 of hash are XOR'd with bytes 20–31 (zero-padded)

### Requirement: Compress Determinism
r[nix-hash-truncate.determinism]

compress_hash MUST be deterministic: the same (hash, target_len) pair
MUST always produce the same output. The output MUST depend only on the
input bytes and target_len, with no hidden state.

### Requirement: Compress Output Length
r[nix-hash-truncate.output-length]

The output of compress_hash MUST have length exactly target_len. This
MUST hold regardless of input length — even when input is shorter than
target_len.

### Requirement: Compress XOR Properties
r[nix-hash-truncate.xor-properties]

compress_hash MUST satisfy: compress(a, n) XOR compress(b, n) equals
compress(a XOR b, n) when a and b have equal length. This follows from
XOR's associativity and commutativity. compress_hash of an all-zero
input MUST return all zeros.

### Store Path Fingerprint

### Requirement: Regular Output Fingerprint
r[nix-fingerprint.regular-output]

For a regular (input-addressed) derivation output, the fingerprint MUST
be the byte string `"output:<output_name>:sha256:<drv_hash>:/nix/store:<name>"`.
Each field MUST be separated by exactly one colon. The drv_hash MUST be
the hex-encoded SHA-256 of the masked derivation.

### Requirement: Fixed Output Fingerprint
r[nix-fingerprint.fixed-output]

For a fixed-output derivation, the fingerprint MUST be
`"fixed:out:<hash_algo>:<hash>:<name>"` where hash_algo is `sha256`,
`sha1`, or `md5`, and hash is the hex-encoded expected output hash.
The output name for fixed-output MUST always be `"out"`.

### Requirement: Text Hash Fingerprint
r[nix-fingerprint.text-hash]

For a text hash (e.g., `builtins.toFile`), the fingerprint MUST be
`"text:<input_hash1>:<input_hash2>:...:<text_hash>:/nix/store:<name>"`.
Input hashes MUST be listed in sorted order. The text_hash is the
SHA-256 of the file contents.

### Requirement: Fingerprint Injectivity
r[nix-fingerprint.injectivity]

Distinct (type, hash, name) triples MUST produce distinct fingerprints.
The colon-separated format MUST be unambiguous — names and hashes MUST
NOT contain colons. This ensures two different derivations never collide
on fingerprint.

#### Scenario: Different names
- GIVEN two derivations identical except for name "foo" vs "bar"
- WHEN fingerprints are computed
- THEN the fingerprints differ

#### Scenario: Different types
- GIVEN a regular and a fixed-output derivation with same hash and name
- WHEN fingerprints are computed
- THEN the fingerprints differ (prefix "output:" vs "fixed:")

### Derivation ATerm Serialization

### Requirement: ATerm String Escaping
r[nix-drv-serialize.string-escape]

ATerm string serialization MUST escape backslash as `\\`, double-quote
as `\"`, newline as `\n`, carriage return as `\r`, and tab as `\t`.
All other bytes MUST be emitted verbatim. The result MUST be wrapped
in double quotes.

### Requirement: ATerm Derivation Format
r[nix-drv-serialize.format]

A derivation MUST serialize as
`Derive(<outputs>,<input_drvs>,<input_srcs>,<platform>,<builder>,<args>,<env>)`
where each field is an ATerm list or string. Lists use brackets `[...]`
with comma separators. Pairs use parentheses `(...)`.

### Requirement: Serialization Determinism
r[nix-drv-serialize.determinism]

Serialization MUST be deterministic. Input derivations MUST be sorted
by path. Input sources MUST be sorted. Environment variables MUST be
sorted by name. Output declarations MUST be sorted by output name.
Given the same derivation structure, serialization MUST always produce
the same byte sequence.

### Requirement: Serialization Injectivity
r[nix-drv-serialize.injectivity]

Distinct derivation structures MUST serialize to distinct byte sequences.
The ATerm format MUST be unambiguous — the escaping scheme and delimiter
structure MUST prevent two different derivations from producing identical
serializations.

#### Scenario: Env var ordering
- GIVEN a derivation with env vars {b: "2", a: "1"}
- WHEN serialized
- THEN env vars appear as [("a","1"),("b","2")] (sorted by key)

#### Scenario: Escape round-trip
- GIVEN an env var value containing `\n` and `"`
- WHEN serialized and parsed
- THEN the original value is recovered

### Store Path Computation

### Requirement: Store Path Format
r[nix-store-path.format]

A store path MUST have the format
`/nix/store/<hash_part>-<name>` where hash_part is the Nix base32
encoding of compress_hash(sha256(fingerprint), 20). The hash_part
MUST be exactly 32 characters (160 bits in base32). The name MUST
follow the fingerprint's name component.

### Requirement: Store Path Hash Part
r[nix-store-path.hash-part]

The hash part MUST be computed as:
1. Compute SHA-256 of the fingerprint bytes
2. Compress to 20 bytes via compress_hash
3. Encode in Nix base32
The result MUST be exactly 32 characters long. This is deterministic
given the fingerprint.

#### Scenario: Store path length
- GIVEN any valid derivation
- WHEN the store path is computed
- THEN the path is `/nix/store/` (11 chars) + 32 char hash + `-` + name

### Requirement: Store Path Determinism
r[nix-store-path.determinism]

The same derivation MUST always produce the same store path. This
follows from the determinism of each component: serialization,
SHA-256, compress_hash, and base32 encode.

### Requirement: Store Path Injectivity
r[nix-store-path.injectivity]

Distinct derivations MUST produce distinct store paths, assuming
SHA-256 is collision-resistant. This follows from the injectivity
chain: serialization is injective, SHA-256 is collision-resistant,
compress_hash preserves distinctness with overwhelming probability,
base32 encode is injective.

### Requirement: Input-Addressed Output Path
r[nix-store-path.input-addressed]

For an input-addressed derivation, the output path MUST be computed by:
1. Serialize the derivation in ATerm format
2. Replace self-referencing output paths with empty strings (masking)
3. Hash the masked serialization with SHA-256
4. Construct the fingerprint as `"output:<name>:sha256:<hex_hash>:/nix/store:<drv_name>"`
5. Compute the store path from the fingerprint

### Requirement: Fixed-Output Path
r[nix-store-path.fixed-output]

For a fixed-output derivation, the output path MUST be computed by:
1. Construct the fingerprint as `"fixed:out:<hash_algo>:<hex_hash>:<name>"`
2. Compute the store path from the fingerprint
The output path depends only on the declared hash and name, NOT on
the derivation's builder, args, or env.

#### Scenario: Fixed-output independence
- GIVEN two fixed-output derivations with same hash and name but different builders
- WHEN output paths are computed
- THEN both produce the same store path

### Store Path Name Validation

### Requirement: Valid Store Path Name
r[nix-store-path.valid-name]

A store path name MUST be 1–211 bytes long. Each byte MUST be one of:
ASCII letter (a-z, A-Z), digit (0-9), or one of `+-._?=`. The name
MUST NOT start with a dot. Names violating these rules MUST be rejected.

#### Scenario: Empty name
- GIVEN an empty name string
- WHEN validated
- THEN validation fails (minimum length is 1)

#### Scenario: Dot prefix
- GIVEN name ".hidden"
- WHEN validated
- THEN validation fails (cannot start with dot)

#### Scenario: Valid characters
- GIVEN name "hello-world_1.0"
- WHEN validated
- THEN validation passes

## ADDED Requirements

All requirements in this document are new additions.

## MODIFIED Requirements

None.

## REMOVED Requirements

None.
