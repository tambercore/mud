\begin{code}


\end{code} 

 \section{Premises (Assumptions)}

\begin{itemize}\item A0: every man is every woman
\end{itemize} 

 \begin{code}

module output_file where

open import Data.Product

open import Relation.Binary.PropositionalEquality using (_≡_; refl; subst; sym; cong)

infix 9 □_ 
infix 10 ◇_ 

postulate
    -- rule in S4 Modal Logic
    □_ : Set → Set
    ◇_ : Set → Set
    -- ◇ as a monad
    ◇-fmap : ∀ { A : Set }{ B : Set } → (((A → B) → ◇ A) → ◇ B)
    ◇-pure : ∀ { A : Set } → (A → ◇ A)
    ◇-lift : ∀ { A : Set }{ B : Set } → (◇ (A → B) → ◇ A → ◇ B)
    ◇-bind : ∀ { A : Set }{ B : Set } → ((◇ A → A → ◇ B) → ◇ B)
    -- □ as a comonad
    □-fmap : ∀ { A : Set }{ B : Set } → ((A → B) → □ A → □ B)
    □-extract : ∀ { A : Set } → (□ A → A)
    □-duplicate : ∀ { A : Set } → (□ A → □ □ A)
    □-cobind : ∀ { A : Set }{ B : Set } → (□ B → (□ B → A) → □ A)


-- Now, introduce the relevant language constructions
postulate
    Entity : Set
    isMan : Entity → Set
    isEvery : Entity → Set
    isWoman : Entity → Set

□-d : ∀ { A : Set } → (□ A → ◇ A)
□-d = λ z → ◇-pure (□-extract z)


□-4 : ∀ { A : Set } → (□ A → □ □ A)
□-4 = □-duplicate


□-t : ∀ { A : Set } → (□ A → A)
□-t = □-extract


□-k : ∀ { A : Set }{ B : Set } → (□ (A → B) → □ A → □ B)
□-k = λ z → λ z₁ → □-fmap (λ z₂ → z₂ (□-extract z₁)) z


-- Record declaration for 'man'
record Manᵣ : Set where
  constructor Man꜀
  field
    e₁ : Entity
    p₁ : isMan e₁


-- Record declaration for 'every man is every woman'
record IsManEveryWomanᵣ : Set where
  constructor IsManEveryWoman꜀
  field
    p : (a₁ : Manᵣ) → isWoman (Manᵣ.e₁ a₁) × isEvery (Manᵣ.e₁ a₁)


record KnowledgeBaseᵣ : Set where
  constructor KnowledgeBase꜀
  field
    j₁ : IsManEveryWomanᵣ

\end{code}