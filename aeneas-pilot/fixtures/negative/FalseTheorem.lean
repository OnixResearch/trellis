import Llbc

/-- Intentionally false control kept outside positive theorem selection. -/
theorem trellis_abs_diff_self_is_one (x : Aeneas.Std.U64) : verified_logic.arithmetic.abs_diff x x = .ok 1#u64 := by
  unfold verified_logic.arithmetic.abs_diff
  rw [if_pos (le_refl x)]
  change Aeneas.Std.UScalar.sub x x = .ok 1#u64
  simp [Aeneas.Std.UScalar.sub]
