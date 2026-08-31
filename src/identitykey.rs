//! Concrete queue-key identity for the parking-lot model.

// r[impl newtypes.parking-lot-identities]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ParkKey(u64);

impl ParkKey {
    // r[impl newtypes.parking-lot-identities]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    // r[impl newtypes.parking-lot-identities]
    pub const fn raw(self) -> u64 {
        self.0
    }
}
