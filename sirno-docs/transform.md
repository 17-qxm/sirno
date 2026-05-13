---
name: Transform
description: A concept that names movement between Sirno forms.
category:
  - concept
---

A transform names a kind of work between Sirno forms.

Sirno uses four transform names:
`lower`, `raise`, `realize`, and `reflect`.

Their direct names are also useful:
`mono-to-sirno`, `sirno-to-mono`, `sirno-to-code`, and `code-to-sirno`.

Transforms are vocabulary for humans, LLMs, skills, CLI interfaces, and MCP tools.
They describe coherent work without requiring every transform to be a one-shot command.

The transform names make design work easier to request and review.
Instead of saying "split this document into smaller pieces" every time,
a user can ask to lower a monograph into the store.
Instead of saying "update the design notes based on this code change",
a user can ask to reflect the code into entries.

The four transforms form a loop:
`mono` lowers into `sirno`,
`sirno` realizes into `code`,
`code` reflects back into `sirno`,
and `sirno` raises into `mono`.
The loop is conceptual, not automatic authority.
Each transform should still be performed with judgment about the current source of truth.

This vocabulary also helps skills stay focused.
A lowering skill should preserve narrative intent while creating entries.
A realization skill should inspect entries before editing code.
A reflection skill should record durable design facts learned from implementation.
A raising skill should compose a readable monograph.

---

> **Sirno generated links begin. Do not edit this section.**

Clustee (to): (none)

> **Sirno generated links end.**
