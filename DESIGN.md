# Sirno

*Semantic Intermediate Representation of Nominal Obligations*

Sirno is a graph-shaped knowledge database for codebases. It mediates between abstract design knowledge and concrete code through a structured graph of named, agent-maintained knowledge units. Agents consult and update the graph as part of any code-touching operation, keeping design and implementation in agreement.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/sirno-20260401.png" width="40%">
    <source media="(prefers-color-scheme: light)" srcset="assets/sirno-nb-20260401.png" width="40%">
    <img src="assets/sirno-nb-20260401.png" width="40%">
  </picture>
</p>

---

## Motivation

Codebases accumulate knowledge that does not appear in their syntax: invariants on valid states, decisions that foreclose design alternatives, and rationale for structural choices. This knowledge governs correctness and evolution. It has no structured representation in the artifacts that version-control systems track.

In its absence, knowledge migrates into comments, commit messages, and design documents that are disconnected from each other and from the code they describe. A change to code or to a recorded claim has no mechanism to identify which other claims must be re-examined. Consistency between knowledge and code is unverifiable.

In response, Sirno provides the structured representation. Entries name individual claims. Dependencies record causal relationships among them. Groundings bind entries to code locations. Obligations make the effect of a mutation explicit and propagate it to dependent claims. Coherence then states when the graph and repository view agree after a change.

---

## Components

Sirno is defined above two smaller components:

- `eter`, which provides immutable versioned graph storage
- `mosaika`, which provides anti-drift codebase alignment through
  delimiter-based analysis of repository text

Sirno defines the knowledge semantics that use those components.

---

## Core Concepts

### Entry

An entry is the primitive object in Sirno. An entry carries:

- a nominal identifier
- an optional human-readable name
- a concise description
- a full explanation

An entry states one claim about the codebase. The claim may describe an
invariant, a design decision, a representation choice, a module purpose, or
another isolated piece of understanding.

Entries are the only durable owner of explanatory prose in Sirno. When Sirno
needs narrative text for another object, it refers to an entry.

An entry explanation may link to other entries. These links are navigational
references in prose. They do not create propagation edges.

### Dependency

A dependency `X -> Y` states that `Y` must be re-examined when the content of
`X` changes.

Dependency direction is the direction of causal force. The source entry is the
claim being depended upon. The target entry is the claim whose validity depends
on the source.

A dependency may refer to an additional entry that explains what the dependency
means. That entry is descriptive metadata. The operational semantics of the
dependency are determined by the dependency endpoints.

### Grounding

A grounding binds an entry to repository text. The binding is stored as a
Sirno grounding specification interpreted through `mosaika`.

A grounding has three components:

- a source selection over files
- one or more delimiter-based log transforms
- a Sirno interpretation of the resulting regions

Sirno uses three grounding interpretations.

An anchor grounding is a one-delimiter region that marks the nominal presence of
the entry in source text.

A region grounding is a region associated with the entry for inspection,
reflection, or actualization. It is not evidentiary by itself.

A witness grounding is a region designated as evidence for the entry's claim.

Groundings are defined over repository artifacts in their textual form.

### Lifting

Lifting is the abstraction from repository observations back into the Sirno
graph.

Lifting consumes grounded repository regions and produces Sirno field writes. It
may create entries, revise entry text, revise dependency egress, and revise
grounding specifications.

Lifting is the primary operation in reflection.

### Obligation

An obligation is a proof burden created by a claim-bearing change.

A change is claim-bearing when it changes either:

- the text of an entry
- the dependency egress of an entry

Grounding changes and lock-state changes are not claim-bearing. They change
repository interpretation or authority. They do not change downstream validity
by themselves.

If a claim-bearing write changes entry `X`, every dependency `X -> Y` in the
resulting graph creates an obligation on `Y`.

### Lock

A lock is a write-capability boundary on an entry.

A locked entry may be read, grounded, and used during propagation. Changing its
claim-bearing fields requires external approval.

Locks protect entries with wide consequences, such as architectural decisions,
global invariants, and externally promised guarantees.

### Justification

A justification is the review object for a proposed locked-entry change.

It contains the proposed write together with an argument entry that explains the
change. The rationale is an entry so that it remains part of the graph rather
than transient review metadata.

### Coherence

A Sirno snapshot is coherent when all of the following hold:

- every obligation induced by the write has been discharged
- every locked-entry change has been approved
- every grounding specification is valid under the `mosaika` analysis model
- every required anchor and witness validates against the repository view used
  for the write

Coherence is the global well-formedness invariant of Sirno.

---

## Storage Model

Sirno is stored as an `eter` node schema.

Every Sirno entry is an `eter` node. The entry identifier is the `NodeId`. A
durable Sirno state is an `eter` snapshot identified by an `Eterator`.

The logical Sirno fields are:

- lifecycle
- entry name
- entry description
- entry explanation
- dependency egress
- grounding specifications
- lock state

The lifecycle field is the `eter` lifecycle field. Sirno uses it to determine
whether an entry exists at a snapshot.

Sirno chooses non-reuse of entry identifiers. Once an identifier has existed,
it remains reserved even after deletion. Nominal identity therefore persists
across the whole graph history.

Dependency egress is stored on the source entry. Reverse adjacency is derived
state.

Grounding specifications are stored as typed Sirno data compatible with the
`mosaika` analysis model.

Locks are stored on entries because authority is part of the graph state.

History is `eter` history.

---

## Repository Semantics

Sirno uses `mosaika` to define and validate grounded repository regions as part
of codebase alignment.

The grounding language is delimiter-based. A grounding identifies source files,
declares delimiter-based log transforms, and interprets the resulting regions as
anchors, regions, or witnesses.

`mosaika` replacement actions belong to actualization tooling that rewrites
repository text to satisfy entries. Sirno grounding uses the analysis side of
`mosaika`.

Grounding validation has three layers.

The first layer is specification validity. The source selection, delimiters, and
region interpretation must form a valid `mosaika` analysis specification.

The second layer is repository analysis. The `mosaika` analysis must resolve the
selected files and produce the required regions without ambiguity.

The third layer is Sirno interpretation. Anchors must bind to the owning entry.
Witnesses must remain evidentiary for the entry's claim. Required grounded
regions must be present.

Groundings are evaluated relative to a repository view. In a repository-backed
deployment, that view is typically a checked-out tree plus any in-progress code
changes owned by the active task.

---

## Operational Model

### Polarity

Sirno uses two reasoning polarities.

In actualization, the graph is authoritative. Repository text is rewritten to
satisfy the selected entries.

In reflection, the repository is authoritative. Repository observations are
lifted back into the graph.

Polarity changes the direction of reasoning. Dependency direction, lock rules,
and storage semantics remain fixed.

### Session

A session is a client-side working interval rooted at one base `Eterator`.

The session holds:

- the base snapshot
- the proposed Sirno field writes relative to that snapshot
- the obligations induced by those writes
- the repository view used for grounding validation
- any pending justifications for locked entries

The session is not a durable storage primitive. Durability begins when the
session commits one `eter` write transaction and receives a new `Eterator`.

### Commit

A commit is the `eter` write transaction that materializes the session's field
writes.

The session computes the resulting Sirno state, validates coherence against the
repository view, and then writes the accepted field rows. If the write
succeeds, `eter` returns a new `Eterator`. That snapshot is the new durable
Sirno state.

Sirno has one write boundary. Repository analysis occurs before the `eter`
commit. Repository materialization, when actualization edits code, also occurs
before the `eter` commit. The graph is committed only after the repository view
and the graph view agree.

---

## Propagation Semantics

Propagation follows dependency edges in their declared direction.

When a session stages a claim-bearing change to entry `X`, Sirno computes the
dependency egress of `X` in the resulting graph. For each dependency `X -> Y`,
Sirno creates an obligation on `Y`.

An obligation is discharged in one of three ways.

Confirmation records that `Y` remains valid under the new upstream state.

Revision records new field writes for `Y`. If that revision is claim-bearing,
propagation continues from `Y`.

Approval records that a previously justified change to a locked `Y` is
accepted. The approved writes are then applied and propagated in the same way
as any other revision.

Cycles are handled at the level of strongly connected components. Every entry in
the component must be re-examined against the same candidate state. The
component is discharged only when its entries reach a fixed point.

---

## Boundary

`eter` provides immutable typed graph storage. `mosaika` provides typed
repository alignment over delimiter-defined textual regions.

Sirno adds:

- entries as nominal knowledge claims
- dependency as the causal graph relation
- grounding interpretation as anchors, regions, and witnesses
- lifting from repository observations into graph writes
- obligation propagation over dependency edges
- locks and justifications for claim-bearing writes
- coherence as a joint invariant over graph state and repository state

Everything else belongs to `eter`, `mosaika`, or application-specific tooling
built above them.
