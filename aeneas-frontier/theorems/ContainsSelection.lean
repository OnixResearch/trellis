import IntervalContains

/-- The generated half-open contains returns the specification selection. -/
theorem trellis_contains_selection (lo hi x : Aeneas.Std.U64) : verified_logic.interval.contains lo hi x = .ok (if lo ≤ x then x < hi else false) := by
  unfold verified_logic.interval.contains
  by_cases h : lo ≤ x
  · simp [h]
  · simp [h]
