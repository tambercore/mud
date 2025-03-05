module AManLikesBigYellowCheese where

open import Data.Product

postulate
  Entity : Set
  isMan : Entity → Set
  isCheese : Entity → Set
  yellow : Entity → Set
  big : Entity → Set
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


record YellowCheeseᵣ : Set where
  constructor YellowCheese꜀
  field
    e₁ : Cheeseᵣ
    p : yellow (Cheeseᵣ.e₁ e₁)


record BigYellowCheeseᵣ : Set where
  constructor BigYellowCheese꜀
  field
    e₁ : YellowCheeseᵣ
    p : big (Cheeseᵣ.e₁ (YellowCheeseᵣ.e₁ e₁))


record LikesManBigYellowCheeseᵣ : Set where
  constructor LikesManBigYellowCheese꜀
  field
    e₁ : Manᵣ
    e₂ : BigYellowCheeseᵣ
    p : likes (Manᵣ.e₁ e₁) (Cheeseᵣ.e₁ (YellowCheeseᵣ.e₁ (BigYellowCheeseᵣ.e₁ e₂)))

