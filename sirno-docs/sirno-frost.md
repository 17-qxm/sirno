---
name: Sirno Frost
description: The private eter root that freezes immutable snapshots of the public Sirno Lake.
category:
  - concept
refines:
  - versioning
---

Sirno Frost is the private `eter` root for frozen Sirno snapshots.
The default convention is `sirno-frost`.
It is optional.
`sirno frost init` adds it to a project and freezes the current public lake.
`sirno frost mv PATH` renames the configured Frost root
and writes the new path back to `[frost].path`.
The move refuses to replace an existing destination.

The public lake remains the editable Markdown working form.
Sirno Frost records immutable versions of that form.
It is not read as part of the public entry lake,
and it should not live under a path that Sirno scans for entries.

A freeze imports the selected public entry set into Sirno Frost.
It writes one `eter` transaction and returns a `SnapshotRef`.
That snapshot reference names the whole committed lake state.
Before writing the transaction,
Sirno removes generated-link regions from committed entry bodies.
If the public lake is unchanged,
the freeze returns the current snapshot reference without writing.
If a previously live entry is missing from the public lake,
the freeze records an `eter` lifecycle deletion marker.

Checkout reads a selected frozen snapshot and writes its live entries as Markdown files.
The conservative checkout policy writes only into an absent or empty target directory.
CLI checkout replaces managed Markdown files in the configured public lake
and preserves ignored paths.

`Sirno.lock` records whether the public lake is current
or checked out to a frozen snapshot.
A normal checkout is made read-only through file permissions.
The checked-out entry body also starts with a visible Markdown blockquote
that says not to edit the file by hand.
`--unsafe-mutable` leaves the checkout writable and records that choice in the lock.

Sirno Frost is private substrate.
Users and tools may inspect it when debugging storage,
but normal Sirno work should read and edit the public lake
or use version-aware Sirno interfaces.

---

> **Sirno generated links begin. Do not edit this section.**

Belongs (from): (none)

Belongs (to): (none)

Refines (from): (none)

Refines (to):
- [versioning](versioning.md)

> **Sirno generated links end.**
