# Open Questions

This document captures unresolved design questions. These should become implementation experiments, not permanent abstractions.

## Patch Algebra

How much of Pijul or Darcs patch theory should Ryver adopt directly, and where should it simplify for the first implementation?

Can Ryver support operation-level undo without solving full general patch commutation immediately?

## Storage Format

What belongs in immutable portable object storage, and what belongs in a disposable local database?

Should portable objects use JSON, CBOR, MessagePack, protobuf, or another format?

## Identity

How should Ryver assign stable identity to files and, later, code entities?

How should it handle a function that is renamed, moved, split, merged, or heavily rewritten?

## Conflict Model

What is the smallest conflict model that is better than Git's while still implementable early?

How should conflicts be represented as local objects?

## Local Issues And Proposals

What is the minimal local issue model?

What is the minimal local proposal model?

How should comments attach to patches, ranges, files, and future code entities?

## Extension API

What extension points should exist first?

Potential hooks:

- patch recorded;
- working tree changed;
- issue created;
- proposal updated;
- check completed;
- semantic entity changed.

How should extensions declare capabilities and store namespaced metadata?

## Language Strategy

Rust should come first. Python is a likely second language because it is popular and challenging. The third language should force a different abstraction, possibly Java/C# for OOP or Haskell/OCaml/F# for FP.

When should the project invest in generic language abstractions versus explicit language-specific implementations?

## UI Strategy

Should the first UI be a TUI, a local web UI, or both?

Can the same UI eventually be reused by remotes so project hosting is not fragmented into incompatible web experiences?
