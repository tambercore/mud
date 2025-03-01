module ModalLogicExample where

open import Level using (Level; _⊔_; lsuc; lzero)
open import Data.Nat using (ℕ)
open import Data.Sum using (_⊎_; inj₁; inj₂)
open import Data.Product using (_×_; Σ; _,_)
open import Data.Empty using (⊥)
open import Data.Unit using (⊤; tt)
open import Relation.Binary.PropositionalEquality as Eq using (_≡_; refl; trans)

------------------------------------------------------------------------
-- 1. Syntax of Modal Formulas
------------------------------------------------------------------------

-- We define a simple language of modal formulas.
data Formula : Set where
  atom    : ℕ → Formula         -- atomic propositions (indexed by ℕ)
  bot     : Formula             -- falsehood
  _∧_     : Formula → Formula → Formula
  _∨_     : Formula → Formula → Formula
  _→_     : Formula → Formula → Formula
  □       : Formula → Formula   -- necessity
  ◇       : Formula → Formula   -- possibility

infixl 4 _∧_
infixl 4 _∨_
infixr 3 _→_

------------------------------------------------------------------------
-- 2. Kripke Frames
------------------------------------------------------------------------

-- A Kripke frame consists of a type of worlds and an accessibility relation,
-- along with proofs that the relation is reflexive and transitive.
record Frame : Set₁ where
  field
    World : Set
    R     : World → World → Set
    refl  : ∀ {w : World} → R w w
    trans : ∀ {w v u : World} → R w v → R v u → R w u

open Frame public

------------------------------------------------------------------------
-- 3. Models
------------------------------------------------------------------------

-- A model is a frame along with a valuation that assigns to each atomic proposition
-- (here represented by a natural number) a predicate on worlds.
record Model : Set₁ where
  field
    frame : Frame
    V     : ℕ → Frame.World (frame) → Set

open Model public

------------------------------------------------------------------------
-- 4. Satisfaction Relation
------------------------------------------------------------------------

-- We define what it means for a model m to satisfy a formula A at a world w.
_⊨_ : Model → (Frame.World (frame m)) → Formula → Set
m ⊨ (atom n) w      = V m n w
m ⊨ bot      w      = ⊥
m ⊨ (A ∧ B)  w      = (m ⊨ A w) × (m ⊨ B w)
m ⊨ (A ∨ B)  w      = (m ⊨ A w) ⊎ (m ⊨ B w)
m ⊨ (A → B)  w      = (m ⊨ A w) → (m ⊨ B w)
m ⊨ (□ A)   w      = ∀ {w'} → R (frame m) w w' → m ⊨ A w'
m ⊨ (◇ A)   w      = Σ[ w' ∈ Frame.World (frame m) ] (R (frame m) w w' × m ⊨ A w')

------------------------------------------------------------------------
-- 5. The 4-Axiom: □P → □□P
------------------------------------------------------------------------

-- In many modal systems, transitivity of the accessibility relation
-- implies the axiom □P → □□P. Here is the corresponding Agda proof.
□-4 : ∀ {m : Model} {w : Frame.World (frame m)} {A : Formula} →
      m ⊨ (□ A) w → m ⊨ (□ (□ A)) w
□-4 mp {w'} r = λ {w''} r' → mp w'' (trans r r')

------------------------------------------------------------------------
-- 6. An Example Model and Satisfaction Check
------------------------------------------------------------------------

-- For a simple example, we let our worlds be Booleans and choose the accessibility
-- relation to be equality (which is reflexive and transitive).

open import Data.Bool using (Bool; true; false)

exampleFrame : Frame
exampleFrame = record
  { World = Bool
  ; R     = λ x y → x ≡ y
  ; refl  = λ {w} → refl
  ; trans = λ {w} {v} {u} r₁ r₂ → trans r₁ r₂
  }

-- Define a valuation:
-- Let the atomic proposition 0 be true only at world true,
-- and let all other atomic propositions be false.
exampleValuation : ℕ → Bool → Set
exampleValuation 0 true  = ⊤
exampleValuation 0 false = ⊥
exampleValuation _ _     = ⊥

-- Assemble the example model.
exampleModel : Model
exampleModel = record
  { frame = exampleFrame
  ; V     = exampleValuation
  }

-- Now, we check that in our example model the formula □ (atom 0) holds at world true.
-- Since our relation is equality, R true w' implies w' is true, and thus atom 0 is true.
exampleCheck : exampleModel ⊨ (□ (atom 0)) true
exampleCheck {w'} r = tt
