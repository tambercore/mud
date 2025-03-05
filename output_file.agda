module output_file.agda where

open import Data.Product

postulate
  Entity : Set
  isJohn : Entity → Set
  happy : Entity → Set


record Johnᵣ : Set where
  constructor John꜀
  field
    e₁ : Entity
    p₁ : isJohn e₁


record HappyJohnᵣ : Set where
  constructor HappyJohn꜀
  field
    e₁ : Johnᵣ
    p : happy (Johnᵣ.e₁ e₁)

