# Roadmap

This roadmap is intentionally staged. Ryver should become useful by building a small reliable core first, then layering semantic code understanding and collaboration features on top.

## Phase 0: Project Foundation

- Create the Rust crate and basic CLI structure.
- Define repository layout under a hidden project directory, likely `.ryver/`.
- Document core object types and storage invariants.
- Add basic tests from the beginning.
- Decide on serialization formats for portable objects and local indexes.

## Phase 1: Text-Based Patch VCS

Build the smallest useful VCS before adding AST awareness.

Features:

- `init`
- `status`
- `record` or `commit`
- `log`
- `show`
- materialize working tree state
- apply and unapply patches
- detect simple conflicts
- store patch dependencies
- support operation-level inverses where possible

Design target:

- commits are patches, not just snapshots;
- snapshots may exist as caches or checkpoints;
- indexes are disposable;
- core objects are portable and documented.

## Phase 2: Better Frontend And Operation Log

Borrow the best UX lessons from jj without copying Git's mental model.

Features:

- operation log for repository changes;
- patch stack editing;
- easy undo of repository operations;
- clear conflict visibility;
- revset-like query language or simpler equivalent;
- local web or TUI view for history exploration.

## Phase 3: Local Collaboration Objects

Make project coordination local-first.

Features:

- local issues;
- local proposals, similar to pull requests;
- review comments attached to files, patches, ranges, and later entities;
- check results;
- approvals and review state;
- syncable object model for future remotes.

## Phase 4: Rust Semantic Layer

Add code entity indexing and semantic operations for Rust first.

Initial entity types:

- modules;
- functions;
- structs;
- enums;
- traits;
- impl blocks;
- methods;
- imports.

Initial semantic operations:

- rename entity;
- move entity;
- add/remove/reorder imports;
- change function signature;
- update references when tool support is reliable;
- show history for an entity across moves and renames;
- restore one property of an entity without reverting unrelated edits.

## Phase 5: Editor And Refactor Integration

Integrate with editors and language servers so intent is captured directly.

Features:

- record refactor operations from editor actions;
- consume language-server rename/move/change-signature operations;
- expose Ryver operations to LLM coding agents;
- allow agents to emit codemods or semantic operations instead of raw patches.

## Phase 6: Additional Languages

Add a second language to stress the abstraction, likely Python because it is popular and semantically challenging.

Add a third language from a different paradigm or ecosystem, such as Java, C#, Haskell, OCaml, or F#.

The goal is not to pretend all languages are the same. The goal is to identify which parts of semantic version control can be generic and which parts must remain language-specific.

## Phase 7: Remotes And Shared UI

Define remotes as sync targets for documented local objects, not as owners of workflow concepts.

Features:

- remote sync protocol;
- hosted or self-hosted remote service;
- reusable web UI for remotes;
- extension points for third-party services;
- local review remains available without a web service.
