
module new where

{-

postulate: 
{"x1": "amber", "x0": "fruit", , "x6": "amber", 'Entity' : 'Set'}

reduced expression: 
Σ(a: Amber) ( Π(x5 : Cake) (likes(a, x5)) x Σ(x0 : Fruit) (likes(x1, a)) )

expanded expression: Π(x5:) (likes(x6, x5)) ^ Σ(x0) (likes(x1, x0))

knwoledge toby will need to get this to work

know that fruit is a  set
know that amber is a  set
know that cake is a  set

--
the context in a dependent sum or product is just the bound variable -- i said this amber

in the bound expression of a given Pi Type, the local context extends the heirarchial parent with the bound variable and it's type.

John likes cake
john : Entity , entity : set, likes: 


Σ(a: Amber) → ( Π(x5 : Cake) (likes(a, x5)) x Σ(x0 : Fruit) (likes(x1, a)) )
                                    ^   Gamma here


element of (x1 : amber)
x1 is a element of amber

THIS IS A TYPE
Σ(x1: Amber) → ( Π(x5 : Cake) (likes(a, x5)) x Σ(x0 : Fruit) (likes(x1, a)) )


AN ELEMENT OF THIS IS A FUNCTION TAKING PROOF OF AMBER TO PROOF THAT THAT AMBER LIKES CAKE AND PROOF THAT THAT AMBER LIKES A FRUIT

|   =   ( <x5, cake> <a, Amber>)
 
-}

postulate
    "x1": "amber"
    "x0": "fruit"
    "x5": "cake
    "x6": "amber"
    'Entity' : 'Set'

  isFruit : Entity → Set


record Fruit : Set where
    constructor fruit
    field
        e₁ : Entity
        proofFruit : isFruit e₁

record Amber : Set where
    constructor fruit
    field
        e₁ : Entity
        proofFruit : isFruit e₁

record Fruit : Set where
    constructor fruit
    field
        e₁ : Entity
        proofFruit : isFruit e₁

-- Π(x5) (likes(x6, x5)) ^ Σ(x0) (likes(x1, x0))