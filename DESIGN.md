# Sirno

*Semantic Intermediate Representation of Nominal Objects*

Sirno is a bidirectional compiler for design-aware programming work.
It moves between one long-form project narrative, a store of compact named Markdown entries,
and the repository codebase.

It gives design a nominal intermediate form. The names are readable by humans, stable for tools,
and small enough for agents to inspect without carrying the whole project in context.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/sirno-20260401.png" width="40%">
    <source media="(prefers-color-scheme: light)" srcset="assets/sirno-nb-20260401.png" width="40%">
    <img src="assets/sirno-nb-20260401.png" width="40%">
  </picture>
</p>

---

## What Is Sirno?

Sirno works through three surfaces.

`mono` is one configured Markdown document, often `DESIGN.md`.
It is the project monograph:
a readable narrative for a person who wants the whole design in one sitting.

`sirno` is one configured entry store, often `docs/`.
It contains named Markdown documents with metadata blocks and bodies of prose.

`code` is the repository implementation surface.
It contains source files, tests, configuration, generated files, assets,
and other artifacts that realize design decisions.

The `sirno` store is the human-readable intermediate representation.
It is text first.
It is structured enough for tools.
It is compact enough for humans and agents to inspect locally.

Before the store exists, the user chooses which surface currently carries more authority:
the working codebase, or the monograph that best describes the intended project.
After the store exists, Sirno treats the store as the structured intermediate form.

Sirno maintains structure.
It does not decide whether a design is good.
It does not prove that code satisfies a claim.
It gives people, agents, skills, and tools stable nominal objects
through which design and implementation can be revised.

---

## Entries

Sirno is a set of named Markdown documents,
each with a metadata block and a body of prose.
The document name is a nominal identifier,
and therefore gives the document a stable canonical reference.

These documents are called *entries*.
An entry is sized to be read in about five minutes or less,
and its YAML metadata block records its full name,
concise description,
category,
and relations.

The filename stem is the entry id.
The id is globally unique within the store and case-sensitive.
The id is used by relation fields,
generated footers,
and witness lookup.

In principle,
ids can follow the filesystem.
In practice,
entries are expected to use filename stems such as
`concept-driven-development`.
Lowercase ASCII kebab-case is the naming convention.

An entry is smaller and tighter than the monograph.
It states a concept,
category,
clique closure,
refinement,
invariant,
interface,
implementation commitment,
witnessable claim,
or narrative route with local prose.

The prose body carries the design content.
The metadata block carries structure that tools must read exactly.

---

## Concept-Driven Development

Sirno is cultivated from a series of elementary principles.

You may have heard of spec-driven development,
intent-driven development,
or test-driven development.
These methodologies are effective in their own ways,
but they still miss one crucial piece of the puzzle.

*Compression*.

Compression saves bandwidth and therefore reduces communication overhead.
Compression is the key to scaling understanding across codebase evolution and
across time.
In response,
Sirno proposes concept-driven development.

*Concepts* are the named ideas that compress knowledge,
everything from design intention to algorithmic details.
Cognition starts from naming things.
It is token-efficient to keep those names as stable anchors for understanding
and reference.

LZ77 uses an adaptive dictionary to replace repeated data with compact
references.
Sirno gives project knowledge a similar dictionary:
each reference remains human-readable,
and a concept entry gathers the specifications,
decisions,
implementation notes,
and tests that share the same name.

Concepts serve three roles simultaneously:
- They cluster behavioral specifications under one named object.
- They keep intent portable across levels of detail.
- They organize tests so that properties and constraints become easier to see.

The initialized `concept` entry is ordinary.
It is created by `init`.
It is not privileged by the system.

---

## Narrative

A narrative records a cognitive route through concepts.

The monograph is the primary narrative surface.
It is outside the store and remains normal Markdown.
It is configured by path,
with `DESIGN.md` as the usual convention.

Browsing through a collection of documented concepts helps systematic
understanding.
Understanding is a process rather than a state.
A concept may unfold over time as the reader's mental model grows and refines.

Narrative shapes understanding.
It can be tailored to the reader's current understanding and needs,
which demands not only presentation but also interaction.

Materialized narratives may live in the store as guides.
They may state prerequisites,
choose a base language,
and refer to concept entries at the end or along the way.
These prerequisites and language choices belong in prose.
They are not mechanically tracked metadata.

An *interactive narrative* presents an entry through dialogue,
asking positioning questions,
observing responses,
and generating the next paragraph or quiz from the reader's current state.

The generated narrative is ephemeral;
canonical knowledge remains in entries and relations,
while the narrative provides a reading interface for onboarding and knowledge
transfer.

The initialized `narrative` entry is ordinary.
It is created by `init`.
It is not privileged by the system.

---

## Categories

Categories are entries.
An entry uses `category` to classify itself by other entries.

There is no separate `meta` field.
The category id `meta` classifies entries that define categories.
For example,
the `concept` and `narrative` entries may themselves be categorized by `meta`.

Categories give projects an extensible vocabulary.
Sirno does not require a closed set of entry kinds.

The `locked` field is future work.
It may later protect entries or regions that a project wants to treat as
controlled.
It is not designed here.

---

## Clustee Of A Clique

Tags,
scopes,
namespaces,
and domains all approximate the same structure:
a named clique of related entries.
Such a named clique is an entry itself.

A *clustee* relation is a metadata block field of the clique member.
It groups entries by shared subject,
local vocabulary,
or design neighborhood,
and the clique name provides a short route into a region of the store
without changing the entries' nominal identities.

The named entry used in `clustee` is the clique closure.
The closure is itself an ordinary entry.
It gives the group a name and a place for explanation.

The mechanism can describe an undirected relation by using a two-member clique
closure.
The closure entry records why the two members belong together.
There is no additional mechanism for that case.

---

## Refinement

Refinement unfolds a high-level idea into lower-level design,
implementation,
and tests.
The refined entry keeps the meaning of the concept intact,
while making its consequences local and concrete.

A refinement chain is a path of increasing specificity.
It starts from a compressed concept and ends near repository text,
preserving the reason that a local decision exists.

If the programming language itself is expressive and clean enough such that
the logic of the design is the clearest when expressed in code,
then the final step of refinement may be a Markdown code block.

A *refiner* relation is a metadata block field of the refined entry.
It points to the entry that it refines.

The field is list-valued.
An entry may refine several entries when the local design realizes several
broader claims.

---

## Mirror Design With Witness

A *witness* relation is a metadata block field of the witnessed entry.
It points to repository artifacts that evidence the entry's claim.

When an entry describes behavior,
representation,
or invariant,
the witness is the concrete program text against which that claim can be
checked.
A test may witness an entry when the test itself is the relevant code.

Sirno queries witnesses through `mosaika` by entry id.
The witness may be source code,
tests,
configuration,
generated files,
assets,
or any repository artifact that `mosaika` can mark and query.

Sirno does not store witness queries in entry metadata.
The query key is the entry id.
This keeps the witness relation nominal.
It also keeps repository marking separate from entry prose and metadata.

The entry body may describe how to search for or interpret an artifact.
That prose is fallback guidance.
The structural convention is the marker plus the entry id.

---

## Directions

Sirno names four directions between its surfaces.

- `lower`: `mono -> sirno`
- `raise`: `sirno -> mono`
- `realize`: `sirno -> code`
- `reflect`: `code -> sirno`

The direct names are also useful:
`mono-to-sirno`,
`sirno-to-mono`,
`sirno-to-code`,
and `code-to-sirno`.

These names are conceptual operations.
They are vocabulary for humans,
LLMs,
skills,
CLI surfaces,
and MCP tools.
They need not all be implemented as one-shot commands.

Lowering gives broad design compact nominal form.
It splits a long narrative into entries without losing the design route that
made the narrative readable.
It should create named objects that future work can cite without retelling the
whole design.

Raising composes entries into a readable monograph.
It is not concatenation.
The monograph introduces terms once,
trusts them afterward,
and omits detail that belongs in local entries.

Realizing uses entries to guide implementation.
A realization step should be able to answer which entry explains the local
design commitment.
Not every line of code needs its own entry.
Important commitments need a nominal place.

Reflecting records durable design facts learned during implementation.
Reflect when code changes a representation,
narrows an invariant,
introduces a new boundary,
invalidates an old explanation,
or reveals a clearer local design than the current entries record.

---

## Metadata

Every entry has a YAML metadata block.
The required fields are `name` and `description`.
Both are plain strings.

The optional structural fields are `category`, `clustee`, and `refiner`.
When present,
they are always lists.
Their values are entry ids.

The optional `witness:` marker is canonical and has no value.
No other witness spelling is accepted.

```yaml
---
name: Witness
description: A relation between an entry and repository artifacts.
category:
  - concept
refiner:
  - relation
witness:
---
```

Operational relations are formed only from metadata.
Markdown links in prose may help readers and external tools,
but they do not define Sirno structure.

Sirno validates references lazily.
Dangling `category`, `clustee`, and `refiner` ids may warn during edits.
They are errors during an explicit check.
Witness validity is checked only during an explicit check.

---

## Generated Footers

Sirno may generate and maintain a footer at the bottom of entries.
The footer is bounded by sentinels.
The sentinels state that Sirno owns the region and that humans and tools should
leave it untouched.

The footer format is configurable.
It may use ordinary Markdown links or Obsidian-style links.

The footer is not the source of Sirno structure.
It reflects metadata-derived structure for external tools that navigate links.

---

## Storage And Interfaces

The entry store is managed through `eter`.

For this design stage,
`eter` is the durable storage and indexing substrate.
Versioning is future work.

Sirno should expose the store through a CLI and MCP surface.
A lightweight GUI or Obsidian extension may later provide a direct editing
experience.

Repository witnesses are managed through `mosaika`.
The entry id is the query key used by Sirno when locating marks.

---

## Checks

Sirno checks structure.
It does not check semantic truth.

Structural checks include required metadata fields,
accepted field shapes,
reference existence,
generated footer boundaries,
and witness lookup validity when requested.

Checks should be light during editing and strict at explicit review boundaries.
This gives users fast local movement without weakening the final state.

---

## Planning

Planning is not a core Sirno primitive.

Sirno can support persistent planning because entries are durable,
named,
and relationally structured.
A skill may use entries to represent a worklist,
but that worklist is a use of Sirno rather than part of the core ontology.

---

## Project Self-Application

This repository uses Sirno's own model.

`DESIGN.md` is the monograph.
The future store will contain compact entries for the concepts,
relations,
interfaces,
and implementation commitments described here.
The codebase will witness those entries through `mosaika`.

The document may grow long.
It should remain ordered as one narrative.
Local details that become too dense should be lowered into entries,
then raised back only when the monograph needs them.

---

## Future Work

The `locked` field is reserved for later design.
It may define how entries or generated regions resist accidental edits.

`eter` versioning is reserved for later design.
The current design depends only on durable storage and indexing.

The exact naming of the four directions may be refined.
The current names are `lower`,
`raise`,
`realize`,
and `reflect`.

Planning skills are future work.
They may use Sirno entries to leave durable work artifacts without changing
Sirno's core fields.
