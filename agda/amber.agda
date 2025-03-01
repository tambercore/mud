module amber where


{- Postulate proofs of what it means to be cake, fruit or amber -}
postulate
  Entity       : Set
  isCake       : Entity → Set
  isFruit      : Entity → Set
  isAmber      : Entity → Set
  isWalnutCake : Entity → Set
  Likes        : Entity → Entity → Set


{- We can introduce some semantics here -}
record Fruit : Set where
    constructor fruit
    field
        e₁      : Entity
        pFruit  : isFruit e₁


record Cake : Set where
    constructor cake
    field
        e₁      : Entity
        pCake   : isCake e₁


record Amber : Set where
    constructor amber
    field
        e₁      : Entity
        pAmber  : isAmber e₁


{- The whole sentence can be represented as -}
record LikesCakeAndFruit : Set where
  constructor likesBoth
  field
    Ind₁   : Amber
    pCake  : (x : Cake) → Likes (Amber.e₁ Ind₁) (Cake.e₁ x)
    pFruit : (x : Fruit) → Likes (Amber.e₁ Ind₁) (Fruit.e₁ x)
