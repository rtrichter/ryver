# Architecture Notes

This document records the starting architectural direction. It is expected to change as the implementation reveals constraints.

## Implemented Repository Layout

The repository metadata directory is `.ryver/`.

The first implementation writes this structure:

```text
.ryver/
  objects/
  patches/
  refs/
  indexes/
  config.toml
```

The exact layout can change, but the separation matters:

- immutable portable objects should be durable;
- refs and indexes can be mutable;
- indexes should be rebuildable;
- third-party metadata should use namespaces.

Current storage behavior:

- `objects/` stores content-addressed file blobs by SHA-256 hash;
- `patches/` stores portable JSON patch records named by patch id;
- `refs/HEAD` stores the current patch id, or is empty before the first recorded patch;
- `indexes/` is created but not used yet;
- `config.toml` records the initial format choices.

## Core Object Types

Early core objects:

- patch/commit;
- operation;
- dependency;
- file tree entry;
- materialized snapshot/checkpoint;
- issue;
- proposal;
- review comment;
- check result;
- entity record, once semantic indexing exists.

## Patch-Oriented Commits

A commit represents a patch/change, not merely a snapshot.

A patch may contain:

- id;
- author;
- timestamp;
- message;
- dependencies;
- operations;
- inverse operations where known;
- affected files;
- affected entities;
- metadata.

The current patch schema records:

- `id`;
- optional `parent`;
- `author`;
- Unix `timestamp`;
- `message`;
- file-level `operations`;
- resulting file `tree`.

This is intentionally narrow. It establishes a portable object and parent chain before adding dependency algebra, inverses, materialization, or semantic operations.

## Operation Model

Start with text and filesystem operations:

- create file;
- modify file;
- delete file.

The first implementation records whole-file creates, modifies, and deletes. Text hunks, moves, copies, executable-bit changes, and operation inverses are still future work.

Later semantic operations:

- rename entity;
- move entity;
- change signature;
- add import;
- remove import;
- update references;
- extract function;
- inline function.

## Storage

Use boring, durable formats. Avoid clever storage before the invariants are known.

Potential split:

- content-addressed files or records for immutable objects;
- SQLite for local indexes and query acceleration;
- TOML for human-editable config;
- JSON, MessagePack, CBOR, or protobuf for portable object records after tradeoffs are evaluated.

## Remotes

The first remote can be extremely simple because issues, proposals, reviews, checks, and patches are local files or portable objects. A remote can initially be file sharing plus conflict detection.

Future remotes can add performance and convenience features such as caching, partial fetches, branch/channel filtering, hosted indexes, check execution, permissions, and web views. Those features should optimize the model, not own it.

The important constraint is that a hosted remote should not become the only place where project workflow objects exist.

## Semantic Indexing

Tree-sitter is a strong candidate for concrete syntax trees and cross-language parser reuse. It does not provide full semantics by itself.

Language-specific layers will still be needed for:

- name resolution;
- symbol identity;
- imports/modules;
- type-aware refactors;
- macro or metaprogramming behavior;
- confidence scoring for rename/move detection.

## Editor Integration

Editors and language servers can provide intent that raw diffing cannot reliably infer.

Ryver should eventually expose a small operation API so tools can record actions like:

- rename symbol;
- move symbol;
- change signature;
- apply codemod;
- add changelog entry;
- run formatter.

## LLM Integration

LLMs should eventually emit structured operations or codemods rather than textual patches. Ryver can then validate those operations, attach them to reviewable commits, and make them undoable.

This requires a stable operation schema and clear validation model. Model-driven and visual editors should remain text-represented so LLMs can inspect, modify, and generate them without depending on opaque editor state.
