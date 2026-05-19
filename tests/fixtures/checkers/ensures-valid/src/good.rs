verus! {
// r[impl fixture.ensures.good]
pub fn has_ensures(value: u64) -> u64
    ensures result == value
{
    value
}
} // verus!
