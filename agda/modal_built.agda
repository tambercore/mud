module modal_built where

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

postulate
  Entity : Set
  isSocrates : Entity → Set
  isMortal : Entity → Set

record Socratesᵣ : Set where
  constructor Socrates꜀
  field
    e₁ : Entity
    p₁ : isSocrates e₁


record MortalSocratesᵣ : Set where
  constructor MortalSocrates꜀
  field
    e₁ : Entity
    p₁ : isSocrates e₁
    p₀ : isMortal e₁


record NecessarilyMortalSocratesᵣ : Set where
  constructor NecessarilyMortalSocrates꜀
  field
    I : □ MortalSocratesᵣ


record KnowledgeBaseᵣ : Set where
  constructor KnowledgeBase꜀
  field
    j₁ : NecessarilyMortalSocratesᵣ


thm₁ : KnowledgeBaseᵣ → MortalSocratesᵣ
thm₁ = λ z → □-t (z .KnowledgeBaseᵣ.j₁ .NecessarilyMortalSocratesᵣ.I)

