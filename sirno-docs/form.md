---
name: Form
description: A shape project knowledge takes inside Sirno.
category:
  - concept
  - narrative
clustee:
  - sirno
---

Sirno works through three forms.

`mono` is one optional configured Markdown document.
It carries the whole project design as a readable monograph.

`sirno` is one configured entry store.
It contains compact Markdown entries with exact metadata.
When history is configured, it is versioned through a separate `eter` history root,
so one store version names one immutable entry set.

`code` is the repository implementation form.
It contains source, tests, configuration, generated files, assets,
and any artifact that can realize or witness design.
Sirno scans code witnesses only when `[code].members` is configured.

The forms are not just storage locations.
They are roles in a design workflow.
The monograph is optimized for continuity,
so a reader can build a mental model in a deliberate order.
The store is optimized for addressability,
so a person or tool can find the named object that matters to a local change.
The code form is optimized for execution and evidence,
so design commitments have concrete artifacts to inspect.

Before the store exists, the user chooses whether the codebase or monograph carries more authority.
Once the store is established, Sirno treats it as the structured intermediate form.

That authority can still be revised by deliberate work.
Lowering lets a monograph seed the store.
Reflecting lets implementation discoveries update the store.
Raising lets the store rebuild a whole-project narrative.
Realizing lets entries guide implementation.

Keeping the three forms distinct prevents one document from trying to serve every reader at once.
The monograph can stay fluent.
Entries can stay compact and named.
Code can stay focused on behavior while still having a place to point for intent.

---

> **Sirno generated links begin. Do not edit this section.**

Clustee (to):
- [sirno](sirno.md)

> **Sirno generated links end.**
