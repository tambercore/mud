module mortal where

open import Data.Product
open import Relation.Binary.PropositionalEquality using (_≡_; refl; subst; sym; cong)

postulate
  Entity : Set
  isMan : Entity → Set
  isMortal : Entity → Set
  isPerishable : Entity → Set
  isSocrates : Entity → Set
  eqMortalPerishable :  isMortal ≡ isPerishable

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


record KnowledgeBaseᵣ : Set where
  constructor KnowledgeBase꜀
  field
    j₁ : IsManMortalᵣ
    j₂ : ManSocratesᵣ


record MortalSocratesᵣ : Set where
  constructor MortalSocrates꜀
  field
    e₁ : Entity
    p₁ : isSocrates e₁
    p₀ : isMortal e₁

record PerishableSocrates : Set where
  constructor mkPerishableSocrates
  field
    e₁ : Entity
    p₁ : isSocrates e₁
    p₀ : isPerishable e₁

thm₁ : KnowledgeBaseᵣ → MortalSocratesᵣ
thm₁ = λ z →
  MortalSocrates꜀ (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.e₁)
  (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.p₁)
  (z .KnowledgeBaseᵣ.j₁ .IsManMortalᵣ.p
   (Man꜀ (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.e₁)
    (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.p₀)))

{-
    The issue here is that Agsy doesn't recognise `subst`
-}
thm₂ : KnowledgeBaseᵣ → PerishableSocrates
thm₂ = λ z →
    mkPerishableSocrates (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.e₁)
    (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.p₁)
    (eqMortalPerishable (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.e₁)
     (z .KnowledgeBaseᵣ.j₁ .IsManMortalᵣ.p
      (Man꜀ (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.e₁)
       (z .KnowledgeBaseᵣ.j₂ .ManSocratesᵣ.p₀))))