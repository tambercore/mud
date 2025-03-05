module AManLikesGoudaAndBrie where

open import Data.Product

postulate
  Entity : Set
  isMan : Entity → Set
  isGouda : Entity → Set
  likes : Entity → Entity → Set
  isBrie : Entity → Set


record Manᵣ : Set where
  constructor Man꜀
  field
    e₁ : Entity
    p₁ : isMan e₁


record Goudaᵣ : Set where
  constructor Gouda꜀
  field
    e₁ : Entity
    p₁ : isGouda e₁


record LikesManGoudaᵣ : Set where
  constructor LikesManGouda꜀
  field
    e₁ : Manᵣ
    e₂ : Goudaᵣ
    p : likes (Manᵣ.e₁ e₁) (Goudaᵣ.e₁ e₂)


record Brieᵣ : Set where
  constructor Brie꜀
  field
    e₁ : Entity
    p₁ : isBrie e₁


record LikesManBrieᵣ : Set where
  constructor LikesManBrie꜀
  field
    e₁ : Manᵣ
    e₂ : Brieᵣ
    p : likes (Manᵣ.e₁ e₁) (Brieᵣ.e₁ e₂)


record LikesManGouda×LikesManBrieᵣ : Set where
  constructor LikesManGouda×LikesManBrie꜀
  field
    e₁ : LikesManGoudaᵣ
    e₂ : LikesManBrieᵣ

