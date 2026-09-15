import BoundsInRange

/-- The generated in-range predicate returns the specification selection. -/
theorem trellis_in_range_selection (value lo hi : Aeneas.Std.U64) : verified_logic.bounds.in_range value lo hi = .ok (if lo ≤ value then value ≤ hi else false) := by
  unfold verified_logic.bounds.in_range
  by_cases h : lo ≤ value
  · simp [h]
  · simp [h]
