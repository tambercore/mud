{-# OPTIONS --cubical #-}

module synset where

open import Cubical.Core.Everything

-- Define the points.
data Quick : Set where
  center : Quick      -- The chosen center.
  fast   : Quick
  quick  : Quick
  rapid  : Quick

-- Postulate these paths to represent the synset.
postulate
  fast≡center  : fast  ≡ center
  quick≡center : quick ≡ center
  rapid≡center : rapid ≡ center

symQuick≡center : center ≡ quick
symQuick≡center = sym quick≡center
