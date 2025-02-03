{-# OPTIONS --without-K #-}

\begin{code}

module ttr where

open import Agda.Primitive
open import Data.Product using (Σ; Σ-syntax; _,_; proj₁; proj₂)
open import Data.Empty using (⊥)

-- We assume the same postulates you provided:
postulate
  People : Set
  John : People
  Donkeys : Set
  Owns : People → Donkeys → Set
  Beats : People → Donkeys → Set
  Eyore : Donkeys
  je : Owns John Eyore

  JohnOwnsADonkey : Σ (Donkeys) (λ x → Owns John x)
  IfJohnOwnsADonkeyHeBeatsIt : (x : Donkeys) → Owns John x → Beats John x

  Men : Set
  Women : Set
  Loves : Men → Women → Set

  EveryManLovesAWoman : ∀ (x : Men) → Σ (Women) (λ y → Loves x y)

  Mary : Women
  John2 : Men
  LovesJohn2Mary : Loves John2 Mary

  Raven : Set
  Black : Raven → Set
  BlackRaven : Σ (Raven) (λ x → Black x)
  AllRavensAreBlack : ∀ (x : Raven) → Black x

  Walks : Men → Set
  SomeManWalks : Σ (Men) (λ x → Walks x)
  EveryManWalks : (x : Men) → Walks x

  Owns2 : Men → Donkeys → Set
  AManOwnsADonkey : Σ (Men) (λ x → Σ (Donkeys) (λ y → Owns2 x y))
  EveryManOwnsADonkey : ∀ (x : Men) → Σ (Donkeys) (λ y → Owns2 x y)

  Whistles : Men → Set
  AManWalksAndHeWhistles : (z : Σ (Men) (λ x → Walks x)) → Whistles (proj₁ z)

  Pedro : Men

  Bottles : Set
  Breaks : Men → Bottles → Set

  Bill : Men
  John3 : Men
  Sees : Men → (Σ (Men) (λ x → ∀ (b : Bottles) → Breaks x b)) → Set
  JohnBrokeEveryBottleAndBillSawIt :
    (z : Σ (Men) (λ x → ∀ (b : Bottles) → Breaks x b))
    → Sees Bill z



------------------------------------------------------------------------
-- Proofs

-- From JohnOwnsADonkey and IfJohnOwnsADonkeyHeBeatsIt we can prove 
-- that John beats the donkey he owns.

johnBeatsHisDonkey : Beats John (proj₁ JohnOwnsADonkey)
johnBeatsHisDonkey = IfJohnOwnsADonkeyHeBeatsIt (proj₁ JohnOwnsADonkey) (proj₂ JohnOwnsADonkey)

-- Explanation:
-- proj₁ JohnOwnsADonkey is the particular donkey John owns.
-- proj₂ JohnOwnsADonkey is the proof that John owns that donkey.
-- IfJohnOwnsADonkeyHeBeatsIt then shows John beats that donkey.

------------------------------------------------------------------------

-- Similarly, from BlackRaven and AllRavensAreBlack, we know that the 
-- particular raven we have (proj₁ BlackRaven) is black.

proofBlackRaven : Black (proj₁ BlackRaven)
proofBlackRaven = AllRavensAreBlack (proj₁ BlackRaven)

------------------------------------------------------------------------

-- We know that SomeManWalks is a witness that there exists a man who walks.
-- AManWalksAndHeWhistles says that if a man walks, he whistles.
-- Therefore, the man from SomeManWalks also whistles.

someManWhistles : Whistles (proj₁ SomeManWalks)
someManWhistles = AManWalksAndHeWhistles SomeManWalks

------------------------------------------------------------------------

-- From EveryManLovesAWoman applied to John2, we get that John2 loves some woman.
john2LovesSomeWoman : Σ (Women) (λ y → Loves John2 y)
john2LovesSomeWoman = EveryManLovesAWoman John2

-- We also have a direct postulate LovesJohn2Mary.
-- This shows that not only does John2 love some woman, specifically he loves Mary.
john2LovesMary : Loves John2 Mary
john2LovesMary = LovesJohn2Mary

\end{code}