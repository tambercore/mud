module adjectives where

{- 
   We start by assuming that there is a type of objects, `Entity`.
   We also assume that there is some distinguished object, which we call `Elephant`
   (for instance, as a prototype or a placeholder).
-}
postulate
  Entity : Set

{- 
   We now postulate two predicates:
   - `isElephant` asserts that an entity is an elephant.
   - `isLarge` asserts that an entity is large.
   
   In a more complete system these might be defined in terms of other structures,
   but here we simply treat them as functions from `Entity` to `Set` (representing
   the type of evidence or proofs that the property holds).
-}
postulate
  isElephant : Entity → Set
  isLarge    : Entity → Set
  isAnimal   : Entity → Set
  p          : isLarge \equiv isBig

{- 
   Finally, we bundle a `large` elephant into a single record type.
   A value of type `LargeElephant` consists of:
     - an entity `e₁ : Entity`,
     - a proof that `e₁` is an elephant,
     - a proof that `e₁` is large.
-}
record LargeElephant : Set where
  constructor largeElephant
  field
    e₁            : Entity
    proofElephant : isElephant e₁
    proofLarge    : isLarge e₁

{-
    Now we can show an `subtype` relation between large elephants, and elephants.
    This can be shown as a function mapping any record of a large elephant back to a record of Elephant.
    1.  Define Elephant
    2.  Construct Function
-}

record Elephant : Set where
    constructor elephant
    field
        e₁              : Entity
        proofElephant   : isElephant e₁


record Point : Set where
    constructor point
    field
        x : ℕ
        y : ℕ



record BigElephant : Set where
  constructor bigElephant
  field
    e₁            : Entity
    proofElephant : isElephant e₁
    proofBig      : isBig e₁

{- Great, now we can show that given some Large Elephant, we can construct an Elephant -}
largeElephantToElephant : LargeElephant → Elephant
largeElephantToElephant le = elephant (LargeElephant.e₁ le) (LargeElephant.proofElephant le)

record Animal : Set where
    constructor animal
    field
        e₁              : Entity
        proofAnimal     : isAnimal e₁

{-
    But what if we have something expecting an Animal - an elephant is an animal surely!
    We can do this by postulating a directional path between proof's of Elephant and proof's of Animal
-}
postulate
    elephantImpliesAnimal : (e : Entity) → isElephant e → isAnimal e

ElephantToAnimal : Elephant → Animal
ElephantToAnimal ele = animal (Elephant.e₁ ele)
                              (elephantImpliesAnimal (Elephant.e₁ ele) (Elephant.proofElephant ele))