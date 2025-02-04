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


{- Great, now we can show that given some Large Elephant, we can construct an Elephant -}
largeElephantToElephant : LargeElephant → Elephant
largeElephantToElephant le = elephant (LargeElephant.e₁ le) (LargeElephant.proofElephant le)
