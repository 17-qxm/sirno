---
name: Storage And Interfaces
description: The design commitment to eter storage and CLI or MCP access interfaces.
category:
  - concept
clustee:
  - sirno
---

The entry store is managed through `eter`.
At this design stage, `eter` provides durable storage and indexing.
Versioning is reserved for later design.

Sirno exposes the store through CLI and MCP interfaces.
A lightweight GUI or Obsidian extension may later provide a direct editing experience.

Repository witnesses are managed through `mosaika`.
The entry id is the query key Sirno uses when locating marks.

The storage design separates the public Markdown form from the durable substrate.
Markdown entries are the human-facing form.
They are easy to read, review, diff, and edit.
`eter` provides the storage and indexing foundation beneath that form,
so Sirno can grow more capable without making the entry files opaque.

The CLI is the first operational interface.
It can initialize stores, create entries, query entries, check structure,
and maintain generated link footers.
Those commands should remain plain enough to use from a terminal
and stable enough for agents and skills to call.

`sirno status` summarizes the configured repository.
It reports the config path, monograph path, store path, entry count,
check policy, link policy, and current check result.

`sirno new` creates one Markdown entry from typed command-line metadata.
It refuses to overwrite an existing entry file.

`sirno query` reads the configured Markdown store.
Its default mode is vague text query.
Exact structural predicates live behind explicit exact flags.

`sirno gen-link` reports generated footer regions that would change.
`sirno gen-link --no-check` creates or replaces Sirno-owned generated footer regions.
`sirno gen-link delete` removes those regions.
Both commands operate on the configured store unless an explicit entry directory is given.

`sirno util completion` emits shell completion scripts.
Completion generation is a utility interface,
not a store operation.

The MCP interface serves interactive tooling.
It can expose the same store model to agents and editors without asking them to shell out for every action.
Future GUI or Obsidian work should keep the same ownership rules:
metadata is structural,
generated footer regions are Sirno-owned,
and prose outside generated regions remains user-owned.

Witness lookup stays separate through `mosaika`.
That lets repository marks evolve with code navigation needs
while Sirno keeps the entry id as the shared nominal handle.

---

> **Sirno generated links begin. Do not edit this section.**

Clustee (from): (none)

Clustee (to)
- [sirno](sirno.md)

Refiner (from)
- [project-config](project-config.md)
- [query](query.md)

> **Sirno generated links end.**
