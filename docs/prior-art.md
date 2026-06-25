# Prior Art

Ryver should learn from existing systems without being constrained by their compatibility goals.

## Pijul

Pijul is the most important backend reference. It is a Rust distributed version control system based on patch theory. Relevant ideas include patches as first-class objects, commutation of independent changes, channels, and a conflict model not based on Git snapshots.

## Darcs

Darcs is the older patch-theory system. Relevant ideas include interactive patch selection, patch dependencies, and change-oriented history.

## Jujutsu

Jujutsu is an important frontend and UX reference. Relevant ideas include an operation log, mutable changes, a clearer conflict model, and a more ergonomic command-line interface.

## Fossil

Fossil is an important product-shape reference. Relevant ideas include one-binary distribution, local web UI, integrated tickets/wiki/forum, autosync, and project state that is not scattered across many services.

## SemanticMerge And Mergiraf

SemanticMerge and Mergiraf are relevant syntax-aware merge references. They show that structured merge is useful, but Ryver's goal is deeper: code entities and semantic operations should become native history concepts, not just smarter merge heuristics.

## Language Servers And Refactoring Tools

Language servers and refactoring tools are essential references because they already understand many editor-level semantic operations. Ryver should integrate with them rather than trying to rediscover every refactor from raw text after the fact.
