module newagda where

{- Entity : Set -}
postulate
  Entity         : Set
  isTomato       : Entity → Set
  isVegetable    : Entity → Set
  tomatoProofIsVegProof : ∀ (e : Entity) → isTomato e → isVegetable e

record Tomato : Set where
  constructor mkTomato
  field
    e₁            : Entity
    proofTomato   : isTomato e₁

record Vegetable : Set where
  constructor mkVegetable
  field
    e₁            : Entity
    proofVegeta   : isVegetable e₁

f : Tomato → Vegetable
f = λ z →
    mkVegetable (z .Tomato.e₁)
    (tomatoProofIsVegProof (z .Tomato.e₁) (z .Tomato.proofTomato))