module comparen where

open import Data.Product

open import Relation.Binary.PropositionalEquality using (_≡_; refl; subst; sym; cong)

infix 9 □_
infix 10 ◇_

postulate
    -- rule in S4 Modal Logic
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

-- Now, introduce the relevant language constructions
postulate
  Entity : Set
  isMan : Entity → Set
  isMortal : Entity → Set
  isSocrates : Entity → Set

record Manᵣ : Set where
  constructor Man꜀
  field
    e₁ : Entity
    p₁ : isMan e₁


record IsManMortalᵣ : Set where
  constructor IsManMortal꜀
  field
    p : (a₁ : Manᵣ) → isMortal (Manᵣ.e₁ a₁)


record Socratesᵣ : Set where
  constructor Socrates꜀
  field
    e₁ : Entity
    p₁ : isSocrates e₁


record ManSocratesᵣ : Set where
  constructor ManSocrates꜀
  field
    e₁ : Entity
    p₁ : isSocrates e₁
    p₀ : isMan e₁


record MortalSocratesᵣ : Set where
  constructor MortalSocrates꜀
  field
    e₁ : Entity
    p₁ : isSocrates e₁
    p₀ : isMortal e₁


record KnowledgeBaseᵣ : Set where
  constructor KnowledgeBase꜀
  field
    j₁ : IsManMortalᵣ
    j₂ : ManSocratesᵣ


thm₁ : KnowledgeBaseᵣ → MortalSocratesᵣ
thm₁ = λ z →
  MortalSocrates꜀ (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.e₁)
  (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.p₁)
  (z .KnowledgeBaseᵣ.j₁ .IsManMortalᵣ.p
   (Man꜀ (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.e₁)
    (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.p₀)))

