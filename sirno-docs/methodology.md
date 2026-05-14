---
name: Methodology
description: A working guide for acting inside the Sirno Lake.
category:
  - concept
  - narrative
clustee:
  - sirno
refiner:
  - concept-driven-development
  - transform
  - witness
---

Sirno is a method for keeping design and implementation in conversation.

The method is disciplined bookkeeping.
It gives people, agents, tools, and editors the same named design objects to inspect.
It does not decide whether a design is good.
It does not prove that code satisfies an entry.
It makes the relevant objects easier to name, connect, revise, and witness.

Start from the lake.
This repository keeps its design source in `sirno-docs/`.
Read `introduction` first when you need the first route through the project.
Then follow categories, clustees, refiners, and witnesses to the local design.

Name the thing before the work becomes local.
An entry should be small enough to read in place
and durable enough to survive the edit that made it useful.
It may name a concept, structural field, refinement, invariant,
implementation commitment, or narrative route.

Use `category` for kind.
Use `clustee` for structural clearness.
Use `refiner` when broad design needs local form.
Use `witness:` when the repository contains evidence for the entry claim.
Leave a structural field out when it does not improve navigation, review, or accountability.

Lower when intent is too broad for the next local change.
Lowering turns narrative design into compact lake entries.
It should preserve the route that made the narrative readable
while giving future work stable handles.

Realize from named objects.
Before editing code,
read the entries that govern the work.
Inspect their clustees, refiners, and witnesses.
Implementation should be able to answer which entry explains an important commitment.

Reflect while the code change is fresh.
Reflect when implementation changes a representation,
narrows an invariant,
introduces a boundary,
invalidates an explanation,
or reveals a clearer local design.
The reflected prose should record the durable design fact,
not narrate the whole edit.

Raise only when a project needs a long-form narrative outside the lake.
Raising composes entries into a readable monograph.
It is not concatenation.
It chooses a route,
introduces terms once,
and leaves dense local detail in entries.

Witness important claims.
The witness may be source code, tests, configuration, generated files, or assets.
Sirno queries witnesses by entry id through `mosaika`.
The entry states the design claim.
The witness block identifies the repository region to inspect.

Let Sirno maintain generated footers.
The generated region is bounded by sentinels and Sirno-owned.
Metadata remains the source of structural truth.
The footer exists for navigation and interoperability.

Check at review boundaries.
During editing, some structural problems can remain warnings.
At review boundaries, dangling structural ids and missing witnesses should be errors.
Checks confirm structure.
They do not replace judgment about meaning.

Treat planning as a use of Sirno, not a core primitive.
A worklist can be represented as ordinary entries when that helps.
Those entries can use categories, clustees, refiners, and witnesses like the rest of the lake.

The habit is simple.
Name the thing.
Write the entry.
Classify it only when classification helps.
Cluster it only when the shared subject deserves a review front door.
Refine it when broad design needs local form.
Witness it when the repository contains its evidence.

Sirno keeps the structure ready.
People and agents keep the meaning alive.

---

> **Sirno generated links begin. Do not edit this section.**

Clustee (to):
- [sirno](sirno.md)

> **Sirno generated links end.**
