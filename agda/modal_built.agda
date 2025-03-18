module modal_built where

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
  isJesal : Entity → Set
  isGoblin : Entity → Set

record Jesalᵣ : Set where
  constructor Jesal꜀
  field
    e₁ : Entity
    p₁ : isJesal e₁


record GoblinJesalᵣ : Set where
  constructor GoblinJesal꜀
  field
    e₁ : Entity
    p₁ : isJesal e₁
    p₀ : isGoblin e₁


record PossiblyGoblinJesalᵣ : Set where
  constructor PossiblyGoblinJesal꜀
  field
    I : ◇ GoblinJesalᵣ


record KnowledgeBaseᵣ : Set where
  constructor KnowledgeBase꜀
  field
    j₁ : PossiblyGoblinJesalᵣ

{-
thm₁ : KnowledgeBaseᵣ → ◇ ◇ GoblinJesalᵣ
thm₁ = λ z → ◇-pure (z .KnowledgeBaseᵣ.j₁ .PossiblyGoblinJesalᵣ.I)
-}
