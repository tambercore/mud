module Contractible where

open import Agda.Primitive using (Level; lsuc; lzero)
open import Agda.Builtin.Equality

-- Type = Set
Type : (ℓ : Level) → Set (lsuc ℓ)
Type ℓ = Set ℓ

Type0 : Type (lsuc lzero)
Type0 = Type lzero

-- A type A is contractible if it has a center and a contraction function.
-- The contraction function shows every element x is equal to the center.
record isContr (A : Type lzero) : Type lzero where
  constructor mkContr
  field
    center      : A
    contraction : (x : A) → center ≡ x

-- Ireland is a type. (It is contractible.)
data Ireland : Type lzero where
  ireland : Ireland

-- We provide a proof that Ireland is contractible.
irelandContr : isContr Ireland
irelandContr = mkContr ireland (λ { ireland → refl })

-- Man is a type.
postulate Man : Type lzero

-- The binary relation "loves".
postulate loves : Man → Ireland → Type lzero

-- "every man loves Ireland" is encoded as a dependent type.
EveryManLovesIreland : Type lzero
EveryManLovesIreland = ∀ (m : Man) → loves m ireland