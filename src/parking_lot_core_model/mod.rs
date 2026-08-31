//! Finite parking-lot core queue model.
//!
//! This module models the queue-table part of `parking_lot_core` with ordinary
//! finite vectors: lock keys map to waiter queues, each waiter identity may
//! appear at most once across the table, and queued waiters must record the key
//! they are parked on.

#[path = "../identitykey.rs"]
mod identitykey;
#[path = "../threadid.rs"]
mod threadid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Waiter {
    pub thread_id: ThreadId,
    pub parked_key: ParkKey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Queue {
    pub key: ParkKey,
    pub waiters: Vec<Waiter>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Table {
    pub queues: Vec<Queue>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequeueResult {
    pub woken: Option<Waiter>,
    pub moved: usize,
    pub table: Table,
}

// r[impl domain-primitives.parking-lot-core.queue-validity] r[impl newtypes.parking-lot-api]
pub fn waiter_matches_key(waiter: &Waiter, key: ParkKey) -> bool {
    waiter.parked_key == key
}

// r[impl domain-primitives.parking-lot-core.queue-validity] r[impl newtypes.parking-lot-api]
pub fn waiter_count(table: &Table, thread_id: ThreadId) -> u64 {
    let mut count = 0u64;
    for queue in &table.queues {
        for waiter in &queue.waiters {
            if waiter.thread_id == thread_id {
                count += 1;
            }
        }
    }
    count
}

// r[impl domain-primitives.parking-lot-core.queue-validity] r[impl newtypes.parking-lot-api]
pub fn has_waiter(table: &Table, thread_id: ThreadId) -> bool {
    waiter_count(table, thread_id) > 0
}

/// Queue identity for the finite parking-lot model.
///
/// ```compile_fail
/// use verified_logic::parking_lot_core_model::{ParkKey, Table, ThreadId, park_waiter};
/// let table = Table { queues: Vec::new() };
/// let thread = ThreadId::new(1);
/// let _ = park_waiter(&table, thread, thread);
/// ```
// r[impl newtypes.parking-lot-identities] r[impl newtypes.parking-lot-evidence]
// r[impl newtypes.parking-lot-docs] r[impl newtypes.parking-lot-non-claims]
// r[impl newtypes.parking-lot-validation]
pub type ParkKey = identitykey::ParkKey;

/// Thread identity for finite waiters and raw-mutex owners.
///
/// ```compile_fail
/// use verified_logic::parking_lot_core_model::{ParkKey, Table, park_waiter};
/// let table = Table { queues: Vec::new() };
/// let key = ParkKey::new(1);
/// let _ = park_waiter(&table, key, key);
/// ```
// r[impl newtypes.parking-lot-identities] r[impl newtypes.parking-lot-evidence]
pub type ThreadId = threadid::ThreadId;

// r[impl domain-primitives.parking-lot-core.queue-validity]
pub fn queue_waiters_match_key(queue: &Queue) -> bool {
    for waiter in &queue.waiters {
        if !waiter_matches_key(waiter, queue.key) {
            return false;
        }
    }
    true
}

// r[impl domain-primitives.parking-lot-core.queue-validity]
pub fn queues_have_unique_keys(table: &Table) -> bool {
    for i in 0..table.queues.len() {
        for j in i + 1..table.queues.len() {
            if table.queues[i].key == table.queues[j].key {
                return false;
            }
        }
    }
    true
}

// r[impl domain-primitives.parking-lot-core.queue-validity]
pub fn waiters_unique(table: &Table) -> bool {
    for i in 0..table.queues.len() {
        for wi in 0..table.queues[i].waiters.len() {
            let id = table.queues[i].waiters[wi].thread_id;
            let mut seen = 0usize;
            for queue in &table.queues {
                for waiter in &queue.waiters {
                    if waiter.thread_id == id {
                        seen += 1;
                    }
                }
            }
            if seen != 1 {
                return false;
            }
        }
    }
    true
}

// r[impl domain-primitives.parking-lot-core.queue-validity]
pub fn table_valid(table: &Table) -> bool {
    if !queues_have_unique_keys(table) || !waiters_unique(table) {
        return false;
    }
    for queue in &table.queues {
        if !queue_waiters_match_key(queue) {
            return false;
        }
    }
    true
}

fn find_queue_index(table: &Table, key: ParkKey) -> Option<usize> {
    table.queues.iter().position(|queue| queue.key == key)
}

fn queue_for_key_mut(table: &mut Table, key: ParkKey) -> &mut Queue {
    if let Some(index) = find_queue_index(table, key) {
        &mut table.queues[index]
    } else {
        table.queues.push(Queue {
            key,
            waiters: Vec::new(),
        });
        let last = table.queues.len() - 1;
        &mut table.queues[last]
    }
}

// r[impl domain-primitives.parking-lot-core.operations] r[impl newtypes.parking-lot-api] r[impl
// newtypes.parking-lot-semantics]
pub fn park_waiter(table: &Table, key: ParkKey, thread_id: ThreadId) -> Table {
    let mut next = table.clone();
    if has_waiter(&next, thread_id) {
        return next;
    }
    queue_for_key_mut(&mut next, key).waiters.push(Waiter {
        thread_id,
        parked_key: key,
    });
    next
}

// r[impl domain-primitives.parking-lot-core.operations] r[impl newtypes.parking-lot-api]
pub fn unpark_one(table: &Table, key: ParkKey) -> (Option<Waiter>, Table) {
    let mut next = table.clone();
    let Some(index) = find_queue_index(&next, key) else {
        return (None, next);
    };
    if next.queues[index].waiters.is_empty() {
        return (None, next);
    }
    let waiter = next.queues[index].waiters.remove(0);
    (Some(waiter), next)
}

// r[impl domain-primitives.parking-lot-core.operations] r[impl newtypes.parking-lot-api]
pub fn unpark_all(table: &Table, key: ParkKey) -> (Vec<Waiter>, Table) {
    let mut next = table.clone();
    let Some(index) = find_queue_index(&next, key) else {
        return (Vec::new(), next);
    };
    let removed = std::mem::take(&mut next.queues[index].waiters);
    (removed, next)
}

// r[impl domain-primitives.parking-lot-core.operations] r[impl newtypes.parking-lot-api] r[impl
// newtypes.parking-lot-semantics]
pub fn requeue_after_wake(table: &Table, from: ParkKey, to: ParkKey) -> RequeueResult {
    let (woken, after_wake) = unpark_one(table, from);
    let (mut moving, mut next) = unpark_all(&after_wake, from);
    let moved = moving.len();
    if moved > 0 {
        let dest = queue_for_key_mut(&mut next, to);
        for mut waiter in moving.drain(..) {
            waiter.parked_key = to;
            dest.waiters.push(waiter);
        }
    }
    RequeueResult {
        woken,
        moved,
        table: next,
    }
}

// r[impl domain-primitives.parking-lot-core.operations]
pub fn requeue_preserves_waiter_count(before: &Table, after: &RequeueResult) -> bool {
    let before_count: usize = before.queues.iter().map(|queue| queue.waiters.len()).sum();
    let after_count: usize = after.table.queues.iter().map(|queue| queue.waiters.len()).sum();
    let woken_count = usize::from(after.woken.is_some());
    before_count == after_count + woken_count
}

// r[verify domain-primitives.parking-lot-core.queue-validity]
// r[impl domain-primitives.parking-lot-core.queue-validity]
pub fn valid_table_has_unique_waiters(table: &Table) -> bool {
    !table_valid(table) || waiters_unique(table)
}

// r[verify domain-primitives.parking-lot-core.operations] r[verify newtypes.parking-lot-semantics]
// r[impl domain-primitives.parking-lot-core.operations]
pub fn park_preserves_validity_when_absent(table: &Table, key: ParkKey, thread_id: ThreadId) -> bool {
    if !table_valid(table) || has_waiter(table, thread_id) {
        true
    } else {
        table_valid(&park_waiter(table, key, thread_id))
    }
}

// r[verify domain-primitives.parking-lot-core.operations] r[verify newtypes.parking-lot-semantics]
// r[impl domain-primitives.parking-lot-core.operations]
pub fn requeue_preserves_validity(table: &Table, from: ParkKey, to: ParkKey) -> bool {
    if !table_valid(table) {
        true
    } else {
        let result = requeue_after_wake(table, from, to);
        table_valid(&result.table) && requeue_preserves_waiter_count(table, &result)
    }
}
