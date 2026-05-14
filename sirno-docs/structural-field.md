---
name: Structural Field
description: A metadata field that carries operational Sirno structure.
category:
  - concept
clustee:
  - sirno
witness:
---

A structural field is a metadata field that Sirno reads as project structure.

The structural fields are `category`, `clustee`, `refiner`, and `witness:`.
They are ordinary entry metadata,
but Sirno treats their values as the graph that powers query, checking, generated links,
and repository witness lookup.

The first three structural fields refer to entries by id.
They are list-valued and may name several targets.
`witness:` is a canonical marker without a value.
Its presence declares that the entry should have repository evidence.

This entry is the clique closure for the structural field entries.
It gives the field set one review front door while leaving each field entry free
to carry its own meaning and other clustee memberships.

---

> **Sirno generated links begin. Do not edit this section.**

Clustee (from):
- [category](category.md)
- [clustee](clustee.md)
- [refiner](refiner.md)
- [witness](witness.md)

Clustee (to):
- [sirno](sirno.md)

Refiner (to): (none)

> **Sirno generated links end.**
