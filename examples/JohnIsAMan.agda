module JohnIsAMan where

open import Data.Product

postulate
  Entity : Set
  isJohn : Entity → Set
  isMan : Entity → Set


record Johnᵣ : Set where
  constructor John꜀
  field
    e₁ : Entity
    p₁ : isJohn e₁


record ManJohnᵣ : Set where
  constructor ManJohn꜀
  field
    e₁ : Entity
    p₁ : isJohn e₁
    p₂ : isMan e₁

