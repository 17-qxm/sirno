# Sirno

*Semantic Intermediate Representation of Nominal Objects*

Sirno is a documentation knowledge graph for concept-driven development and anti-drift codebase alignment.
It stores named concepts and their relations, refines broad design into local implementation,
binds those claims to repository artifacts, and requires re-examination when an upstream concept or relation changes.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/sirno-20260401.png" width="40%">
    <source media="(prefers-color-scheme: light)" srcset="assets/sirno-nb-20260401.png" width="40%">
    <img src="assets/sirno-nb-20260401.png" width="40%">
  </picture>
</p>

---

## What is Sirno?

Sirno is a set of named Markdown documents, each with a metadata block and a body of prose.
The document name is a nominal identifier, and therefore gives the document a stable canonical reference.

These documents are called *entries*.
An entry is sized to be read in about five minutes or less,
and its YAML metadata block records its full name, concise description, category, and relations.

Some entries are categorized as *concepts* or *narratives*,
and relational metadata includes *clustee*, *refiner*, and *witness*.

---

## The Thoughts, The Ambitions, The Principles We Would Follow

Sirno is cultivated from a series of elementary principles.

### Concept-Driven Development

You may have heard of spec-driven development, intent-driven development, or test-driven development.
These methodologies are effictive in their own ways, but they are still missing one crucial pieces of the puzzle.

*Compression*.

Compression saves bandwidth and therefore reduces communication overhead.
Compression is the key to scaling understanding across codebase evolution and across time.
In response, we propose concept-driven development.

*Concepts* are the named ideas that compress knowledge, everything from design intention to algorithmic details.
Cognition starts from naming things.
It's rather token-efficient to keep those names as the stable anchors for understanding and reference.

LZ77 uses an adaptive dictionary to replace repeated data with compact references.
Sirno gives project knowledge a similar dictionary:
each reference remains human-readable,
and a concept entry gathers the specifications, decisions, and tests that share the same name.

Reflecting on aforementioned paradigms, concepts serve three roles simultaneously:
- They cluster behavioral specifications under one named object.
- They keep intent portable across levels of detail.
- They organize tests so that properties and constraints become easier to see.

### Interactive Narrative

Browsing through a collection of documented concepts undoubtedly helps systematic understanding,
at least more systematic than chatting with a large language model or reading code alone.

Understanding is a process rather than a state.
A concept may unfold over time as the reader's mental model grows and refines.
But there's a gap: the reader's progress is not directly observable by the writer.

We all know that narrative shapes understanding and flattens learning curve.
But what we often miss is that narrative can be tailored to the reader's current understanding and needs,
which demands not only presentation but also interaction.

An *interactive narrative* presents an entry through dialogue,
asking positioning questions, observing responses,
and generating the next paragraph or quiz from the reader's current state.

The generated narrative is ephemeral;
canonical knowledge remains in entries and relations,
while the narrative provides a reading interface for onboarding and knowledge transfer.

### Clustee of a Clique

Tags, scopes, namespaces, and domains all approximate the same structure:
a named clique of related entries. Such named clique is an entry itself.

A *clustee* relation is a metadata block field of the clique member.
It groups entries by shared subject, local vocabulary, or design neighborhood,
and the clique name provides a short route into a region of the graph
without changing the entries' nominal identities.

### Refinement: From Specification to Implementation

Refinement unfolds a high-level idea into lower-level design, implementation, and tests.
The refined entry keeps the meaning of the concept intact,
while making its consequences local and concrete.

A refinement chain is a path of increasing specificity.
It starts from a compressed concept and ends near repository text,
preserving the reason that a local decision exists.

If the programming language itself is expressive and clean enough such that
the logic of the design is the clearest when expressed in code,
then the final step of refinement may be a markdown code block.

A *refiner* relation is a metadata block field of the refined entry.
It points to the entry that it refines.

### Mirror Design with Witness

A *witness* relation is a metadata block field of the witnessed entry.
It points to code in repository that evidences the entry's claim.

When an entry describes behavior, representation, or invariant,
the witness is the concrete program text against which that claim can be checked.
A test may witness an entry when the test itself is the relevant code.
