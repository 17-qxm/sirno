---
name: Generated Link Policy
description: Configuration that chooses which structural links appear in generated footers.
category:
  - concept
clustee:
  - generated-footer
refiner:
  - generated-footer
witness:
---

Generated link policy decides which metadata-derived sections appear in a generated footer.

`category`, `clustee`, and `refiner` can each generate outgoing links to targets
and incoming links from sources.
A boolean setting enables or disables both link sides.
A `{ to = ..., from = ... }` setting chooses the two link sides separately.

`links.clique` adds separate clique-derived sections.
It does not change direct clustee projection.
When enabled, each clustee closure induces clique edges:
the closure links to its members,
and members link to the closure and to one another.
When disabled, only configured structural field sections are rendered.

This policy is configuration, not entry data.
Changing it alters generated navigation surfaces without changing structural metadata.

---

> **Sirno generated links begin. Do not edit this section.**

Clustee (to):
- [generated-footer](generated-footer.md)

> **Sirno generated links end.**
