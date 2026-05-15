---
name: Metadata
description: The exact YAML schema that carries Sirno entry structure.
category:
  - concept
belongs:
  - sirno-lake
---

Metadata is the exact schema that carries Sirno structure.

Every entry has a YAML metadata block.
The required fields are `name` and `description`,
both plain strings.

The optional structural fields are `category`, `belongs`, and `refines`.
They are always lists when present, and their values are entry ids.

The optional marker fields are `witness:` and `frozen:`.
They are written without values.
`witness:` declares that repository evidence should exist for the entry id.
`frozen:` declares that the entry file is read-only
and must be melted before Sirno Frost can commit it.

Operational structure is formed only from metadata.
Prose links may help readers and external tools,
but they do not define Sirno structure.

The metadata block should be small and stable.
It is the part of an entry that tools must read without interpretation.
That is why required fields are plain strings,
and structural fields are lists of ids.

The body can explain nuance,
but the metadata must not require prose parsing.
If a tool needs to know that one entry refines another,
the `refines` field must say so.
If an agent needs to inspect repository evidence for an entry,
it should run `sirno witness ENTRY_ID --full`.

A canonical entry shape looks like this:

```yaml
---
name: Concept
description: A named idea that compresses project knowledge.
category:
  - concept
witness:
---
```

The schema is intentionally conservative.
Adding fields is future design work,
because every accepted field becomes part of the public structure that readers and tools may rely on.

---

> **Sirno generated links begin. Do not edit this section.**

Belongs (from): (none)

Belongs (to):
- [sirno-lake](sirno-lake.md)

> **Sirno generated links end.**
