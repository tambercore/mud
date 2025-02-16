{-# OPTIONS --cubical #-}

module hott_adjectives where

open import Cubical.Core.Everything         -- Core Cubical Agda functionality
open import Cubical.Foundations.Everything  -- Includes ≡ (Path types)
open import Cubical.Data.Bool               -- If using Bool types
open import Cubical.Relation.Nullary  -- If dealing with decidable equality
open import Cubical.Foundations.Prelude  -- Includes identity types

-- Declare the base type for objects
postulate
  Entity : Set

-- Declare predicates for properties of entities
postulate
  isElephant : Entity → Set
  isLarge    : Entity → Set
  isBig      : Entity → Set
  isAnimal   : Entity → Set
  
  bigLargePath : (e : Entity) → isLarge e ≡ isBig e
  --  bigLargePath : isLarge ≡ isBig


-- Define a record for Elephants
record Elephant : Set where
  constructor elephant
  field
    e₁            : Entity
    proofElephant : isElephant e₁

-- Define a record for Big Elephants explicitly, without `extends`
record BigElephant : Set where
  constructor bigElephant
  field
    e₁            : Entity
    proofElephant : isElephant e₁
    proofBig      : isBig e₁  -- Explicitly include `e₁` and `proofElephant`

-- Define a record for Large Elephants explicitly, without `extends`
record LargeElephant : Set where
  constructor largeElephant
  field
    e₁            : Entity
    proofElephant : isElephant e₁
    proofLarge    : isLarge e₁  -- Explicitly include `e₁` and `proofElephant`

-- Convert a Big Elephant to a Large Elephant
bigToLarge : BigElephant → LargeElephant
bigToLarge be =
  record
    { e₁ = BigElephant.e₁ be
    ; proofElephant = BigElephant.proofElephant be
    ; proofLarge = transport (sym (bigLargePath (BigElephant.e₁ be))) (BigElephant.proofBig be)
    }

-- Define a record for Animals
record Animal : Set where
  constructor animal
  field
    e₁          : Entity
    proofAnimal : isAnimal e₁

-- Assume that all Elephants are Animals
postulate
  elephantImpliesAnimal : (e : Entity) → isElephant e → isAnimal e

-- Convert an Elephant to an Animal
elephantToAnimal : Elephant → Animal
elephantToAnimal ele = animal (Elephant.e₁ ele)
                              (elephantImpliesAnimal (Elephant.e₁ ele) (Elephant.proofElephant ele)) 