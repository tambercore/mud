module ppp.agda where

open import Data.Product

postulate
  Entity : Set
  isAmber : Entity → Set
  isCake : Entity → Set
  loves : Entity → Entity → Set


record Amberᵣ : Set where
  constructor Amber꜀
  field
    e₁ : Entity
    p₁ : isAmber e₁


record Cakeᵣ : Set where
  constructor Cake꜀
  field
    e₁ : Entity
    p₁ : isCake e₁


record LovesAmberCakeᵣ : Set where
  constructor LovesAmberCake꜀
  field
    e₁ : Amberᵣ
    e₂ : Cakeᵣ
    p : loves (Amberᵣ.e₁ e₁) (Cakeᵣ.e₁ e₂)

