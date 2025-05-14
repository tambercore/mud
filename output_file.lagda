\begin{code}


\end{code} 

 \section{Premises (Assumptions)}

\begin{itemize}\item A0: john is quick
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
    isQuick : Entity → Set
    isFast : Entity → Set
    quick_syn_fast : isQuick ≡ isFast
    fast_syn_quick : isFast ≡ isQuick

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


quick_syn_fast_pointwise : (e : Entity) → isQuick e → isFast e
quick_syn_fast_pointwise = λ e → λ m → subst (λ X → X e) quick_syn_fast m


-- Record declaration for 'John is quick'
record QuickJohnᵣ : Set where
  constructor QuickJohn꜀
  field
    e₁ : Entity
    p₁ : isJohn e₁
    p₀ : isQuick e₁


fast_syn_quick_pointwise : (e : Entity) → isFast e → isQuick e
fast_syn_quick_pointwise = λ e → λ m → subst (λ X → X e) fast_syn_quick m


-- Record declaration for 'John is fast'
record FastJohnᵣ : Set where
  constructor FastJohn꜀
  field
    e₁ : Entity
    p₁ : isJohn e₁
    p₀ : isFast e₁


record KnowledgeBaseᵣ : Set where
  constructor KnowledgeBase꜀
  field
    j₁ : QuickJohnᵣ


\end{code} 

 \section{Theorems}
\subsection{Theorem 1: `john is fast'}

To provide evidence of 'john is fast', there must exist an entity, and evidence that the entity is John, and evidence that the entity is Fast
\begin{enumerate}
  \item Given that john is quick (A0), there exists an entity
  \item Given that john is quick (A0), there exists evidence that the entity is John
  \item quick is synonymous with fast
  \begin{enumerate}
    \item Given that john is quick (A0), there exists an entity
    \item Given that john is quick (A0), there exists evidence that the entity is Quick
  \end{enumerate}
  \item Therefore, john is fast
\end{enumerate}
 

 \begin{code}

thm₁ : KnowledgeBaseᵣ → FastJohnᵣ
thm₁ = λ z →
  FastJohn꜀ (z .KnowledgeBaseᵣ.j₁ .QuickJohnᵣ.e₁)
  (z .KnowledgeBaseᵣ.j₁ .QuickJohnᵣ.p₁)
  (quick_syn_fast_pointwise (z .KnowledgeBaseᵣ.j₁ .QuickJohnᵣ.e₁)
   (z .KnowledgeBaseᵣ.j₁ .QuickJohnᵣ.p₀))

\end{code}