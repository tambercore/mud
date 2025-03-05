module AManLikesCheese where

open import Data.Product

postulate
  Entity : Set
  isMan : Entity → Set
  isCheese : Entity → Set
  likes : Entity → Entity → Set


record Manᵣ : Set where
  constructor Man꜀
  field
    e₁ : Entity
    p₁ : isMan e₁


record Cheeseᵣ : Set where
  constructor Cheese꜀
  field
    e₁ : Entity
    p₁ : isCheese e₁


record LikesManCheeseᵣ : Set where
  constructor LikesManCheese꜀
  field
    e₁ : Manᵣ
    e₂ : Cheeseᵣ
    p : likes (Manᵣ.e₁ e₁) (Cheeseᵣ.e₁ e₂)

