hello

\begin{code}

open import Cubical.Core.Everything   
open import Cubical.Foundations.Prelude 
open import Cubical.Foundations.Isomorphism


module yellowCheeseProof where


postulate
  Entity : Set
  isSocrates : Entity → Set
  isMan : Entity → Set


record Socrates : Set where
  constructor socrates
  field
    e₁ : Entity
    p1  : isSocrates e₁

record Man : Set where
  constructor man
  field
    e₁ : Entity
    p1  : isMan e₁

postulate
  socratesIsMan : Socrates ≡ Man

p1 : Socrates → Man
p1 s = transport (λ i → {!   !}) socratesIsMan s

\end{code}


