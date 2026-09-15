import OrderingMin

/-- The generated min is commutative, as the ordering.min requirement states. -/
theorem trellis_min_commutative (a b : Aeneas.Std.U64) : verified_logic.ordering.min a b = verified_logic.ordering.min b a := by
  unfold verified_logic.ordering.min
  by_cases hab : a ≤ b
  · by_cases hba : b ≤ a
    · have equal : a = b := le_antisymm hab hba
      subst b
      rfl
    · simp [hab, hba]
  · have hba : b ≤ a := le_of_lt (lt_of_not_ge hab)
    simp [hab, hba]
