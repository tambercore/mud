{-# OPTIONS --rewriting #-}

module modal_logic where

open import Data.Product
open import Relation.Binary.PropositionalEquality using (_≡_; refl; subst; sym; cong)

-- First, we define falsity (⊥) and negation (¬)
postulate
  ⊥ : Set
  
¬ : Set → Set
¬ A = A → ⊥

-- For example, if we aim for a modal logic that validates the T and K axioms, we can postulate:
postulate
  □ : Set → Set
  □-T : ∀ {A : Set} → □ A → A                       -- T (reflexivity): Necessity implies truth.
  □-K : ∀ {A B : Set} → □ (A → B) → (□ A → □ B)     -- K (distribution over implication):
  □-P : ∀ {A : Set} → □ A → □ (□ A)
  □-× : ∀ {A B : Set} → □ (A × B) → (□ A) × (□ B)

  ◇       : Set → Set
  ◇-intro : ∀ {A : Set} → A → ◇ A
  ◇-pos   : ∀ {A : Set} → ◇ A → □ (◇ A)
  ◇-dual₁ : ∀ {A : Set} → (◇ A → ¬ (□ (¬ A)))
  ◇-dual₂ : ∀ {A : Set} → (¬ (□ (¬ A)) → ◇ A)
  -- ◇-ness  : ∀ {A : Set} → A → □ A

variable A : Set

thm₁ : □ A → A
thm₁ = □-T
 
thm₂ : ◇ A → ¬ (□ (¬ A))
thm₂ = ◇-dual₁ 


postulate
  Entity : Set
  isMortal : Entity → Set
  isPerishable : Entity → Set
  Socrates : Entity
  MortalIsPerishable₃ : isMortal ≡ isPerishable
  sleeps : Entity → Set
  isJohn : Entity → Set

Predicate : Set₁
Predicate = Entity → Set

thm₃ : (p₁ : Predicate) → (p₂ : Predicate) → (e : Entity) → p₁ ≡ p₂ → □ (p₁ e) → □ (p₂ e)
thm₃ = λ p₁ p₂ e equiv w → subst (λ P → □ (P e)) equiv w

thm₄ : (e : Entity) → □ (isMortal e) → □ (isPerishable e)
thm₄ = λ e → thm₃ isMortal isPerishable e MortalIsPerishable₃

record JohnSleeps : Set where
    constructor mkJohnSleeps
    field
        e : Entity
        p1 : isJohn e
        p2 : sleeps e
  
record John : Set where
    constructor mkJohn
    field
        e : Entity
        p : isJohn e

postulate
  □proj₁ : (e : □ JohnSleeps) → (□ (isJohn (JohnSleeps.e (□-T e))))
  □proj₂ : (e : □ JohnSleeps) → (□ (sleeps (JohnSleeps.e (□-T e))))
  □const : (e : Entity) → (□ (sleeps e)) → (□ (isJohn e)) → □ JohnSleeps

  NeccJohn : (e : Entity) → □ (isJohn e) → □ John

thm₆ : □ JohnSleeps → □ John
thm₆ = λ z → NeccJohn (JohnSleeps.e (□-T z)) (□proj₁ z)

thm₇ : (e : Entity) → □ (isJohn e) → □ (sleeps e) → □ JohnSleeps
thm₇ = λ e z z₁ → □const e z₁ z

-- thm₆ = λ js → NeccJohn ( JohnSleeps.e (□-T js) ) ( (NeccJohnSleeps js) )

thm₅ : □ ( □ (isMortal Socrates)) → □ ( □ (isPerishable Socrates))
thm₅ = λ z → □-P (thm₄ Socrates (thm₁ z))
 