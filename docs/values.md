# Values

Ryver exists because version control can be more humane, more understandable, and more closely aligned with how programmers actually change code.

## Usability

Common workflows should be obvious and hard to misuse. Dangerous operations should be inspectable and reversible when possible. The system should explain what happened in terms of changes, not implementation trivia.

## Learnability

A new user should not need to learn years of historical accidents before making productive use of the tool. Advanced workflows can exist, but the core model should be teachable: patches, dependencies, working state, objects, proposals, and sync.

## Performance

Version control is infrastructure. Basic operations should feel instant on small repositories and remain usable on large repositories. Expensive semantic indexes should be incremental, cacheable, and disposable.

## Correctness

Ryver should prefer visible conflicts over incorrect clean merges. Semantic automation must be conservative. If the system is uncertain, it should say so and preserve enough context for a human to decide.

## Extensibility

Third parties should be able to build on Ryver without forking core. Extensions should be able to add namespaced metadata, hooks, checks, sync adapters, visualizations, and language integrations.

## Flexibility

Ryver should not force one hosted service, one review workflow, or one project-management model. Issues, proposals, reviews, checks, and metadata should exist locally and sync outward.

## Local-First Ownership

A repository should contain the project's essential collaboration state. Hosted services are useful, but they should not be the only place issues, reviews, proposals, and checks can exist.

## Semantic Awareness

Source code is structured. Ryver should eventually understand code entities and intentional refactors natively, while still preserving text-level escape hatches.

## LLM Compatibility

LLM coding agents should be able to interact with code through structured changes. Instead of emitting fragile textual patches, an agent should eventually be able to propose codemods, refactors, or semantic operations that Ryver can validate, apply, review, and undo.

## Public Good

The end goal is the product and the workflow improvement, not ownership of the winning implementation. If another project builds the right system first, that is still progress for software engineering. Ryver should share ideas publicly by default so the broader ecosystem can learn from them, reuse them, challenge them, or outbuild them.

## FLOSS

Ryver should be free and open-source software. The project should be useful as a personal tool even if it never becomes commercially successful. A closed-source competitor proving the ideas would still validate the direction, but it would be an incomplete win because the desired product includes user freedom, inspectability, and community ownership.
