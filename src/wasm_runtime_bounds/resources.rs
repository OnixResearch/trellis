//! Initialization-only allocation and explicit linear-memory growth policy.

/// Maximum abstract allocation capacity in WebAssembly pages.
pub const MAX_ALLOCATED_PAGES: u32 = u32::MAX - 1;

/// Abstract allocation lifecycle.
// r[impl wasm-runtime-bounds.allocation]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllocationPhase {
    Initializing,
    Running,
    Finished,
}

/// Finite allocation state.
// r[impl wasm-runtime-bounds.allocation]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageAllocator {
    pub phase: AllocationPhase,
    pub capacity_pages: u32,
    pub allocated_pages: u32,
}

/// Stable allocation rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllocationFault {
    InvalidState,
    PhaseClosed,
    CountOverflow,
    OutOfMemory,
}

/// Explicit memory-growth policy.
// r[impl wasm-runtime-bounds.growth-policy]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrowthPolicy {
    Allow,
    Deny,
}

/// Memory-growth result classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrowthOutcome {
    Denied,
    InvalidMemory,
    PlatformLimit,
    Applied { previous_pages: u32 },
    Rejected,
}

fn allocator_valid(allocator: PageAllocator) -> bool {
    allocator.capacity_pages <= MAX_ALLOCATED_PAGES && allocator.allocated_pages <= allocator.capacity_pages
}

/// Allocates pages monotonically during initialization only.
// r[impl wasm-runtime-bounds.allocation]
pub fn allocate_pages(allocator: PageAllocator, requested_pages: u32) -> Result<PageAllocator, AllocationFault> {
    if !allocator_valid(allocator) {
        return Err(AllocationFault::InvalidState);
    }
    if allocator.phase != AllocationPhase::Initializing {
        return Err(AllocationFault::PhaseClosed);
    }
    let allocated_pages =
        allocator.allocated_pages.checked_add(requested_pages).ok_or(AllocationFault::CountOverflow)?;
    if allocated_pages > allocator.capacity_pages {
        return Err(AllocationFault::OutOfMemory);
    }
    Ok(PageAllocator {
        phase: allocator.phase,
        capacity_pages: allocator.capacity_pages,
        allocated_pages,
    })
}

/// Moves an admitted allocator from initialization to execution.
// r[impl wasm-runtime-bounds.allocation]
pub fn begin_execution(allocator: PageAllocator) -> Result<PageAllocator, AllocationFault> {
    if !allocator_valid(allocator) {
        return Err(AllocationFault::InvalidState);
    }
    if allocator.phase != AllocationPhase::Initializing {
        return Err(AllocationFault::PhaseClosed);
    }
    Ok(PageAllocator {
        phase: AllocationPhase::Running,
        ..allocator
    })
}

/// Closes an admitted running allocator.
// r[impl wasm-runtime-bounds.allocation]
pub fn finish_execution(allocator: PageAllocator) -> Result<PageAllocator, AllocationFault> {
    if !allocator_valid(allocator) {
        return Err(AllocationFault::InvalidState);
    }
    if allocator.phase != AllocationPhase::Running {
        return Err(AllocationFault::PhaseClosed);
    }
    Ok(PageAllocator {
        phase: AllocationPhase::Finished,
        ..allocator
    })
}

fn memory_shape_admitted(memory: &crate::wasm_primitives::LinearMemory) -> bool {
    memory.pages <= crate::wasm_primitives::WASM_MAX_PAGES
        && memory.pages.checked_mul(crate::wasm_primitives::WASM_PAGE_SIZE_BYTES) == Some(memory.bytes.len())
}

/// Applies the existing checked growth primitive only when policy allows it.
// r[impl wasm-runtime-bounds.growth-policy]
pub fn apply_growth(
    memory: &mut crate::wasm_primitives::LinearMemory,
    delta_pages: u32,
    policy: GrowthPolicy,
) -> GrowthOutcome {
    if policy == GrowthPolicy::Deny {
        return GrowthOutcome::Denied;
    }
    if !memory_shape_admitted(memory) {
        return GrowthOutcome::InvalidMemory;
    }
    let previous_pages = match u32::try_from(memory.pages) {
        Ok(value) => value,
        Err(_) => return GrowthOutcome::PlatformLimit,
    };
    let platform_delta = match usize::try_from(delta_pages) {
        Ok(value) => value,
        Err(_) => return GrowthOutcome::PlatformLimit,
    };
    match crate::wasm_primitives::memory_grow(memory, platform_delta) {
        Some(_) => GrowthOutcome::Applied { previous_pages },
        None => GrowthOutcome::Rejected,
    }
}
