{-# OPTIONS --cubical #-}

module hit_adjectives where

open import Cubical.Core.Everything
open import Cubical.Foundations.Everything
open import Cubical.Data.Bool
open import Cubical.Foundations.Prelude
open import Cubical.Relation.Nullary
open import Cubical.HITs.SetTruncation



-- Base type for entities
postulate
  Entity : Set
  isElephant : Entity → Set  -- Define Elephant predicate



-- Define a HIT that encodes "big" and "large" as the same property
data BigLarge (e : Entity) : Set where
  big : BigLarge e
  large : BigLarge e
  bigLargePath : big ≡ large



-- Postulate the induction principle for BigLarge
postulate
  BigLarge-ind : ∀ {e : Entity} (P : BigLarge e → Set) →
                 (p : P big) →
                 (q : P large) →
                 (transport P bigLargePath p ≡ q) →
                 (b : BigLarge e) → P b



-- Define a record for Elephants
record Elephant : Set where
  constructor elephant
  field
    e₁            : Entity
    proofElephant : isElephant e₁



-- Define a record for "Big-Large" Elephants
record BigLargeElephant : Set where
  constructor bigLargeElephant
  field
    e₁            : Entity
    proofElephant : isElephant e₁
    proofBigLarge : BigLarge e₁  -- Now using HIT instead of separate predicates



-- Convert a BigLarge proof into a normalized (large) proof using the recursor.
bigToLarge : (e : Entity) → BigLarge e → BigLarge e
bigToLarge e = BigLarge-ind (λ _ → BigLarge e)
                              large    -- value on `big`
                              large    -- value on `large`
                              refl     -- proof that transport in the constant family is trivial