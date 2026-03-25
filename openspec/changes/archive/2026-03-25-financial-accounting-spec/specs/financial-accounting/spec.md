# Financial Accounting Specification

## Purpose

Defines correctness requirements for financial accounting primitives:
single-account balance operations, two-party transfers with conservation
of value, double-entry bookkeeping with the accounting identity, and
append-only journals with running balance integrity. These building
blocks apply to any system that tracks monetary or token balances.

## Requirements

### Balance Operations

#### Requirement: Credit and Debit Arithmetic

`credit(balance, amount)` MUST return `balance + amount` when the sum
fits in u64, and None on overflow. `debit(balance, amount)` MUST return
`balance - amount` when the balance covers the amount, and None on
underflow. Both operations MUST use checked arithmetic — no silent
truncation or wraparound.

##### Scenario: Successful credit

- GIVEN a balance of 100 and an amount of 50
- WHEN credit is applied
- THEN the result MUST be Some(150)

##### Scenario: Overflow credit

- GIVEN a balance of u64::MAX and an amount of 1
- WHEN credit is applied
- THEN the result MUST be None

##### Scenario: Successful debit

- GIVEN a balance of 100 and an amount of 30
- WHEN debit is applied
- THEN the result MUST be Some(70)

##### Scenario: Underflow debit

- GIVEN a balance of 50 and an amount of 100
- WHEN debit is applied
- THEN the result MUST be None

#### Requirement: Credit-Debit Inverse

Credit and debit MUST be inverse operations. If `credit(b, a)` succeeds
producing `c`, then `debit(c, a)` MUST return `b`. If `debit(b, a)`
succeeds producing `d`, then `credit(d, a)` MUST return `b`.

##### Scenario: Credit then debit roundtrip

- GIVEN a balance of 100 and an amount of 50
- WHEN credit(100, 50) produces 150 and debit(150, 50) is applied
- THEN the result MUST be 100

##### Scenario: Debit then credit roundtrip

- GIVEN a balance of 100 and an amount of 30
- WHEN debit(100, 30) produces 70 and credit(70, 30) is applied
- THEN the result MUST be 100

#### Requirement: Zero Identity

Crediting zero MUST return the original balance. Debiting zero MUST
return the original balance. Zero is the identity element for both
operations.

##### Scenario: Zero credit

- GIVEN any balance b
- WHEN credit(b, 0) is applied
- THEN the result MUST be b

##### Scenario: Zero debit

- GIVEN any balance b
- WHEN debit(b, 0) is applied
- THEN the result MUST be b

#### Requirement: Credit Commutativity

The order of two credits MUST NOT matter. If both `credit(credit(b, a1), a2)`
and `credit(credit(b, a2), a1)` succeed, their results MUST be equal.

##### Scenario: Commutative credits

- GIVEN balance 100 and amounts 30 and 50
- WHEN applying credits in either order
- THEN both final balances MUST be 180

#### Requirement: Sufficient Funds Predicate

`has_funds(balance, amount)` MUST return true if and only if
`debit(balance, amount)` would succeed. The predicate and the debit
operation MUST agree on every input.

##### Scenario: Funds available

- GIVEN balance 100, amount 50
- WHEN has_funds is checked
- THEN it MUST return true and debit MUST succeed

##### Scenario: Funds insufficient

- GIVEN balance 50, amount 100
- WHEN has_funds is checked
- THEN it MUST return false and debit MUST return None

#### Requirement: Valid Amount

`is_valid_amount(amount)` MUST return true if and only if amount > 0.
Zero is not a valid transaction amount.

#### Requirement: Balance Conservation

For any balance and amount, `credit(b, a) - a == b` and
`debit(b, a) + a == b` MUST hold in integer arithmetic. Value is
neither created nor destroyed by either operation.

### Value Transfer

#### Requirement: Transfer Validation

`transfer_valid(from, to, amount)` MUST return true if and only if:
the amount is positive, the sender has sufficient funds, and the
receiver would not overflow. All three conditions MUST be checked.

##### Scenario: Valid transfer

- GIVEN sender balance 100, receiver balance 50, amount 30
- WHEN transfer_valid is checked
- THEN it MUST return true

##### Scenario: Zero amount

- GIVEN any balances
- WHEN amount is 0
- THEN transfer_valid MUST return false

##### Scenario: Insufficient sender

- GIVEN sender balance 100, amount 101
- WHEN transfer_valid is checked
- THEN it MUST return false

##### Scenario: Receiver overflow

- GIVEN receiver balance u64::MAX, amount 1
- WHEN transfer_valid is checked
- THEN it MUST return false

#### Requirement: Transfer Conservation

After a valid transfer, the sum of sender and receiver balances MUST
equal the sum before the transfer. No value is created or destroyed.
This MUST hold for any valid (from, to, amount) triple.

##### Scenario: Conservation

- GIVEN sender 100, receiver 50, amount 30
- WHEN transfer executes producing sender 70, receiver 80
- THEN 70 + 80 == 100 + 50

##### Scenario: Multi-hop conservation

- GIVEN three accounts with balances b1, b2, b3 and two valid transfers
- WHEN both transfers execute
- THEN b1 + b2 + b3 MUST be unchanged

#### Requirement: Transfer Execution

`execute_transfer(from, to, amount)` MUST return the new sender and
receiver balances. The sender balance MUST strictly decrease by the
amount. The receiver balance MUST strictly increase by the amount.

##### Scenario: Sender decreases

- GIVEN a valid transfer of 30 from balance 100
- WHEN executed
- THEN the new sender balance MUST be 70

##### Scenario: Receiver increases

- GIVEN a valid transfer of 30 to balance 50
- WHEN executed
- THEN the new receiver balance MUST be 80

#### Requirement: Idempotency Protection

`is_duplicate(request_id, last_processed_id)` MUST return true when
request_id <= last_processed_id. Duplicate transfers MUST be rejected
before any balance check — the DUPLICATE_TRANSFER status code takes
precedence over all other validations.

##### Scenario: Duplicate rejected

- GIVEN last processed ID is 5 and request ID is 3
- WHEN validate_transfer is called
- THEN the result MUST be DUPLICATE_TRANSFER regardless of balances

##### Scenario: Fresh request passes

- GIVEN last processed ID is 5 and request ID is 6
- WHEN validate_transfer is called
- THEN the result depends on balance checks (not duplicate)

#### Requirement: Exhaustive Validation

`validate_transfer` MUST return exactly one of five status codes:
TRANSFER_OK, INSUFFICIENT_FUNDS, INVALID_AMOUNT, RECEIVER_OVERFLOW,
or DUPLICATE_TRANSFER. The status MUST be TRANSFER_OK if and only if
the transfer is valid and not a duplicate. The five codes MUST be
mutually exclusive and collectively exhaustive.

### Double-Entry Bookkeeping

#### Requirement: Sum Correctness

`sum_amounts(amounts)` MUST return the sum of all elements when it fits
in u64, and None on overflow. The sum of an empty sequence MUST be 0.
The sum of a single element MUST be that element. Sum MUST be
non-negative for all inputs.

##### Scenario: Basic sum

- GIVEN amounts [1, 2, 3]
- WHEN summed
- THEN the result MUST be Some(6)

##### Scenario: Overflow

- GIVEN amounts [u64::MAX, 1]
- WHEN summed
- THEN the result MUST be None

#### Requirement: Sum Composition

Sum MUST distribute over concatenation: `sum(a ++ b) == sum(a) + sum(b)`.
Appending element x MUST increase the sum by x:
`sum(amounts ++ [x]) == sum(amounts) + x`.

##### Scenario: Concatenation

- GIVEN sequences [10, 20] and [30]
- WHEN summed together
- THEN sum([10, 20, 30]) == sum([10, 20]) + sum([30]) == 30 + 30 == 60

#### Requirement: Accounting Identity

`is_balanced(debits, credits)` MUST return true if and only if the sum
of debits equals the sum of credits. This is the fundamental accounting
equation. Balance MUST be symmetric: swapping debits and credits
preserves the result.

##### Scenario: Balanced

- GIVEN debits [60, 40] and credits [100]
- WHEN checking balance
- THEN is_balanced MUST return true (100 == 100)

##### Scenario: Unbalanced

- GIVEN debits [100] and credits [50]
- WHEN checking balance
- THEN is_balanced MUST return false

##### Scenario: Empty balanced

- GIVEN empty debits and credits
- WHEN checking balance
- THEN is_balanced MUST return true (0 == 0)

#### Requirement: Balanced Composition

Concatenating two balanced pairs MUST produce a balanced pair:
if `is_balanced(d1, c1)` and `is_balanced(d2, c2)`, then
`is_balanced(d1++d2, c1++c2)` MUST hold. Adding equal amounts
to both sides MUST preserve balance.

##### Scenario: Merge balanced ledgers

- GIVEN balanced pair ([100], [100]) and balanced pair ([50], [50])
- WHEN concatenated
- THEN ([100, 50], [100, 50]) MUST be balanced

#### Requirement: Trial Balance

`trial_balance(debits, credits)` MUST return the signed difference
(debits - credits). A balanced ledger MUST have zero trial balance.
Zero trial balance MUST imply balanced. Trial balance MUST be additive
across concatenated entries.

##### Scenario: Zero trial balance

- GIVEN a balanced ledger
- WHEN trial_balance is computed
- THEN the absolute value MUST be 0

#### Requirement: Simple Entry

`is_simple_entry(debit, credit)` MUST return true if and only if both
amounts are equal and positive. A simple entry MUST always be balanced.

### Journal Integrity

#### Requirement: Entry Application

`apply_entry(prior_balance, amount, is_debit)` MUST subtract the amount
for debits and add for credits. Debits MUST return None on underflow.
Credits MUST return None on overflow. A credit MUST always increase or
maintain the balance. A debit MUST always decrease or maintain the
balance.

##### Scenario: Credit

- GIVEN prior balance 100 and credit of 30
- WHEN apply_entry is called with is_debit=false
- THEN the result MUST be Some(130)

##### Scenario: Debit underflow

- GIVEN prior balance 10 and debit of 20
- WHEN apply_entry is called with is_debit=true
- THEN the result MUST be None

#### Requirement: Running Balance Correctness

Each journal entry's `running_balance` field MUST equal the result of
applying that entry to the prior balance (initial balance for the first
entry, previous entry's running_balance for subsequent entries).
`entry_balance_correct` MUST verify this property for a single entry.

##### Scenario: First entry

- GIVEN initial balance 100 and a credit of 50
- WHEN the entry has running_balance 150
- THEN entry_balance_correct MUST return true

##### Scenario: Wrong balance

- GIVEN initial balance 100 and a credit of 50
- WHEN the entry has running_balance 999
- THEN entry_balance_correct MUST return false

#### Requirement: Journal Validity

`journal_valid(entries, initial_balance)` MUST return true if and only
if every entry has a correct running balance relative to its predecessor.
An empty journal MUST always be valid.

##### Scenario: Valid journal

- GIVEN initial balance 100 and entries: credit 50 (balance 150), debit 30 (balance 120)
- WHEN journal_valid is checked
- THEN it MUST return true

##### Scenario: Invalid entry

- GIVEN a journal where one entry has an incorrect running balance
- WHEN journal_valid is checked
- THEN it MUST return false

#### Requirement: Append Preserves Validity

Appending a valid entry to a valid journal MUST produce a valid journal.
The new entry's running balance MUST be correct relative to the final
balance of the existing journal. After append, the final balance MUST
equal the new entry's running_balance.

##### Scenario: Append to valid journal

- GIVEN a valid journal with final balance 150
- WHEN a credit of 50 with running_balance 200 is appended
- THEN the extended journal MUST be valid

#### Requirement: Credit-Debit Roundtrip in Journal

Applying a credit followed by a debit of the same amount MUST return
to the original balance. Applying a debit followed by a credit of the
same amount MUST also return to the original balance.

##### Scenario: Credit then debit

- GIVEN balance 100 and amount 30
- WHEN credit(100, 30) = 130, then debit(130, 30) is applied
- THEN the result MUST be 100

#### Requirement: Final Balance

`final_balance(entries, initial)` MUST return the last entry's
running_balance, or initial_balance if the journal is empty.

##### Scenario: Non-empty journal

- GIVEN a journal with last entry running_balance 250
- WHEN final_balance is called
- THEN the result MUST be 250

##### Scenario: Empty journal

- GIVEN an empty journal with initial balance 100
- WHEN final_balance is called
- THEN the result MUST be 100

#### Requirement: Sequence Number Monotonicity

Journal entries MUST have strictly increasing sequence numbers.
`make_entry` MUST reject entries whose sequence number does not
strictly exceed the last entry's sequence number.

##### Scenario: Advancing sequence

- GIVEN a journal with last seq_num 5
- WHEN make_entry is called with seq_num 6
- THEN the entry MUST be accepted

##### Scenario: Stale sequence

- GIVEN a journal with last seq_num 5
- WHEN make_entry is called with seq_num 5
- THEN the entry MUST be rejected
