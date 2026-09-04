import Llbc

/-- Symmetry of the generated Trellis absolute-difference model. -/
theorem abs_diff_symmetric (a b : Aeneas.Std.U64) :
    verified_logic.arithmetic.abs_diff a b =
      verified_logic.arithmetic.abs_diff b a := by
  simp [verified_logic.arithmetic.abs_diff, Aeneas.Std.UScalar.sub]
  omega

/-- Self-difference is zero in the generated Trellis model. -/
theorem abs_diff_self_is_zero (x : Aeneas.Std.U64) :
    verified_logic.arithmetic.abs_diff x x = .ok 0#u64 := by
  simp [verified_logic.arithmetic.abs_diff, Aeneas.Std.UScalar.sub]
