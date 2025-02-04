{-# OPTIONS --cubical #-}

module adjectives (Noun : Set) where

open import Agda.Primitive
open import Agda.Builtin.Equality

-- Noun is now a parameter of the module.
-- You can instantiate it with any type you like in another module.

-- An adjective is modeled as a function on nouns.
Adjective : Set
Adjective = Noun → Noun

-- A type A is contractible if it has a “center” (a canonical element)
-- and every element of A is (propositionally) equal to that center.
record isContr {ℓ : Level} (A : Set ℓ) : Set ℓ where
  constructor contr
  field
    center : A
    contraction : (x : A) → center ≡ x

-- A synset is a collection of adjectives that are all synonymous.
-- This is encoded by requiring that the type of adjectives in the synset is contractible.
record SynAdjective : Set where
  constructor mkSynAdj
  field
    adjectives : Set  
    isContrAdjs : isContr adjectives

-- Given a SynAdjective, we can choose the canonical (or center) adjective.
canonicalAdj : SynAdjective → Adjective
canonicalAdj S = isContr.center (SynAdjective.isContrAdjs S)

{- Explanation:

   Suppose you have a synset S with carrier (i.e. a type) of adjectives and a proof 
   isContrAdjs : isContr (adjectives S). Then for any two adjectives a, b : adjectives S 
   we have:
   
     a ≡ isContr.center (isContrAdjs S)  and  b ≡ isContr.center (isContrAdjs S),
     
   hence a ≡ b.
   
   If you now “apply” any noun c (of type Noun) to a and b, then by function extensionality
   (which is available in Cubical Agda) you deduce that a c ≡ b c.
   
   This ensures that if, for example, Fast and Rapid belong to the same synset, then for any noun c,
   Fast c ≡ Rapid c.
-}

-- An example of a particular synset for the idea “fast”:
-- (In practice, you might want a more refined carrier than the entire type Adjective.)
FastSynset : SynAdjective
FastSynset = mkSynAdj Carrier isContrFast
  where
    -- Choose a particular adjective as the canonical one.
    fast : Adjective
    fast c = c  -- For example purposes, we leave this abstract.
               -- In a realistic setting, you would define how "fast" modifies c.

    -- Our chosen carrier is the full type of adjectives.
    Carrier : Set
    Carrier = Adjective

    -- A (dummy) proof that Carrier is contractible.
    -- In a real development, this proof would be based on restricting Carrier to adjectives
    -- that have been proven to have the same meaning as 'fast'.
    isContrFast : isContr Carrier
    isContrFast = contr fast (λ a → refl)

{- 
   Now, if you define two adjectives:

     Fast  : Adjective
     Rapid : Adjective

   and you prove (or assume) that both belong to FastSynset (i.e., they are elements of its carrier),
   then by contractibility we have

     Fast ≡ canonicalAdj FastSynset   and  Rapid ≡ canonicalAdj FastSynset,

   hence Fast ≡ Rapid. Consequently, for any noun c, Fast c ≡ Rapid c.
-}

-- Assume an abstract type for nouns.
Noun : Set
Noun = ...   -- Fill in with your actual noun type.

-- A predicate indicating that a noun is a car.
-- (For instance, this might be defined by a decidable property or by membership in a certain subset.)
IsCar : Noun → Set
IsCar x = ...   -- Provide a definition or leave it abstract.

-- A predicate indicating that a noun is fast.
IsFast : Noun → Set
IsFast x = ...  -- Provide a definition or leave it abstract.

-- Now we can represent the noun phrase "Fast Car" as a record type.
-- The idea is that a "FastCar" consists of an underlying noun x along with:
--   - a proof that x is a car,
--   - and a proof that x is fast.
record FastCar : Set where
  constructor mkFastCar
  field
    x      : Noun
    carPrf : IsCar x
    fastPrf: IsFast x

record Car : Set where
  constructor mkCar
  field
    x      : Noun
    isCar  : IsCar x

fastCarToCar : FastCar → Car
fastCarToCar fc = mkCar (FastCar.x fc) (FastCar.carPrf fc)