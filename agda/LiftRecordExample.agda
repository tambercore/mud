{-# OPTIONS --rewriting #-}

module LiftRecordExample where

open import Agda.Builtin.Reflection using {String}
open import Agda.Builtin.Unit
open import Agda.Primitive

-- Postulated modal operators and helpers.
postulate
  □     : Set → Set
  □-T   : ∀ {A : Set} → □ A → A
  □-K   : ∀ {A B : Set} → □ (A → B) → □ A → □ B

-- A helper to “lift” a term into the □‑world.
postulate
  rigid : ∀ {A : Set} → A → □ A

-- A simple record for demonstration.
record Person : Set where
  constructor mkPerson
  field
    name : String
    age  : Nat

-- Its “lifted” version: each field is wrapped in □.
record NecessaryPerson : Set where
  constructor mkNecessaryPerson
  field
    name : □ String
    age  : □ Nat

{-!
  The macro ‘liftRecord’ is intended to generate a lambda term of type

    □ Person → □ NecessaryPerson

  by “lifting” the fields. Concretely, for an input record term (of type □ Person)
  it generates a term equivalent to

    λ rec → rigid (mkNecessaryPerson (rigid (Person.name (□-T rec)))
                                      (rigid (Person.age (□-T rec))))

  Note: This implementation uses the reflection API to “assemble” the term.
!-}
macro
  liftRecord : Term → TC Term
  liftRecord _ = do
    -- We generate a lambda abstraction that binds a variable “rec”
    recName ← freshName "rec"
    let recVar = var 0 []  -- de Bruijn index 0 will correspond to “rec”

    -- Look up the names of the functions/constructors we need.
    -- (In practice, you might use 'quoteQName' or similar functions.
    -- Here we use 'quoteName' as a stand-in.)
    tBoxTName         ← pure (quoteName "□-T")
    tRigidName        ← pure (quoteName "rigid")
    tMkNecPersonName  ← pure (quoteName "mkNecessaryPerson")
    tPersonNameName   ← pure (quoteName "Person.name")
    tPersonAgeName    ← pure (quoteName "Person.age")

    -- Build the following subterms:
    --
    -- p = □-T rec
    let boxT = def tBoxTName []
    let pTerm = apply boxT [arg recVar]

    -- lifted name = rigid (Person.name p)
    let personName = def tPersonNameName []
    let nameField = apply personName [arg pTerm]
    let rigidTerm = def tRigidName []
    let liftedName = apply rigidTerm [arg nameField]

    -- lifted age = rigid (Person.age p)
    let personAge = def tPersonAgeName []
    let ageField = apply personAge [arg pTerm]
    let liftedAge = apply rigidTerm [arg ageField]

    -- recordResult = mkNecessaryPerson liftedName liftedAge
    let mkNecPerson = def tMkNecPersonName []
    let recordResult = apply mkNecPerson [arg liftedName , arg liftedAge]

    -- final result: wrap the record with rigid to get a term of type □ NecessaryPerson
    let finalResult = apply rigidTerm [arg recordResult]

    -- Build the lambda abstraction: λ rec → finalResult.
    let lamTerm = lam recName (abs recName finalResult)
    pure lamTerm

-- Usage:
-- We can now define myLiftedPerson by splicing the macro.
myLiftedPerson : □ Person → □ NecessaryPerson
myLiftedPerson = liftRecord _
 