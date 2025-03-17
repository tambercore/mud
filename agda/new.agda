module new where

open import Data.Product

open import Relation.Binary.PropositionalEquality using (_≡_; refl; subst; sym; cong)

postulate
  Entity : Set
  isMan : Entity → Set
  isFast : Entity → Set
  isJohn : Entity → Set
  isQuick : Entity → Set
  fast_syn_quick : isFast ≡ isQuick

record Manᵣ : Set where
  constructor Man꜀
  field
    e₁ : Entity
    p₁ : isMan e₁


fast_syn_quick_pointwise : (e : Entity) → isFast e → isQuick e
fast_syn_quick_pointwise = λ (e) → λ (m) → subst (λ (X) → X e) fast_syn_quick m


record IsManFastᵣ : Set where
  constructor IsManFast꜀
  field
    p : (a₁ : Manᵣ) → isFast (Manᵣ.e₁ a₁)


record Johnᵣ : Set where
  constructor John꜀
  field
    e₁ : Entity
    p₁ : isJohn e₁


record ManJohnᵣ : Set where
  constructor ManJohn꜀
  field
    e₁ : Entity
    p₁ : isJohn e₁
    p₀ : isMan e₁


record KnowledgeBaseᵣ : Set where
  constructor KnowledgeBase꜀
  field
    j₁ : IsManFastᵣ
    j₂ : ManJohnᵣ


record QuickJohnᵣ : Set where
  constructor QuickJohn꜀
  field
    e₁ : Entity
    p₁ : isJohn e₁
    p₀ : isQuick e₁


thm₁ : KnowledgeBaseᵣ → QuickJohnᵣ
thm₁ = λ z →
  QuickJohn꜀ (z .KnowledgeBaseᵣ.j₂ .ManJohnᵣ.e₁)
  (z .KnowledgeBaseᵣ.j₂ .ManJohnᵣ.p₁)
  (fast_syn_quick_pointwise (z .KnowledgeBaseᵣ.j₂ .ManJohnᵣ.e₁)
   (z .KnowledgeBaseᵣ.j₁ .IsManFastᵣ.p
    (Man꜀ (z .KnowledgeBaseᵣ.j₂ .ManJohnᵣ.e₁)
     (z .KnowledgeBaseᵣ.j₂ .ManJohnᵣ.p₀))))

