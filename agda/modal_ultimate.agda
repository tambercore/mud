
module modal_ultimate where 

open import Data.Product
open import Relation.Binary.PropositionalEquality using (_≡_; refl; subst; sym; cong)

infix 9 □_
infix 10 ◇_

postulate
    -- rule in S4 Modal Logic
    -- Necessity is some operation over 2 types
    □_   : Set → Set
    ◇_   : Set → Set

    -- ◇ as a monad
    ◇-fmap : ∀ {A B : Set}   → (A → B) → ◇ A → ◇ B
    ◇-pure : ∀ {A : Set}     → A → ◇ A 
    ◇-lift : ∀ {A B : Set}   → ◇ (A → B) → ◇ A → ◇ B
    ◇-bind : ∀ {A B : Set}   → (◇ A) → (A → ◇ B) → ◇ B

    -- □ as a comonad
    □-fmap : ∀ {A B : Set} → (A → B) → □ A → □ B
    □-extract : ∀ {A : Set} → □ A → A
    □-duplicate : ∀ {A : Set} → □ A → □ □ A 
    □-cobind : ∀ {A B : Set} → □ B → (□ B → A) → □ A

-- Derive S4 Modal Logic (as follows)
□-k : ∀ {A B : Set} → □ (A → B) → (□ A → □ B)
□-k = λ z z₁ → □-fmap (λ z₂ → z₂ (□-extract z₁)) z

□-t : ∀ {A : Set} → □ A → A
□-t = □-extract

□-4 : ∀ {A : Set} → □ A → □ □ A 
□-4 = □-duplicate

-- □-d says that if □ A then it is possible that A
□-d : ∀ {A : Set} → □ A → ◇ A
□-d = λ z → ◇-pure (□-extract z)

{-
Rejection of S5 Modal Logic (in this system.)
◇-5 : ∀ {A : Set} → ◇ A → □ ◇ A

-- □-b says that if Prop A then it is necessairly possible that Prop A
-- □-b : ∀ {A : Set} → A → □ ◇ A
-- □-b = λ z → ◇-5 (◇-pure z)

-- Possible to derive Gödels Necessity Combinator
-- thm₃ : {A : Set} → A → □ A
-- thm₃ = λ z → □-cobind (λ z₁ → z) (◇-5 (◇-pure z))
-}

{- Examples -}

postulate
    Ind : Set
    IsProperty₁ : Ind → Set
    IsProperty₂ : Ind → Set
    Person₁ : Ind

record Person₁Both : Set where
    no-eta-equality
    constructor mkPerson₁Both
    field
        i : Ind
        p1 : IsProperty₁ i
        p2 : IsProperty₂ i

record Person₁One : Set where
    constructor mkPerson₁One
    field
        i : Ind
        p1 : IsProperty₁ i
 
record Person₁Two : Set where
    constructor mkPerson₁Two
    field
        i : Ind
        p1 : IsProperty₂ i



{- We can construct this necessity. -}
thm' : □ Person₁Both → □ Person₁One
thm' = □-fmap (λ z → mkPerson₁One (z .Person₁Both.i) (z .Person₁Both.p1))

thm''' : □ Person₁Both → □ Person₁Two
thm''' = □-fmap (λ z → mkPerson₁Two (z .Person₁Both.i) (z .Person₁Both.p2))

thm₀ : □ Person₁Both → ( □ Person₁One × □ Person₁Two )
thm₀ = λ z → thm' z , thm''' z



{- Theorem 9 Provable -}
thm₉' : (p : □ Person₁Both) → (□ (IsProperty₁ (Person₁Both.i (□-t p))))
thm₉' p = □-cobind p (λ _ → Person₁Both.p1 (□-extract p))

thm₉'' : (p : □ Person₁Both) → (□ (IsProperty₂ (Person₁Both.i (□-t p))))
thm₉'' p = □-cobind p (λ _ → Person₁Both.p2 (□-extract p))

thm₉ : (p : □ Person₁Both) → (□ (IsProperty₁ (Person₁Both.i (□-t p))) × □ (IsProperty₂ (Person₁Both.i (□-t p))))
thm₉ = λ p → thm₉' p , thm₉'' p



{- Theorem 3 Provable -}
thm₃' :  □ Person₁Both → (Person₁One)
thm₃' p = □-t (□-fmap (λ z → mkPerson₁One (z .Person₁Both.i) (z .Person₁Both.p1)) p)

thm₃'' : □ Person₁Both → (Person₁Two)
thm₃'' p = □-t (□-fmap (λ z → mkPerson₁Two (z .Person₁Both.i) (z .Person₁Both.p2)) p)

thm₃ : □ Person₁Both → (Person₁One × Person₁Two)
thm₃ p = thm₃' p , thm₃'' p



{- Theorem 8 Provable -}
thm₈ : (i : Ind) → (□ IsProperty₁ i) → (□ IsProperty₂ i) → □ Person₁Both
thm₈ = λ i z → □-fmap (λ z₁ → mkPerson₁Both i (□-t z) z₁)
 

{-
-- This doesn't work, and it should not work.
thm₂ : Person₁Both → (□ Person₁One × □ Person₁Two)
thm₂ = ?
-}

{-


thm₄ : ◇ Person₁One → ◇ Person₁Two → ◇ Person₁Both
thm₄ = {! -m  !}

thm₅ : (i : Ind) → IsProperty₁ i → IsProperty₂ i → Person₁Both
thm₅ = λ i z z₁ → mkPerson₁Both i z z₁

thm₆ : (i : Ind) → IsProperty₁ i → IsProperty₂ i → ◇ Person₁Both
thm₆ = λ i z z₁ → ◇-pure (mkPerson₁Both i z z₁)

thm₇ : (i : Ind) → IsProperty₁ i → IsProperty₂ i → □ Person₁Both
thm₇ = {!  -m !}



thm₁₀ : (p : Person₁Both) → (□ (IsProperty₁ (Person₁Both.i p)) × □ (IsProperty₂ (Person₁Both.i p)))
thm₁₀ = {! -m  !}

-- Works for ◇
thm₁₁ : (p : Person₁Both) → (◇ (IsProperty₁ (Person₁Both.i p)) × ◇ (IsProperty₂ (Person₁Both.i p)))
thm₁₁ = λ p → ◇-pure (p .Person₁Both.p1) , ◇-pure (p .Person₁Both.p2)

-- Default Case (nonmodal)
thm₁₂ : (p : Person₁Both) → ((IsProperty₁ (Person₁Both.i p)) × (IsProperty₂ (Person₁Both.i p)))
thm₁₂ = λ p → p .Person₁Both.p1 , p .Person₁Both.p2 
-}    