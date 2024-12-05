open import Data.Product
open import Agda.Primitive
open import Data.Empty

{-
postulate
  People : Set
  John : People
  Donkeys : Set
  Eyore : Donkeys
  Owns : People → Donkeys → Set
  je : Owns John Eyore
  Beats : People → Donkeys → Set  
  Sad : Donkeys → Set

  donkey : (p : People) → (z : Σ Donkeys (λ x → Owns p x)) → Beats p (proj₁ z)
  sad : (d : Donkeys)
      → Σ People (λ p → Beats p d)
      → Sad d
  
thm : Sad Eyore
thm = sad Eyore (John , (donkey John (Eyore , je)))
-}


postulate
  People : Set
  John : People
  Donkeys : Set
  Owns : People → Donkeys → Set
  Beats : People → Donkeys → Set
  Eyore : Donkeys
  je : Owns John Eyore

  -- john owns a donkey
  JohnOwnsADonkey : Σ (Donkeys) (λ x → Owns John x)

  -- if john owns a donkey he beats it
  IfJohnOwnsADonkeyHeBeatsIt : (x : Donkeys) → Owns John x → Beats John x

  Men : Set
  Women : Set
  Loves : Men → Women → Set

 -- every man loves a woman
  EveryManLovesAWoman : ∀ (x : Men) → Σ (Women) (λ y → Loves x y)
  
 -- john loves mary
  Mary : Women
  John2 : Men
  LovesJohn2Mary : Loves John2 Mary

  -- john loves her

  -- john stops smoking

  -- x is a raven
  Raven : Set
  Black : Raven → Set

  -- x is a black raven
  BlackRaven : Σ (Raven) (λ x → Black x)

  -- all ravens are black
  AllRavensAreBlack : ∀ (x : Raven) → (Black x)

 -- some man walks
  Walks : Men → Set
  SomeManWalks : Σ (Men) (λ x → Walks x)

 -- every man walks
  EveryManWalks : ∀ (x : Men) → (Walks x)

 -- a man owns a donkey
  Owns2 : Men → Donkeys → Set
  AManOwnsADonkey : Σ (Men) λ x → Σ (Donkeys) (λ y → Owns2 x y)

 -- every man owns a donkey
  EveryManOwnsADonkey : ∀ (x : Men) → (Σ (Donkeys) (λ y → Owns2 x y))

 -- a man walks and he whistles
  Whistles : Men → Set
  AManWalksAndHeWhistles : (z : Σ (Men) (λ x → Walks x)) → Whistles (proj₁ z)

  -- if pedro owns every donkey he beats it (unsat)

  -- if pedro owns every donkey he likes it
  Pedro : Men

  -- john broke every bottle and bill saw it
  Bottles : Set
  Breaks : Men → Bottles → Set
  Sees : Men → (Σ (Men) (Breaks Men Bottles)) → Set -- what is the type of this
  Bill : Men
  John3 : Men
  
  JohnBrokeEveryBottleAndBillSawIt : (z : Σ (Men) (λ x → ∀ (x : Bottles) → (Breaks John3 x))) → (Sees Bill z)

  -- if you give every child a present some child will open it

  -- if you don't salute the colonel will see it