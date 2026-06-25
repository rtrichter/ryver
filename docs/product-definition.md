# Product Definition

Ryver is a local-first, free and open-source version control and collaboration system for source code. It treats changes, code entities, reviews, and project coordination objects as first-class local data instead of delegating those concepts to a hosted forge.

The project exists to improve the software engineering process. Ryver succeeding is one path to that goal, but the broader goal is for this kind of product to exist, be understandable, be inspectable, and be available as FLOSS. Ideas should be shared publicly so other tools can adopt or improve them.

The initial implementation will be a Rust command-line tool and local project store. The first engine should be patch-based rather than snapshot-first: a commit represents a patch/change with dependencies, operations, metadata, and inverses. Snapshots may be materialized and cached, but they should not be the only source of truth.

## Core Thesis

Modern version control should understand more than files and lines. It should understand intentional changes and, over time, code entities such as functions, classes, modules, imports, traits, structs, and methods.

Ryver should eventually support workflows such as:

- restore the old name of a function without reverting later body edits;
- show history for a specific symbol across moves and renames;
- represent a refactor as a semantic operation instead of a pile of text edits;
- review proposals locally without depending on GitHub, GitLab, or another hosted forge;
- store issues, review threads, checks, and project metadata locally;
- allow remotes and third-party services to sync and extend the same local object model;
- let editors, visual tools, and LLMs submit codemods or semantic operations instead of raw patches.

## Target Audience

The initial audience is the project author and developers who want better version control, local-first collaboration, and semantic code history. The long-term audience also includes non-programmers who could participate in software creation through visual, model-driven, or domain-specific editors backed by real source code and real version history.

## Initial Scope

Ryver starts as a personal learning and tooling project, not as a commercial Git replacement. The early goal is to build a working VCS core in Rust, learn the hard constraints of patch-based history, and then add semantic code-awareness on top.

The first versions should prioritize:

- a clear local storage model;
- a usable CLI;
- patch recording and application;
- operation-level undo where practical;
- a foundation for future semantic operations;
- local storage for issues, proposals, reviews, and checks;
- documentation of tradeoffs and invariants as they are discovered.

## Non-Goals For Early Versions

Ryver does not need to be Git-compatible. It does not need to interoperate with GitHub. It does not need to be commercially viable. It does not need to solve every programming language at once.

Early versions should avoid promising general semantic merge. The first semantic features should be narrow, inspectable, and conservative.
