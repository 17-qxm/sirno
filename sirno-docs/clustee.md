---
name: Clustee
description: A structural field that places an entry in a review neighborhood.
category:
  - concept
clustee:
  - structural-field
witness:
---

`clustee` places an entry in a named review neighborhood.

The target entry is the clique closure.
It gives a shared subject, local vocabulary, or design region a front door.
The relation is horizontal.
A local design or program change should often be reviewed by visiting that closure,
its members,
and their witnesses or refinements.
The field is list-valued and not exclusive.
An entry may belong to several clustees when each target is a real review perspective.

This field covers the useful part of tags, scopes, namespaces, and domains.
The member entries keep their own ids while the clique entry provides a route into the group.

Use `clustee` when entries should be visited together because they share working context.
The field says that the member belongs to a named neighborhood.
It does not say that the member is an instance of a kind
or that it makes the closure entry more concrete.
Use `category` for kind.
Use `refiner` when the current entry narrows a broader design claim.

Keep clustee targets sparse.
A target should help navigation, review, or accountability.
A loose browsing tag should not become structural metadata.

Generated clustee links preserve direct target and source links.
`links.clique` can add separate clique-derived sections.
With clique sections enabled,
the closure links to its members
and each member links to the closure and the other members.

This is useful for domains that cut across categories.
For example, a lake neighborhood may include concepts, metadata rules,
generated footer behavior, and checks.
Those entries are different kinds of objects,
but they belong together because they explain the same part of the project.

The closure entry should carry real explanatory value.
If the group helps a reader enter a complicated region of the lake,
then the closure gives that region a stable front door.
When splitting an entry,
keep the new entries in the same clustee if the same review should visit them together.
Add clustees only when they improve review locality.
Create a new clique closure only when the split creates a real new review boundary.

---

> **Sirno generated links begin. Do not edit this section.**

Clustee (from): (none)

Clustee (to):
- [structural-field](structural-field.md)

Refiner (to): (none)

> **Sirno generated links end.**
