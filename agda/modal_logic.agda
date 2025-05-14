infix 9 □_
3 postulate
4 □_ : Set → Set
5 □-fmap : { A : Set }{ B : Set } → ((A → B) → □ A → □ B)
6 □-extract : { A : Set } → (□ A → A)
7 □-duplicate : { A : Set } → (□ A → □ □ A)
8 □-cobind : { A : Set }{ B : Set } → (□ B → (□ B → A) → □ A)
9
10 postulate
11 Entity : Set
12 isMan : Entity → Set
13 runs : Entity → Set
14 isJohn : Entity → Set
15
16 record Man : Set where ... -- Shortened in this example, for brevity.
17
18 record John : Set where ... -- Shortened in this example, for brevity.
19
20 record RunsMan : Set where
21 constructor RunsMan
22 field
23 p : (a1 : Man) → runs (Man.e1 a1)
24
25 record ManJohn : Set where
26 constructor ManJohn
27 field
28 e1 : Entity
29 p1 : isJohn e1
30 p0 : isMan e1
31
32 record NecessarilyManJohn : Set where
33 constructor NecessarilyManJohn
34 field
35 I : □ ManJohn
36
37 record RunsJohn : Set where
38 constructor RunsJohn
39 field
40 e1 : John
41 p : runs (John.e1 e1)
42
43 record NecessarilyRunsJohn : Set where
44 constructor NecessarilyRunsJohn
45 field
46 I : □ RunsJohn
47
48 record KnowledgeBase : Set where
49 constructor KnowledgeBase
50 field
51 j1 : RunsMan
52 j2 : NecessarilyManJohn
53
54 thm : KnowledgeBase → NecessarilyRunsJohn
55 thm = x → NecessarilyRunsJohn ( □-fmap ( z →
56 RunsJohn (John (z .ManJohn.e1) (z .ManJohn.p1))
57 (x .KnowledgeBase.j1 .RunsMan.p
58 (Man (z .ManJohn.e1) (z .ManJohn.p0)))) (x .KnowledgeBase.j2 .NecessarilyManJohn.I)