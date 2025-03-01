{-# OPTIONS --cubical #-}

module truncation where

open import Agda.Primitive
open import Cubical.Core.Everything
open import Cubical.Foundations.Everything
open import Cubical.Foundations.HLevels     -- Import `isSet`
open import Cubical.Data.Bool
open import Cubical.Foundations.Prelude

-- The Set Truncation of A (|-| represents truncation brackets)
data ∥_∥₀ (A : Set) : Set where
  ∣_∣ : A → ∥ A ∥₀                    -- Constructor embedding A into its truncation
  squash : (x y : ∥ A ∥₀) → isSet (∥ A ∥₀) x y  -- Use predefined `isSet`