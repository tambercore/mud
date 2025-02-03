module contractible where

open import Agda.Builtin.Sigma

-- We postulate a type for individuals, and constants/families for jack,
-- the murder relation, and the craziness predicate.
postulate
  Individual : Set
  jack : Individual
  murdered : Individual → Individual → Set
  crazy : Individual → Set

-- Here we express that there is some individual who murdered jack.
-- The type 'Σ Individual (λ x → murdered x jack)' is the type of pairs
-- consisting of an individual and a proof that they murdered Jack.
postulate
  murderer : Σ Individual (λ x → murdered x jack)

-- This postulate states that for any individual x, if x murdered jack,
-- then x is crazy.
postulate
  crazyMurderer : ∀ {x : Individual} → murdered x jack → crazy x

-- Now let's postulate that John murdered jack.
postulate
  john : Individual
  johnMurderedJack : murdered john jack

-- Now we can prove that John is crazy, since he murdered jack, and whoever murdered jack is crazy.
ProofCrazyJohn : crazy john
ProofCrazyJohn = crazyMurderer johnMurderedJack
