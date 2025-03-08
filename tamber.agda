module tamber.agda where

open import Data.Product

postulate
  Entity : Set
  isAmber : Entity → Set
  isToby : Entity → Set
  loves : Entity → Entity → Set


record Amberᵣ : Set where
  constructor Amber꜀
  field
    e₁ : Entity
    p₁ : isAmber e₁


record Tobyᵣ : Set where
  constructor Toby꜀
  field
    e₁ : Entity
    p₁ : isToby e₁


record LovesAmberTobyᵣ : Set where
  constructor LovesAmberToby꜀
  field
    e₁ : Amberᵣ
    e₂ : Tobyᵣ
    p : loves (Amberᵣ.e₁ e₁) (Tobyᵣ.e₁ e₂)

