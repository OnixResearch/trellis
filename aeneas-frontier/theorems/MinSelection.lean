import OrderingMin

/-- The generated min returns the specification selection. -/
theorem trellis_min_selection (a b : Aeneas.Std.U64) : verified_logic.ordering.min a b = .ok (if a ≤ b then a else b) := by
  unfold verified_logic.ordering.min
  by_cases h : a ≤ b
  · simp [h]
  · simp [h]
