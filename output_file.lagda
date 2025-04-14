\begin{code}


\end{code} 

 \section{Premises (Assumptions)}

\begin{itemize}\item A0: socrates is a man
\item A1: every man is mortal
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
    isSocrates : Entity → Set
    isMan : Entity → Set
    isMortal : Entity → Set

□-d : ∀ { A : Set } → (□ A → ◇ A)
□-d = λ z → ◇-pure (□-extract z)


□-4 : ∀ { A : Set } → (□ A → □ □ A)
□-4 = □-duplicate


□-t : ∀ { A : Set } → (□ A → A)
□-t = □-extract


□-k : ∀ { A : Set }{ B : Set } → (□ (A → B) → □ A → □ B)
□-k = λ z → λ z₁ → □-fmap (λ z₂ → z₂ (□-extract z₁)) z


-- Record declaration for 'socrates'
record Socratesᵣ : Set where
  constructor Socrates꜀
  field
    e₁ : Entity
    p₁ : isSocrates e₁


-- Record declaration for 'Socrates is man'
record ManSocratesᵣ : Set where
  constructor ManSocrates꜀
  field
    e₁ : Entity
    p₁ : isSocrates e₁
    p₀ : isMan e₁


-- Record declaration for 'man'
record Manᵣ : Set where
  constructor Man꜀
  field
    e₁ : Entity
    p₁ : isMan e₁


-- Record declaration for 'every man is mortal'
record IsManMortalᵣ : Set where
  constructor IsManMortal꜀
  field
    p : (a₁ : Manᵣ) → isMortal (Manᵣ.e₁ a₁)


-- Record declaration for 'Socrates is mortal'
record MortalSocratesᵣ : Set where
  constructor MortalSocrates꜀
  field
    e₁ : Entity
    p₁ : isSocrates e₁
    p₀ : isMortal e₁


record KnowledgeBaseᵣ : Set where
  constructor KnowledgeBase꜀
  field
    j₁ : ManSocratesᵣ
    j₂ : IsManMortalᵣ


\end{code} 

 \section{Theorems}
\subsection{Theorem 1: `socrates is mortal'}

To provide evidence of 'socrates is mortal', there must exist an entity, and evidence that the entity is Socrates, and evidence that the entity is Mortal
\begin{enumerate}
  \item Given that socrates is man (A0), there exists an entity
  \item Given that socrates is man (A0), there exists evidence that the entity is Socrates
  \item Given that every man is mortal (A1), it is known that for every Man, that the Man is mortal
  \begin{enumerate}
    \item To provide evidence of 'man', there must exist an entity, and evidence that the entity is man
    \begin{enumerate}
      \item Given that socrates is man (A0), there exists an entity
      \item Given that socrates is man (A0), there exists evidence that the entity is Man
    \end{enumerate}
  \end{enumerate}
  \item Therefore, socrates is mortal
\end{enumerate}
 

 \begin{code}

thm₁ : KnowledgeBaseᵣ → MortalSocratesᵣ
thm₁ = λ z →
  MortalSocrates꜀ (z .KnowledgeBaseᵣ.j₁ .ManSocratesᵣ.e₁)
  (z .KnowledgeBaseᵣ.j₁ .ManSocratesᵣ.p₁)
  (z .KnowledgeBaseᵣ.j₂ .IsManMortalᵣ.p
   (Man꜀ (z .KnowledgeBaseᵣ.j₁ .ManSocratesᵣ.e₁)
    (z .KnowledgeBaseᵣ.j₁ .ManSocratesᵣ.p₀)))

\end{code}