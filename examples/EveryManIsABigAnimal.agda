module EveryManIsABigAnimal where

open import Data.Product

postulate
  Entity : Set
  isMan : Entity → Set
  isAnimal : Entity → Set
  isBig : Entity → Set


record Manᵣ : Set where
  constructor Man꜀
  field
    e₁ : Entity
    p₁ : isMan e₁


record IsManBigAnimalᵣ : Set where
  constructor IsManBigAnimal꜀
  field
    p : (a₁ : Manᵣ) → isAnimal (Manᵣ.e₁ a₁) × isBig (Manᵣ.e₁ a₁)

