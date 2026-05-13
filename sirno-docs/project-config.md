---
name: Project Config
description: The Sirno.toml file that marks and configures a Sirno-managed repository.
category:
  - concept
clustee:
  - sirno-store
refiner:
  - storage-and-interfaces
---

`Sirno.toml` marks a repository as Sirno-managed.

The file configures the monograph,
the public entry store,
and the operational policies that Sirno applies to the store.
It may also configure a private history root.

`[mono].path` names the monograph.
`[store].path` names the Markdown entry store.
`[history].path` optionally names the private `eter` history root.
Relative paths are resolved from the directory that contains `Sirno.toml`.

A project can use Sirno without history.
`sirno init` creates the config and public entry store.
`sirno history init` adds the history config and commits the current public store.

`Sirno.lock` records the public store's history state when history is configured.
It lives next to `Sirno.toml`.
The lock says whether the store is current
or checked out to a historical version.

`[store].ignore` lists paths relative to the store root.
Sirno skips those paths and their descendants while reading, checking,
querying, and changing generated links.
Ignored paths are for adjacent tool state, not for entries.

`[check].link` controls generated-link freshness checks.
It is enabled by default.
Malformed generated-link sentinels remain errors,
because malformed sentinels make Sirno ownership ambiguous.

`[links]` controls which structural fields are projected into generated footers.
`category`, `clustee`, and `refiner` each accept either a boolean
or `{ to = boolean, from = boolean }`.
A boolean applies to both link sides.

`to` links from the entry to metadata targets.
`from` links from the entry to entries that name it as a metadata target.
`links.clique` adds separate clique-derived sections through named clustee closures.

---

> **Sirno generated links begin. Do not edit this section.**

Clustee (to)
- [sirno-store](sirno-store.md)

Refiner (from): (none)

> **Sirno generated links end.**
