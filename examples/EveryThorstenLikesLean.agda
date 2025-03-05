module EveryThorstenLikesLean where

open import Data.Product

postulate
  Entity : Set
  isLean : Entity → Set
  isThorsten : Entity → Set
  likes : Entity → Entity → Set


record Leanᵣ : Set where
  constructor Lean꜀
  field
    e₁ : Entity
    p₁ : isLean e₁


record Thorstenᵣ : Set where
  constructor Thorsten꜀
  field
    e₁ : Entity
    p₁ : isThorsten e₁


record LikesThorstenLeanᵣ : Set where
  constructor LikesThorstenLean꜀
  field
    e₁ : Leanᵣ
    p : (a₁ : Thorstenᵣ) → likes (Thorstenᵣ.e₁ a₁) (Leanᵣ.e₁ e₁)

