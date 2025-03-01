{-# OPTIONS --cubical #-}

module quest0 where

open import Cubical.Core.Everything
open import Cubical.Data.Bool
open import Cubical.HIT.S1

-- We also need a notion of equivalence (isomorphism) between types.
record _≃_ {ℓ ℓ'} (A : Type ℓ) (B : Type ℓ') : Type (ℓ ⊔ ℓ') where
  constructor equiv
  field
    to      : A → B
    from    : B → A
    leftInv : ∀ x → from (to x) ≡ x
    rightInv: ∀ y → to (from y) ≡ y

open _≃_

-- The trivial (constant) path at the base point.
Refl : base ≡ base
Refl = refl

-- The flip function on Bool.
Flip : Bool → Bool
Flip false = true
Flip true  = false

-- An equivalence from Bool to Bool.
flipIso : Bool ≃ Bool
flipIso = equiv Flip Flip (λ x → refl) (λ x → refl)

-- By univalence, any equivalence gives rise to an equality between types.
flipPath : Bool ≡ Bool
flipPath = ua flipIso

-- The double cover of the circle S¹: the fiber over the base is Bool and
-- the “monodromy” (transport along loop) is given by flipPath.
doubleCover : S¹ → Type
doubleCover = S¹-rec Bool flipPath

-- Given a path p : base ≡ base in S¹, transport along p sends true to a point
-- in the fiber over base.
endPtOfTrue : base ≡ base → doubleCover base
endPtOfTrue p = transport doubleCover p true

-- A helper to show that true is not equal to false.
true≢false : true ≡ false → ⊥
true≢false ()

-- Using the double cover we may show that the trivial path is not equal to the loop.
Refl≢loop : refl ≡ loop → ⊥
Refl≢loop p = true≢false (ap endPtOfTrue p)

{- 
  The side quests remain commented out, as in the original file.
  
  {- 
  toEmpty : (A : Type) → Type
  toEmpty A = {!!}

  pathEmpty : (A : Type) → Type₁
  pathEmpty A = {!!}

  isoEmpty : (A : Type) → Type
  isoEmpty A = {!!}

  outOf⊥ : (A : Type) → ⊥ → A
  outOf⊥ A ()

  toEmpty→isoEmpty : (A : Type) → toEmpty A → isoEmpty A
  toEmpty→isoEmpty A = {!!}

  isoEmpty→pathEmpty : (A : Type) → isoEmpty A → pathEmpty A
  isoEmpty→pathEmpty A = {!!}

  pathEmpty→toEmpty : (A : Type) → pathEmpty A → toEmpty A
  pathEmpty→toEmpty A = {!!}
  -}

  {- 
  true≢false' : true ≡ false → ⊥
  true≢false' = {!!}
  -}
-}
