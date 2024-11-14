

This is a literate part here

\begin{code}

module Siblings where

open import Relation.Nullary
open import Data.Product
open import Data.Empty
open import Relation.Binary.PropositionalEquality

-- iff
infix 1 ⇔
record _⇔_ (A B : Set) : Set where
  field
    to   : A → B
    from : B → A

-- people type
People : Set

-- relationship
Parent : People → People → Set
Sibling : People → People → Set

-- axioms
SiblingAntiRefl : ∀ x → Sibling x x → ⊥
siblingSymmetric : ∀ {x y} → Sibling x y → Sibling y x

-- sibling transitivity
siblingTrans : ∀ {x y z} → ¬ (x ≡ z) → Sibling x y → Sibling y z → Sibling x z

-- common parent
CommonParent : People → People → Set
CommonParent x y = Σ[ z ∈ People ] Parent z x × Parent z y  

-- All common parent
AllCommonParent : People → People → Set
AllCommonParent x y = (z : People) → (Parent z x → Parent z y)


-- half sibling
HalfSibling : People → People → Set
HalfSibling x y = CommonParent x y × ¬ (x ≡ y)

-- full sibling
FullSibling : People → People → Set
FullSibling x y = AllCommonParent x y × ¬ (x ≡ y)

\end{code}   