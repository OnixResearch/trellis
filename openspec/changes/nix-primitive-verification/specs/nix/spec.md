# Nix Primitive Verification Specification

## Purpose

Defines the requirements for machine-checked implementations of Nix language
builtin operations. Each requirement maps to a `spec fn` / `exec fn` pair
verified by Verus, with Tracey annotations linking back to this document.

Operations model Nix semantics over Verus-native types: `Seq<u8>` for strings,
`Seq<(Seq<u8>, V)>` for attribute sets (sorted by key), `Seq<T>` for lists.

## Requirements

### Nix Version Comparison

#### Requirement: Version Component Split

r[nix-version.split-components]
split_components MUST split a version string (`Seq<u8>`) on `.` and `-`
delimiters into a sequence of components. Each component is either numeric
(digits only) or alphabetic (non-digit, non-delimiter). Empty components
between adjacent delimiters MUST be preserved as empty sequences.
Concatenating all components with their delimiters MUST reconstruct the
original string.

#### Requirement: Component Classification

r[nix-version.classify-component]
classify_component MUST return Numeric when all bytes are ASCII digits,
Alpha when any byte is not a digit, and Empty when the component has
length zero. Numeric components MUST compare by integer value. Alpha
components MUST compare lexicographically. Empty components MUST sort
before non-empty components.

#### Requirement: Compare Versions

r[nix-version.compare]
compare_versions MUST compare two version strings component-by-component
using Nix semantics: split into components, classify each, compare
pairwise. Shorter version strings are padded with empty components.
The result MUST be -1, 0, or 1. The ordering MUST be total, transitive,
and antisymmetric.

#### Scenario: Numeric ordering
- GIVEN versions "2.10" and "2.9"
- WHEN compared
- THEN "2.10" > "2.9" (numeric comparison: 10 > 9)

#### Scenario: Alpha fallback
- GIVEN versions "2.0a" and "2.0b"
- WHEN compared
- THEN "2.0a" < "2.0b" (lexicographic comparison)

#### Scenario: Pre-release ordering
- GIVEN versions "2.0" and "2.0pre1"
- WHEN compared
- THEN "2.0" > "2.0pre1" (empty component "" < "pre1")

#### Requirement: Parse Derivation Name

r[nix-version.parse-drv-name]
parse_drv_name MUST split a derivation name into (name, version) at the
first hyphen followed by a digit. If no such split point exists, the
version MUST be empty. Recombining name + "-" + version MUST equal the
original when version is non-empty.

### Nix String Operations

#### Requirement: String Length

r[nix-string.length]
string_length MUST return the number of bytes in the string. The result
MUST equal the Seq length. string_length of an empty string MUST be 0.

#### Requirement: Substring

r[nix-string.substring]
substring(s, start, len) MUST return the subsequence of s from index
start with at most len bytes. When start >= s.len(), MUST return empty.
When start + len > s.len(), MUST return from start to end. The result
length MUST be min(len, s.len() - start) when start < s.len().

#### Scenario: Within bounds
- GIVEN string "hello" and start=1, len=3
- WHEN substring is called
- THEN result is "ell"

#### Scenario: Past end
- GIVEN string "hello" and start=3, len=10
- WHEN substring is called
- THEN result is "lo"

#### Requirement: Replace Strings

r[nix-string.replace-strings]
replace_strings(s, from, to) MUST replace all non-overlapping occurrences
of `from[i]` with `to[i]`, scanning left to right, matching the first
applicable pattern at each position. from and to MUST have equal length.
When no patterns match, MUST return s unchanged.

### Nix Attribute Set Operations

Attribute sets are modeled as `Seq<(Seq<u8>, V)>` sorted by key with no
duplicate keys. The sorted-key invariant MUST be preserved by all operations.

#### Requirement: Attr Names

r[nix-attrset.names]
attr_names MUST return a sorted sequence of all keys in the attribute set.
The result length MUST equal the attribute set length. Each key MUST appear
exactly once.

#### Requirement: Attr Values

r[nix-attrset.values]
attr_values MUST return the values in key-sorted order. The result length
MUST equal the attribute set length.

#### Requirement: Has Attr

r[nix-attrset.has-attr]
has_attr(attrs, key) MUST return true if and only if key exists in attrs.
MUST use binary search. has_attr on an empty set MUST return false.

#### Requirement: Get Attr

r[nix-attrset.get-attr]
get_attr(attrs, key) MUST return Some(value) when key exists, None
otherwise. When has_attr returns true, get_attr MUST return Some.
When has_attr returns false, get_attr MUST return None.

#### Requirement: Merge (Update Operator)

r[nix-attrset.merge]
merge(a, b) MUST produce an attribute set containing all keys from both
inputs. For keys present in both, the value from b MUST take precedence.
Merge MUST be associative: merge(a, merge(b, c)) == merge(merge(a, b), c).
Merge with an empty set MUST be the identity: merge(a, {}) == a and
merge({}, a) == a. The result MUST maintain sorted-key order with no
duplicates.

#### Scenario: Disjoint merge
- GIVEN attrs {a: 1} and {b: 2}
- WHEN merged
- THEN result is {a: 1, b: 2}

#### Scenario: Overlapping merge
- GIVEN attrs {a: 1, b: 2} and {b: 3, c: 4}
- WHEN merged
- THEN result is {a: 1, b: 3, c: 4} (b from right wins)

#### Requirement: Intersect Attrs

r[nix-attrset.intersect]
intersect_attrs(a, b) MUST return an attribute set containing only keys
present in both a and b, with values from b. Intersect MUST be
commutative on key sets: keys(intersect(a, b)) == keys(intersect(b, a)).
Intersect of disjoint sets MUST return empty. Intersect with self MUST
return self.

#### Requirement: Remove Attrs

r[nix-attrset.remove]
remove_attrs(attrs, keys) MUST return attrs without any entries whose key
is in `keys`. Removing no keys MUST return attrs unchanged. Removing all
keys MUST return empty. remove_attrs(merge(a, b), keys(b)) MUST equal
remove_attrs(a, keys(b)) when a and b are disjoint on those keys.

#### Requirement: List to Attrs

r[nix-attrset.list-to-attrs]
list_to_attrs MUST convert a sequence of (name, value) pairs into a
sorted attribute set. Duplicate names MUST use the last occurrence.
The result MUST satisfy the sorted-key invariant.

### Nix Attribute Path

#### Requirement: Attr Path Lookup

r[nix-attrpath.lookup]
attr_path_lookup(attrs, path) MUST traverse nested attribute sets
following each component of path in order. MUST return None if any
component is missing. MUST return Some(value) at the final component.
An empty path MUST return the root value.

#### Requirement: Attr Path Exists

r[nix-attrpath.exists]
attr_path_exists(attrs, path) MUST return true if and only if
attr_path_lookup returns Some. MUST be equivalent to
attr_path_lookup(...).is_some().

### Nix List Operations

#### Requirement: Map

r[nix-list.map]
map(list, f) MUST apply f to every element, producing a list of equal
length where result[i] == f(list[i]). Map MUST preserve length.
Map over empty MUST return empty. Map MUST satisfy the functor identity
law: map(list, id) == list.

#### Requirement: Filter

r[nix-list.filter]
filter(list, pred) MUST return a subsequence containing exactly the
elements for which pred returns true, preserving relative order. Filter
length MUST be <= input length. Filter with always-true MUST return the
input. Filter with always-false MUST return empty.

#### Requirement: Foldl

r[nix-list.foldl]
foldl(f, init, list) MUST apply f left-to-right: f(f(f(init, list[0]),
list[1]), list[2])... Fold over empty MUST return init. Fold over a
singleton MUST return f(init, list[0]).

#### Requirement: Concat Lists

r[nix-list.concat-lists]
concat_lists(lists) MUST concatenate a list of lists into a single list.
The result length MUST equal the sum of input lengths. Concat MUST be
associative. Concat of empty list MUST return empty. Concat of single
list MUST return that list unchanged.

#### Requirement: Gen List

r[nix-list.gen-list]
gen_list(f, n) MUST produce a list of length n where result[i] == f(i).
gen_list(f, 0) MUST return empty.

#### Requirement: Elem

r[nix-list.elem]
elem(list, x) MUST return true if and only if x appears in list.
elem on empty MUST return false. elem MUST be equivalent to
filter(list, |e| e == x).len() > 0.

#### Requirement: Elem At

r[nix-list.elem-at]
elem_at(list, n) MUST return Some(list[n]) when n < list.len(), None
otherwise. elem_at on empty MUST return None for any index.

#### Requirement: Head

r[nix-list.head]
head(list) MUST return Some(list[0]) when list is non-empty, None when
empty. head MUST equal elem_at(list, 0).

#### Requirement: Tail

r[nix-list.tail]
tail(list) MUST return the list without its first element. tail of a
singleton MUST return empty. tail length MUST equal list.len() - 1 for
non-empty lists. tail of empty MUST return empty.

#### Requirement: Length

r[nix-list.length]
length(list) MUST return the number of elements. MUST equal the Seq
length. length of empty MUST be 0.

### Nix Sort

#### Requirement: Sort

r[nix-sort.sort]
sort(list, cmp) MUST return a list that is sorted according to cmp and
is a permutation of the input. The result length MUST equal the input
length. Every element in the input MUST appear in the output with the
same multiplicity.

#### Scenario: Numeric sort
- GIVEN list [3, 1, 2] and numeric comparator
- WHEN sorted
- THEN result is [1, 2, 3]

#### Scenario: Empty sort
- GIVEN empty list
- WHEN sorted
- THEN result is empty

#### Requirement: Sort Permutation

r[nix-sort.permutation]
The sort output MUST be a permutation of the input: there exists a
bijection between input and output indices such that output[f(i)] ==
input[i] for all i. This guarantees no elements are lost or duplicated.

#### Requirement: Sort Idempotent

r[nix-sort.idempotent]
Sorting an already-sorted list MUST return the same list. sort(sort(list))
MUST equal sort(list).

### Nix Partition

#### Requirement: Partition

r[nix-partition.partition]
partition(list, pred) MUST split list into (right, wrong) where right
contains elements satisfying pred and wrong contains the rest. Every
element MUST appear in exactly one output list. Relative order within
each output MUST be preserved.

#### Requirement: Partition Length

r[nix-partition.length]
right.len() + wrong.len() MUST equal list.len(). No elements are lost
or duplicated.

#### Requirement: Partition Filter Duality

r[nix-partition.filter-duality]
partition(list, pred).right MUST equal filter(list, pred).
partition(list, pred).wrong MUST equal filter(list, !pred).
Partition computes both filter results in one pass.

### Nix Generic Closure

#### Requirement: Generic Closure

r[nix-closure.closure]
generic_closure(start_set, operator) MUST compute the fixed point:
starting from start_set, repeatedly apply operator to each element,
adding results whose key has not been seen. MUST terminate when no new
keys are produced. The result MUST contain every element reachable from
start_set.

#### Requirement: Closure Monotonicity

r[nix-closure.monotonicity]
The key set MUST grow monotonically: at each iteration, the set of seen
keys is a superset of the previous iteration. Keys are never removed.

#### Requirement: Closure Idempotent

r[nix-closure.idempotent]
Running generic_closure on its own output MUST return the same result.
The closure is a fixed point: closure(closure(S)) == closure(S).

#### Requirement: Closure Contains Start

r[nix-closure.contains-start]
Every element in start_set MUST appear in the output. The closure is
a superset of its initial set.

## ADDED Requirements

All requirements in this document are new additions.

## MODIFIED Requirements

None.

## REMOVED Requirements

None.
