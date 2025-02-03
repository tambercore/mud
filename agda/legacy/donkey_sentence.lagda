{-# OPTIONS --without-K --safe #-}

\begin{code}

module donkey_sentence where

open import Data.Product
open import Agda.Primitive
open import Data.Empty

postulate
  People  : Set
  Donkeys : Set

  John    : People
  Eyore   : Donkeys

  Owns    : People → Donkeys → Set
  Beats   : People → Donkeys → Set

  je : Owns John Eyore

  -- JohnOwnsADonkey asserts there is at least one donkey owned by John.
  JohnOwnsADonkey : Σ Donkeys (λ x → Owns John x)

  -- If John owns a donkey x, then he beats it.
  IfJohnOwnsADonkeyHeBeatsIt : (x : Donkeys) → Owns John x → Beats John x


-- **Proof:**

-- We know from `je` that `Owns John Eyore`. 
-- This lets us exhibit a particular donkey John owns, namely Eyore.
-- Thus, we can constructively prove JohnOwnsADonkey by giving Eyore and the proof je.
johnsDonkey : Σ Donkeys (λ x → Owns John x)
johnsDonkey = Eyore , je

-- From the existential `JohnOwnsADonkey`, we have a specific donkey `Eyore` that John owns.
-- Now, apply the universal conditional `IfJohnOwnsADonkeyHeBeatsIt` to Eyore and `je`.
johnBeatsThatDonkey : Beats John (proj₁ johnsDonkey)
johnBeatsThatDonkey = IfJohnOwnsADonkeyHeBeatsIt (proj₁ johnsDonkey) (proj₂ johnsDonkey)

\end{code}