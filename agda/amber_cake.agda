module amber_cake where

{- Amber likes cake -}

{- Postulate proofs of what it means to be cake, fruit or amber -}
postulate
  Entity       : Set
  isAmber      : Entity → Set
  isCake       : Entity → Set
  likes        : Entity → Entity → Set

{- We can introduce some semantics here -}
record Cake : Set where
    constructor cake
    field
        e : Entity
        p : isCake e

record Amber : Set where
    constructor amber
    field
        e : Entity
        p : isAmber e
        
record loves : Set where
  constructor c_loves
  field
    e₁ : Amber
    e₂ : Cake
    p : likes (Amber.e e₁) (Cake.e e₂)