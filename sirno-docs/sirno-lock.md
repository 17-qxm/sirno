---
name: Sirno Lock
description: The TOML file that records the history state of the public store.
category:
  - concept
clustee:
  - sirno-store
refiner:
  - versioning
---

`Sirno.lock` records the public store's state relative to the configured history root.
It is TOML and lives next to `Sirno.toml`.
It is written only when history is configured.

The lock contains one `[history]` table.
`status = "current"` means the public store represents the current editable history version.
`status = "checked-out"` means the public store materializes a selected historical version.
The `version` field stores the raw `Eterator` value for that state.

A normal checkout is immutable.
Sirno removes write permission from the public store root and managed entry files.
It also writes a visible Markdown blockquote at the start of each checked-out entry body
that says the file is read-only and should not be edited by hand.
`sirno history checkout VERSION --unsafe-mutable` leaves the checkout writable
and records `mutable = true`.

Committing a mutable checkout writes a new current history version
and rewrites the lock to `status = "current"`.
Sirno refuses to commit an immutable checkout.

---

> **Sirno generated links begin. Do not edit this section.**

Clustee (to)
- [sirno-store](sirno-store.md)

Refiner (from): (none)

> **Sirno generated links end.**
