import BoundsClamp

/-- The generated clamp returns the specification selection. -/
theorem trellis_clamp_selection (value lo hi : Aeneas.Std.U64) : verified_logic.bounds.clamp value lo hi = .ok (if value < lo then lo else if value > hi then hi else value) := by
  unfold verified_logic.bounds.clamp
  by_cases h1 : value < lo
  · simp [h1]
  · by_cases h2 : value > hi
    · simp [h1, h2]
    · simp [h1, h2]
