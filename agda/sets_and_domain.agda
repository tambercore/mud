open import Agda.Builtin.Sigma
open import Data.Product using (Σ; _,_; proj₁; proj₂)

-- Open-ended domain of individuals and a predicate for being a man
postulate
  Individual : Set
  jack brian : Individual
  isMan : Individual → Set
  jackIsMan  : isMan jack
  brianIsMan : isMan brian

-- The type of men as a subtype of Individual
Man : Set
Man = Σ Individual isMan

-- Introducing cheese and the likes relation
postulate
  cheese : Individual
  likes  : Individual → Individual → Set

-- Every man likes cheese
postulate
  everyManLikesCheese : ∀ (m : Man) → likes (proj₁ m) cheese

-- Given that 'every' man likes cheese, we can derive that John & Brian likes cheese.
-- using the proof that `jack` is a man, and the proof that `brian` is a man.
jackLikesCheese : likes jack cheese
jackLikesCheese = everyManLikesCheese (jack , jackIsMan)

brianLikesCheese : likes brian cheese
brianLikesCheese = everyManLikesCheese (brian , brianIsMan) 