\begin{code}


\end{code} 

 \section{Premises (Assumptions)}

\begin{itemize}\item A0: john must eat
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
    isJohn : Entity → Set
    eat : Entity → Set

□-d : ∀ { A : Set } → (□ A → ◇ A)
□-d = λ z → ◇-pure (□-extract z)


□-4 : ∀ { A : Set } → (□ A → □ □ A)
□-4 = □-duplicate


□-t : ∀ { A : Set } → (□ A → A)
□-t = □-extract


□-k : ∀ { A : Set }{ B : Set } → (□ (A → B) → □ A → □ B)
□-k = λ z → λ z₁ → □-fmap (λ z₂ → z₂ (□-extract z₁)) z


-- Record declaration for 'john'
record Johnᵣ : Set where
  constructor John꜀
  field
    e₁ : Entity
    p₁ : isJohn e₁


-- Record declaration for 'eat john'
record EatJohnᵣ : Set where
  constructor EatJohn꜀
  field
    e₁ : Johnᵣ
    p : eat (Johnᵣ.e₁ e₁)


-- Record declaration for 'must eat john'
record MustEatJohnᵣ : Set where
  constructor MustEatJohn꜀
  field
    I : □ EatJohnᵣ


-- Record declaration for 'necessarily must eat john'
record NecessarilyMustEatJohnᵣ : Set where
  constructor NecessarilyMustEatJohn꜀
  field
    I : □ MustEatJohnᵣ


record KnowledgeBaseᵣ : Set where
  constructor KnowledgeBase꜀
  field
    j₁ : MustEatJohnᵣ


\end{code} 

 \section{Theorems}
\subsection{Theorem 1: `necessarily must eat john'}

...
 

 \begin{code}

thm₁ : KnowledgeBaseᵣ → NecessarilyMustEatJohnᵣ
thm₁ = λ z →
  NecessarilyMustEatJohn꜀
  (□-cobind (z .KnowledgeBaseᵣ.j₁ .MustEatJohnᵣ.I)
   (λ z₁ → z .KnowledgeBaseᵣ.j₁))

\end{code}