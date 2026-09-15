import BoundsClamp

/-- Intentionally false. Clamp does not always return `hi`.
This file must fail to check, and it stays outside positive selection. -/
theorem trellis_clamp_always_hi (value lo hi : Aeneas.Std.U64) :
    verified_logic.bounds.clamp value lo hi = .ok hi := by
  simp [verified_logic.bounds.clamp]
