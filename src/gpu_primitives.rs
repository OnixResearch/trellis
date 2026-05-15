//! Finite GPU programming primitive admission checks.
use vstd::prelude::*;

verus! {
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuBlockDim {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub max_threads_per_block: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuGridDim {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub max_blocks: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuThreadIndex {
    pub thread_x: u16,
    pub thread_y: u16,
    pub thread_z: u16,
    pub block_x: u16,
    pub block_y: u16,
    pub block_z: u16,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuMemoryAccess {
    pub base: u64,
    pub lane_count: u16,
    pub element_bytes: u16,
    pub segment_bytes: u32,
    pub buffer_bytes: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuSharedTile {
    pub rows: u16,
    pub cols: u16,
    pub element_bytes: u16,
    pub capacity_bytes: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuLaunchEnvelope {
    pub block: GpuBlockDim,
    pub grid: GpuGridDim,
    pub shared_tile: GpuSharedTile,
    pub max_shared_bytes_per_block: u64,
}

// r[depends gpu.block-dim]
pub open spec fn gpu_block_thread_count_spec(b: GpuBlockDim) -> nat {
    (b.x as nat) * (b.y as nat) * (b.z as nat)
}

// r[depends gpu.block-dim]
pub open spec fn gpu_block_dim_spec(b: GpuBlockDim) -> bool {
    b.x > 0 && b.y > 0 && b.z > 0 && b.max_threads_per_block > 0
        && gpu_block_thread_count_spec(b) <= b.max_threads_per_block as nat
}

// r[impl gpu.block-dim]
pub fn gpu_block_dim_valid(b: GpuBlockDim) -> (result: bool)
    ensures result == gpu_block_dim_spec(b)
{
    assert((b.x as u64) * (b.y as u64) <= u64::MAX) by(nonlinear_arith);
    assert((b.x as u64) * (b.y as u64) * (b.z as u64) <= u64::MAX) by(nonlinear_arith);
    b.x > 0 && b.y > 0 && b.z > 0 && b.max_threads_per_block > 0
        && (b.x as u64) * (b.y as u64) * (b.z as u64) <= b.max_threads_per_block as u64
}

// r[verify gpu.block-dim]
pub proof fn valid_gpu_block_dim_is_admitted(b: GpuBlockDim)
    requires gpu_block_dim_spec(b)
    ensures gpu_block_dim_spec(b)
{
}

// r[depends gpu.grid-dim]
pub open spec fn gpu_grid_block_count_spec(g: GpuGridDim) -> nat {
    (g.x as nat) * (g.y as nat) * (g.z as nat)
}

// r[depends gpu.grid-dim]
pub open spec fn gpu_grid_dim_spec(g: GpuGridDim) -> bool {
    g.x > 0 && g.y > 0 && g.z > 0 && g.max_blocks > 0
        && gpu_grid_block_count_spec(g) <= g.max_blocks as nat
}

// r[impl gpu.grid-dim]
pub fn gpu_grid_dim_valid(g: GpuGridDim) -> (result: bool)
    ensures result == gpu_grid_dim_spec(g)
{
    assert((g.x as u64) * (g.y as u64) <= u64::MAX) by(nonlinear_arith);
    assert((g.x as u64) * (g.y as u64) * (g.z as u64) <= u64::MAX) by(nonlinear_arith);
    g.x > 0 && g.y > 0 && g.z > 0 && g.max_blocks > 0
        && (g.x as u64) * (g.y as u64) * (g.z as u64) <= g.max_blocks
}

// r[verify gpu.grid-dim]
pub proof fn valid_gpu_grid_dim_is_admitted(g: GpuGridDim)
    requires gpu_grid_dim_spec(g)
    ensures gpu_grid_dim_spec(g)
{
}

// r[depends gpu.thread-index]
pub open spec fn gpu_thread_index_spec(i: GpuThreadIndex, b: GpuBlockDim, g: GpuGridDim) -> bool {
    gpu_block_dim_spec(b) && gpu_grid_dim_spec(g)
        && i.thread_x < b.x && i.thread_y < b.y && i.thread_z < b.z
        && i.block_x < g.x && i.block_y < g.y && i.block_z < g.z
}

// r[depends gpu.thread-index]
pub open spec fn gpu_local_thread_ordinal_spec(i: GpuThreadIndex, b: GpuBlockDim) -> nat {
    (i.thread_x as nat) + (b.x as nat) * ((i.thread_y as nat) + (b.y as nat) * (i.thread_z as nat))
}

// r[depends gpu.thread-index]
pub open spec fn gpu_block_ordinal_spec(i: GpuThreadIndex, g: GpuGridDim) -> nat {
    (i.block_x as nat) + (g.x as nat) * ((i.block_y as nat) + (g.y as nat) * (i.block_z as nat))
}

// r[impl gpu.thread-index]
pub fn gpu_thread_index_valid(i: GpuThreadIndex, b: GpuBlockDim, g: GpuGridDim) -> (result: bool)
    ensures result == gpu_thread_index_spec(i, b, g)
{
    gpu_block_dim_valid(b) && gpu_grid_dim_valid(g)
        && i.thread_x < b.x && i.thread_y < b.y && i.thread_z < b.z
        && i.block_x < g.x && i.block_y < g.y && i.block_z < g.z
}

// r[verify gpu.thread-index]
pub proof fn valid_gpu_thread_index_is_admitted(i: GpuThreadIndex, b: GpuBlockDim, g: GpuGridDim)
    requires gpu_thread_index_spec(i, b, g)
    ensures gpu_thread_index_spec(i, b, g)
{
}

// r[depends gpu.coalesced-access]
pub open spec fn gpu_memory_span_spec(a: GpuMemoryAccess) -> nat {
    (a.lane_count as nat) * (a.element_bytes as nat)
}

// r[depends gpu.coalesced-access]
pub open spec fn gpu_memory_access_spec(a: GpuMemoryAccess) -> bool {
    a.lane_count > 0 && a.element_bytes > 0 && a.segment_bytes > 0
        && gpu_memory_span_spec(a) <= a.segment_bytes as nat
        && a.base <= a.buffer_bytes
        && gpu_memory_span_spec(a) <= (a.buffer_bytes - a.base) as nat
}

// r[impl gpu.coalesced-access]
pub fn gpu_memory_access_valid(a: GpuMemoryAccess) -> (result: bool)
    ensures result == gpu_memory_access_spec(a)
{
    assert((a.lane_count as u64) * (a.element_bytes as u64) <= u64::MAX) by(nonlinear_arith);
    a.lane_count > 0
        && a.element_bytes > 0
        && a.segment_bytes > 0
        && (a.lane_count as u64) * (a.element_bytes as u64) <= a.segment_bytes as u64
        && a.base <= a.buffer_bytes
        && (a.lane_count as u64) * (a.element_bytes as u64) <= a.buffer_bytes - a.base
}

// r[verify gpu.coalesced-access]
pub proof fn valid_gpu_memory_access_is_in_bounds(a: GpuMemoryAccess)
    requires gpu_memory_access_spec(a)
    ensures gpu_memory_access_spec(a), gpu_memory_span_spec(a) <= a.segment_bytes as nat
{}

// r[depends gpu.shared-tile]
pub open spec fn gpu_shared_tile_bytes_spec(t: GpuSharedTile) -> nat {
    (t.rows as nat) * (t.cols as nat) * (t.element_bytes as nat)
}

// r[depends gpu.shared-tile]
pub open spec fn gpu_shared_tile_spec(t: GpuSharedTile) -> bool {
    t.rows > 0 && t.cols > 0 && t.element_bytes > 0
        && gpu_shared_tile_bytes_spec(t) <= t.capacity_bytes as nat
}

// r[impl gpu.shared-tile]
pub fn gpu_shared_tile_valid(t: GpuSharedTile) -> (result: bool)
    ensures result == gpu_shared_tile_spec(t)
{
    assert((t.rows as u64) * (t.cols as u64) <= u64::MAX) by(nonlinear_arith);
    assert((t.rows as u64) * (t.cols as u64) * (t.element_bytes as u64) <= u64::MAX) by(nonlinear_arith);
    t.rows > 0 && t.cols > 0 && t.element_bytes > 0
        && (t.rows as u64) * (t.cols as u64) * (t.element_bytes as u64) <= t.capacity_bytes
}

// r[verify gpu.shared-tile]
pub proof fn valid_gpu_shared_tile_fits_capacity(t: GpuSharedTile)
    requires gpu_shared_tile_spec(t)
    ensures gpu_shared_tile_bytes_spec(t) <= t.capacity_bytes as nat
{}

// r[depends gpu.launch-envelope]
pub open spec fn gpu_launch_envelope_spec(e: GpuLaunchEnvelope) -> bool {
    gpu_block_dim_spec(e.block) && gpu_grid_dim_spec(e.grid) && gpu_shared_tile_spec(e.shared_tile)
        && e.shared_tile.capacity_bytes <= e.max_shared_bytes_per_block
}

// r[impl gpu.launch-envelope]
pub fn gpu_launch_envelope_valid(e: GpuLaunchEnvelope) -> (result: bool)
    ensures result == gpu_launch_envelope_spec(e)
{
    gpu_block_dim_valid(e.block) && gpu_grid_dim_valid(e.grid) && gpu_shared_tile_valid(e.shared_tile)
        && e.shared_tile.capacity_bytes <= e.max_shared_bytes_per_block
}

// r[verify gpu.launch-envelope]
pub proof fn valid_gpu_launch_envelope_components_are_valid(e: GpuLaunchEnvelope)
    requires gpu_launch_envelope_spec(e)
    ensures gpu_block_dim_spec(e.block), gpu_grid_dim_spec(e.grid), gpu_shared_tile_spec(e.shared_tile)
{}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuPitchedBuffer {
    pub width: u16,
    pub height: u16,
    pub element_bytes: u16,
    pub pitch_bytes: u32,
    pub buffer_bytes: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuTensor3D {
    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub element_bytes: u16,
    pub buffer_bytes: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuTensorIndex3D {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuTileProjection {
    pub origin_x: u16,
    pub origin_y: u16,
    pub rows: u16,
    pub cols: u16,
}

// r[depends gpu.pitched-buffer]
pub open spec fn gpu_pitched_row_bytes_spec(b: GpuPitchedBuffer) -> nat {
    (b.width as nat) * (b.element_bytes as nat)
}

// r[depends gpu.pitched-buffer]
pub open spec fn gpu_pitched_buffer_spec(b: GpuPitchedBuffer) -> bool {
    b.width > 0 && b.height > 0 && b.element_bytes > 0
        && gpu_pitched_row_bytes_spec(b) <= b.pitch_bytes as nat
        && (b.pitch_bytes as nat) * (b.height as nat) <= b.buffer_bytes as nat
}

// r[impl gpu.pitched-buffer]
pub fn gpu_pitched_buffer_valid(b: GpuPitchedBuffer) -> (result: bool)
    ensures result == gpu_pitched_buffer_spec(b)
{
    assert((b.width as u64) * (b.element_bytes as u64) <= u64::MAX) by(nonlinear_arith);
    assert((b.pitch_bytes as u64) * (b.height as u64) <= u64::MAX) by(nonlinear_arith);
    b.width > 0
        && b.height > 0
        && b.element_bytes > 0
        && (b.width as u64) * (b.element_bytes as u64) <= b.pitch_bytes as u64
        && (b.pitch_bytes as u64) * (b.height as u64) <= b.buffer_bytes
}

// r[verify gpu.pitched-buffer]
pub proof fn valid_gpu_pitched_buffer_is_admitted(b: GpuPitchedBuffer)
    requires gpu_pitched_buffer_spec(b)
    ensures gpu_pitched_buffer_spec(b), gpu_pitched_row_bytes_spec(b) <= b.pitch_bytes as nat
{}

// r[depends gpu.pitched-address]
pub open spec fn gpu_pitched_offset_spec(b: GpuPitchedBuffer, x: u16, y: u16) -> nat {
    (y as nat) * (b.pitch_bytes as nat) + (x as nat) * (b.element_bytes as nat)
}

// r[depends gpu.pitched-address]
pub open spec fn gpu_pitched_address_spec(b: GpuPitchedBuffer, x: u16, y: u16) -> bool {
    gpu_pitched_buffer_spec(b) && x < b.width && y < b.height
        && (b.element_bytes as nat) <= b.buffer_bytes as nat
        && gpu_pitched_offset_spec(b, x, y) + (b.element_bytes as nat) <= b.buffer_bytes as nat
}

// r[impl gpu.pitched-address]
pub fn gpu_pitched_address_valid(b: GpuPitchedBuffer, x: u16, y: u16) -> (result: bool)
    ensures result == gpu_pitched_address_spec(b, x, y)
{
    assert((y as u64) * (b.pitch_bytes as u64) <= u64::MAX) by(nonlinear_arith);
    assert((x as u64) * (b.element_bytes as u64) <= u64::MAX) by(nonlinear_arith);
    assert((y as u64) * (b.pitch_bytes as u64) + (x as u64) * (b.element_bytes as u64) <= u64::MAX) by(nonlinear_arith);
    gpu_pitched_buffer_valid(b)
        && x < b.width
        && y < b.height
        && (b.element_bytes as u64) <= b.buffer_bytes
        && (y as u64) * (b.pitch_bytes as u64) + (x as u64) * (b.element_bytes as u64)
            <= b.buffer_bytes - b.element_bytes as u64
}

// r[verify gpu.pitched-address]
pub proof fn valid_gpu_pitched_address_is_in_bounds(b: GpuPitchedBuffer, x: u16, y: u16)
    requires gpu_pitched_address_spec(b, x, y)
    ensures gpu_pitched_offset_spec(b, x, y) + (b.element_bytes as nat) <= b.buffer_bytes as nat
{}

// r[depends gpu.tensor3d-layout]
pub open spec fn gpu_tensor3d_element_count_spec(t: GpuTensor3D) -> nat {
    (t.width as nat) * (t.height as nat) * (t.depth as nat)
}

// r[depends gpu.tensor3d-layout]
pub open spec fn gpu_tensor3d_bytes_spec(t: GpuTensor3D) -> nat {
    gpu_tensor3d_element_count_spec(t) * (t.element_bytes as nat)
}

// r[depends gpu.tensor3d-layout]
pub open spec fn gpu_tensor3d_spec(t: GpuTensor3D) -> bool {
    t.width > 0 && t.height > 0 && t.depth > 0 && t.element_bytes > 0
        && gpu_tensor3d_bytes_spec(t) <= t.buffer_bytes as nat
}

// r[impl gpu.tensor3d-layout]
pub fn gpu_tensor3d_valid(t: GpuTensor3D) -> (result: bool)
    ensures result == gpu_tensor3d_spec(t)
{
    assert((t.width as u64) * (t.height as u64) <= u64::MAX) by(nonlinear_arith);
    assert((t.width as u64) * (t.height as u64) * (t.depth as u64) <= u64::MAX) by(nonlinear_arith);
    assert((t.width as u64) * (t.height as u64) * (t.depth as u64) * (t.element_bytes as u64) <= u64::MAX) by(nonlinear_arith);
    t.width > 0
        && t.height > 0
        && t.depth > 0
        && t.element_bytes > 0
        && (t.width as u64) * (t.height as u64) * (t.depth as u64) * (t.element_bytes as u64)
            <= t.buffer_bytes
}

// r[verify gpu.tensor3d-layout]
pub proof fn valid_gpu_tensor3d_is_admitted(t: GpuTensor3D)
    requires gpu_tensor3d_spec(t)
    ensures gpu_tensor3d_spec(t), gpu_tensor3d_bytes_spec(t) <= t.buffer_bytes as nat
{}

// r[depends gpu.tensor3d-index]
pub open spec fn gpu_tensor3d_index_spec(t: GpuTensor3D, i: GpuTensorIndex3D) -> bool {
    gpu_tensor3d_spec(t) && i.x < t.width && i.y < t.height && i.z < t.depth
}

// r[depends gpu.tensor3d-index]
pub open spec fn gpu_tensor3d_offset_spec(t: GpuTensor3D, i: GpuTensorIndex3D) -> nat {
    (((i.z as nat) * (t.height as nat) + (i.y as nat)) * (t.width as nat) + (i.x as nat))
        * (t.element_bytes as nat)
}

// r[impl gpu.tensor3d-index]
pub fn gpu_tensor3d_index_valid(t: GpuTensor3D, i: GpuTensorIndex3D) -> (result: bool)
    ensures result == gpu_tensor3d_index_spec(t, i)
{
    gpu_tensor3d_valid(t) && i.x < t.width && i.y < t.height && i.z < t.depth
}

// r[verify gpu.tensor3d-index]
pub proof fn valid_gpu_tensor3d_index_is_admitted(t: GpuTensor3D, i: GpuTensorIndex3D)
    requires gpu_tensor3d_index_spec(t, i)
    ensures gpu_tensor3d_index_spec(t, i)
{}

// r[depends gpu.tile-projection]
pub open spec fn gpu_tile_projection_spec(b: GpuPitchedBuffer, tile: GpuTileProjection) -> bool {
    gpu_pitched_buffer_spec(b)
        && tile.rows > 0
        && tile.cols > 0
        && (tile.origin_x as nat) + (tile.cols as nat) <= b.width as nat
        && (tile.origin_y as nat) + (tile.rows as nat) <= b.height as nat
}

// r[impl gpu.tile-projection]
pub fn gpu_tile_projection_valid(b: GpuPitchedBuffer, tile: GpuTileProjection) -> (result: bool)
    ensures result == gpu_tile_projection_spec(b, tile)
{
    gpu_pitched_buffer_valid(b)
        && tile.rows > 0
        && tile.cols > 0
        && (tile.origin_x as u64) + (tile.cols as u64) <= b.width as u64
        && (tile.origin_y as u64) + (tile.rows as u64) <= b.height as u64
}

// r[verify gpu.tile-projection]
pub proof fn valid_gpu_tile_projection_is_admitted(b: GpuPitchedBuffer, tile: GpuTileProjection)
    requires gpu_tile_projection_spec(b, tile)
    ensures gpu_tile_projection_spec(b, tile)
{}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuElementwiseProgram {
    pub elements: u64,
    pub threads: u16,
    pub elements_per_thread: u16,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuGridStrideLoop {
    pub len: u64,
    pub start: u64,
    pub stride: u16,
    pub iterations: u16,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuTiledKernel2D {
    pub width: u16,
    pub height: u16,
    pub tile_width: u16,
    pub tile_height: u16,
    pub grid_x: u16,
    pub grid_y: u16,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuBlockReductionProgram {
    pub input_len: u64,
    pub block_threads: u16,
    pub elements_per_thread: u16,
    pub blocks: u16,
    pub element_bytes: u16,
    pub shared_capacity_bytes: u64,
}

// r[depends gpu.elementwise-program]
pub open spec fn gpu_elementwise_program_capacity_spec(p: GpuElementwiseProgram) -> nat {
    (p.threads as nat) * (p.elements_per_thread as nat)
}

// r[depends gpu.elementwise-program]
pub open spec fn gpu_elementwise_program_spec(p: GpuElementwiseProgram) -> bool {
    p.elements > 0 && p.threads > 0 && p.elements_per_thread > 0
        && p.elements as nat <= gpu_elementwise_program_capacity_spec(p)
}

// r[impl gpu.elementwise-program]
pub fn gpu_elementwise_program_valid(p: GpuElementwiseProgram) -> (result: bool)
    ensures result == gpu_elementwise_program_spec(p)
{
    assert((p.threads as u64) * (p.elements_per_thread as u64) <= u64::MAX) by(nonlinear_arith);
    p.elements > 0
        && p.threads > 0
        && p.elements_per_thread > 0
        && p.elements <= (p.threads as u64) * (p.elements_per_thread as u64)
}

// r[verify gpu.elementwise-program]
pub proof fn valid_gpu_elementwise_program_covers_elements(p: GpuElementwiseProgram)
    requires gpu_elementwise_program_spec(p)
    ensures p.elements as nat <= gpu_elementwise_program_capacity_spec(p)
{}

// r[depends gpu.grid-stride-loop]
pub open spec fn gpu_grid_stride_last_index_spec(l: GpuGridStrideLoop) -> int {
    (l.start as int) + ((l.iterations as int) - 1int) * (l.stride as int)
}

// r[depends gpu.grid-stride-loop]
pub open spec fn gpu_grid_stride_loop_spec(l: GpuGridStrideLoop) -> bool {
    l.len > 0 && l.stride > 0 && l.iterations > 0 && l.start < l.len
        && gpu_grid_stride_last_index_spec(l) < l.len as int
}

// r[impl gpu.grid-stride-loop]
pub fn gpu_grid_stride_loop_valid(l: GpuGridStrideLoop) -> (result: bool)
    ensures result == gpu_grid_stride_loop_spec(l)
{
    if !(l.len > 0 && l.stride > 0 && l.iterations > 0 && l.start < l.len) {
        false
    } else {
        assert(((l.iterations as u64) - 1u64) * (l.stride as u64) <= u64::MAX) by(nonlinear_arith);
        ((l.iterations as u64) - 1u64) * (l.stride as u64) <= (l.len - 1u64) - l.start
    }
}

// r[verify gpu.grid-stride-loop]
pub proof fn valid_gpu_grid_stride_loop_stays_in_bounds(l: GpuGridStrideLoop)
    requires gpu_grid_stride_loop_spec(l)
    ensures gpu_grid_stride_last_index_spec(l) < l.len as int
{}

// r[depends gpu.tiled-kernel2d]
pub open spec fn gpu_tiled_kernel2d_covers_spec(k: GpuTiledKernel2D) -> bool {
    (k.grid_x as nat) * (k.tile_width as nat) >= k.width as nat
        && (k.grid_y as nat) * (k.tile_height as nat) >= k.height as nat
}

// r[depends gpu.tiled-kernel2d]
pub open spec fn gpu_tiled_kernel2d_spec(k: GpuTiledKernel2D) -> bool {
    k.width > 0 && k.height > 0 && k.tile_width > 0 && k.tile_height > 0
        && k.grid_x > 0 && k.grid_y > 0 && gpu_tiled_kernel2d_covers_spec(k)
}

// r[impl gpu.tiled-kernel2d]
pub fn gpu_tiled_kernel2d_valid(k: GpuTiledKernel2D) -> (result: bool)
    ensures result == gpu_tiled_kernel2d_spec(k)
{
    assert((k.grid_x as u64) * (k.tile_width as u64) <= u64::MAX) by(nonlinear_arith);
    assert((k.grid_y as u64) * (k.tile_height as u64) <= u64::MAX) by(nonlinear_arith);
    k.width > 0
        && k.height > 0
        && k.tile_width > 0
        && k.tile_height > 0
        && k.grid_x > 0
        && k.grid_y > 0
        && (k.grid_x as u64) * (k.tile_width as u64) >= k.width as u64
        && (k.grid_y as u64) * (k.tile_height as u64) >= k.height as u64
}

// r[verify gpu.tiled-kernel2d]
pub proof fn valid_gpu_tiled_kernel2d_covers_output(k: GpuTiledKernel2D)
    requires gpu_tiled_kernel2d_spec(k)
    ensures gpu_tiled_kernel2d_covers_spec(k)
{}

// r[depends gpu.block-reduction-program]
pub open spec fn gpu_block_reduction_capacity_spec(p: GpuBlockReductionProgram) -> nat {
    (p.block_threads as nat) * (p.elements_per_thread as nat) * (p.blocks as nat)
}

// r[depends gpu.block-reduction-program]
pub open spec fn gpu_block_reduction_shared_bytes_spec(p: GpuBlockReductionProgram) -> nat {
    (p.block_threads as nat) * (p.element_bytes as nat)
}

// r[depends gpu.block-reduction-program]
pub open spec fn gpu_block_reduction_program_spec(p: GpuBlockReductionProgram) -> bool {
    p.input_len > 0 && p.block_threads > 0 && p.elements_per_thread > 0 && p.blocks > 0
        && p.element_bytes > 0
        && p.input_len as nat <= gpu_block_reduction_capacity_spec(p)
        && gpu_block_reduction_shared_bytes_spec(p) <= p.shared_capacity_bytes as nat
}

// r[impl gpu.block-reduction-program]
pub fn gpu_block_reduction_program_valid(p: GpuBlockReductionProgram) -> (result: bool)
    ensures result == gpu_block_reduction_program_spec(p)
{
    assert((p.block_threads as u64) * (p.elements_per_thread as u64) <= u64::MAX) by(nonlinear_arith);
    assert((p.block_threads as u64) * (p.elements_per_thread as u64) * (p.blocks as u64) <= u64::MAX) by(nonlinear_arith);
    assert((p.block_threads as u64) * (p.element_bytes as u64) <= u64::MAX) by(nonlinear_arith);
    p.input_len > 0
        && p.block_threads > 0
        && p.elements_per_thread > 0
        && p.blocks > 0
        && p.element_bytes > 0
        && p.input_len
            <= (p.block_threads as u64) * (p.elements_per_thread as u64) * (p.blocks as u64)
        && (p.block_threads as u64) * (p.element_bytes as u64) <= p.shared_capacity_bytes
}

// r[verify gpu.block-reduction-program]
pub proof fn valid_gpu_block_reduction_program_has_capacity(p: GpuBlockReductionProgram)
    requires gpu_block_reduction_program_spec(p)
    ensures
        p.input_len as nat <= gpu_block_reduction_capacity_spec(p),
        gpu_block_reduction_shared_bytes_spec(p) <= p.shared_capacity_bytes as nat
{}

} // verus!

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_block() -> GpuBlockDim {
        GpuBlockDim {
            x: 16,
            y: 8,
            z: 1,
            max_threads_per_block: 256,
        }
    }

    fn valid_grid() -> GpuGridDim {
        GpuGridDim {
            x: 8,
            y: 4,
            z: 2,
            max_blocks: 64,
        }
    }

    #[test]
    fn block_dim_accepts_positive_product_under_limit() {
        assert!(gpu_block_dim_valid(valid_block()));
    }

    #[test]
    fn block_dim_rejects_zero_or_too_many_threads() {
        assert!(!gpu_block_dim_valid(GpuBlockDim {
            x: 0,
            y: 8,
            z: 1,
            max_threads_per_block: 256
        }));
        assert!(!gpu_block_dim_valid(GpuBlockDim {
            x: 32,
            y: 32,
            z: 2,
            max_threads_per_block: 1024
        }));
    }

    #[test]
    fn thread_index_checks_block_and_thread_bounds() {
        let i = GpuThreadIndex {
            thread_x: 15,
            thread_y: 7,
            thread_z: 0,
            block_x: 7,
            block_y: 3,
            block_z: 1,
        };
        assert!(gpu_thread_index_valid(i, valid_block(), valid_grid()));
        assert!(!gpu_thread_index_valid(GpuThreadIndex { thread_x: 16, ..i }, valid_block(), valid_grid()));
        assert!(!gpu_thread_index_valid(GpuThreadIndex { block_z: 2, ..i }, valid_block(), valid_grid()));
    }

    #[test]
    fn memory_access_requires_segment_and_buffer_fit() {
        assert!(gpu_memory_access_valid(GpuMemoryAccess {
            base: 128,
            lane_count: 32,
            element_bytes: 4,
            segment_bytes: 128,
            buffer_bytes: 4096
        }));
        assert!(!gpu_memory_access_valid(GpuMemoryAccess {
            base: 128,
            lane_count: 32,
            element_bytes: 8,
            segment_bytes: 128,
            buffer_bytes: 4096
        }));
        assert!(!gpu_memory_access_valid(GpuMemoryAccess {
            base: 4000,
            lane_count: 32,
            element_bytes: 4,
            segment_bytes: 128,
            buffer_bytes: 4096
        }));
    }

    #[test]
    fn launch_envelope_requires_shared_tile_within_launch_limit() {
        let tile = GpuSharedTile {
            rows: 16,
            cols: 16,
            element_bytes: 4,
            capacity_bytes: 1024,
        };
        assert!(gpu_launch_envelope_valid(GpuLaunchEnvelope {
            block: valid_block(),
            grid: valid_grid(),
            shared_tile: tile,
            max_shared_bytes_per_block: 2048
        }));
        assert!(!gpu_launch_envelope_valid(GpuLaunchEnvelope {
            block: valid_block(),
            grid: valid_grid(),
            shared_tile: tile,
            max_shared_bytes_per_block: 512
        }));
    }
    fn valid_pitched_buffer() -> GpuPitchedBuffer {
        GpuPitchedBuffer {
            width: 64,
            height: 32,
            element_bytes: 4,
            pitch_bytes: 256,
            buffer_bytes: 8192,
        }
    }

    #[test]
    fn pitched_buffer_requires_pitch_and_backing_capacity() {
        assert!(gpu_pitched_buffer_valid(valid_pitched_buffer()));
        assert!(!gpu_pitched_buffer_valid(GpuPitchedBuffer {
            pitch_bytes: 128,
            ..valid_pitched_buffer()
        }));
        assert!(!gpu_pitched_buffer_valid(GpuPitchedBuffer {
            buffer_bytes: 4096,
            ..valid_pitched_buffer()
        }));
    }

    #[test]
    fn pitched_address_checks_coordinate_bounds_and_final_element() {
        let b = valid_pitched_buffer();
        assert!(gpu_pitched_address_valid(b, 63, 31));
        assert!(!gpu_pitched_address_valid(b, 64, 31));
        assert!(!gpu_pitched_address_valid(b, 63, 32));
    }

    #[test]
    fn tensor3d_layout_and_index_are_finite() {
        let t = GpuTensor3D {
            width: 8,
            height: 4,
            depth: 2,
            element_bytes: 4,
            buffer_bytes: 256,
        };
        assert!(gpu_tensor3d_valid(t));
        assert!(gpu_tensor3d_index_valid(t, GpuTensorIndex3D { x: 7, y: 3, z: 1 }));
        assert!(!gpu_tensor3d_index_valid(t, GpuTensorIndex3D { x: 8, y: 3, z: 1 }));
        assert!(!gpu_tensor3d_valid(GpuTensor3D { buffer_bytes: 255, ..t }));
    }

    #[test]
    fn tile_projection_must_fit_inside_pitched_buffer() {
        let b = valid_pitched_buffer();
        assert!(gpu_tile_projection_valid(b, GpuTileProjection {
            origin_x: 16,
            origin_y: 8,
            rows: 8,
            cols: 16
        }));
        assert!(!gpu_tile_projection_valid(b, GpuTileProjection {
            origin_x: 60,
            origin_y: 8,
            rows: 8,
            cols: 8
        }));
        assert!(!gpu_tile_projection_valid(b, GpuTileProjection {
            origin_x: 16,
            origin_y: 30,
            rows: 4,
            cols: 8
        }));
    }

    #[test]
    fn elementwise_program_requires_capacity_for_all_elements() {
        assert!(gpu_elementwise_program_valid(GpuElementwiseProgram {
            elements: 1024,
            threads: 256,
            elements_per_thread: 4
        }));
        assert!(!gpu_elementwise_program_valid(GpuElementwiseProgram {
            elements: 1025,
            threads: 256,
            elements_per_thread: 4
        }));
        assert!(!gpu_elementwise_program_valid(GpuElementwiseProgram {
            elements: 1024,
            threads: 0,
            elements_per_thread: 4
        }));
    }

    #[test]
    fn grid_stride_loop_last_index_must_stay_in_bounds() {
        assert!(gpu_grid_stride_loop_valid(GpuGridStrideLoop {
            len: 100,
            start: 3,
            stride: 8,
            iterations: 13
        }));
        assert!(!gpu_grid_stride_loop_valid(GpuGridStrideLoop {
            len: 100,
            start: 3,
            stride: 8,
            iterations: 14
        }));
        assert!(!gpu_grid_stride_loop_valid(GpuGridStrideLoop {
            len: 100,
            start: 100,
            stride: 8,
            iterations: 1
        }));
    }

    #[test]
    fn tiled_kernel2d_grid_must_cover_output_domain() {
        assert!(gpu_tiled_kernel2d_valid(GpuTiledKernel2D {
            width: 100,
            height: 65,
            tile_width: 16,
            tile_height: 16,
            grid_x: 7,
            grid_y: 5
        }));
        assert!(!gpu_tiled_kernel2d_valid(GpuTiledKernel2D {
            width: 100,
            height: 65,
            tile_width: 16,
            tile_height: 16,
            grid_x: 6,
            grid_y: 5
        }));
        assert!(!gpu_tiled_kernel2d_valid(GpuTiledKernel2D {
            width: 100,
            height: 65,
            tile_width: 16,
            tile_height: 0,
            grid_x: 7,
            grid_y: 5
        }));
    }

    #[test]
    fn block_reduction_program_requires_input_and_shared_capacity() {
        assert!(gpu_block_reduction_program_valid(GpuBlockReductionProgram {
            input_len: 4096,
            block_threads: 256,
            elements_per_thread: 4,
            blocks: 4,
            element_bytes: 4,
            shared_capacity_bytes: 1024
        }));
        assert!(!gpu_block_reduction_program_valid(GpuBlockReductionProgram {
            input_len: 4097,
            block_threads: 256,
            elements_per_thread: 4,
            blocks: 4,
            element_bytes: 4,
            shared_capacity_bytes: 1024
        }));
        assert!(!gpu_block_reduction_program_valid(GpuBlockReductionProgram {
            input_len: 4096,
            block_threads: 256,
            elements_per_thread: 4,
            blocks: 4,
            element_bytes: 4,
            shared_capacity_bytes: 512
        }));
    }
}
