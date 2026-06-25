# CLI

Ryver currently provides a small experimental CLI for initializing a local store and recording whole-file patch objects.

## Commands

Initialize a repository:

```text
ryver init
```

Show working tree changes relative to the current Ryver patch:

```text
ryver status
```

Record the current working tree changes as a patch:

```text
ryver record --message "describe the change"
```

Show the patch chain from newest to oldest:

```text
ryver log
```

Show the current patch, or a specific patch by id:

```text
ryver show
ryver show <patch-id>
```

## Author Selection

`record` uses the first available author source:

- `--author`;
- `RYVER_AUTHOR`;
- `GIT_AUTHOR_NAME` and `GIT_AUTHOR_EMAIL`;
- Git config `user.name` and `user.email`;
- `unknown <unknown>`.

## Current Limits

The first implementation records whole-file create, modify, and delete operations. It does not yet support text hunks, patch application, patch unapplication, conflict detection, operation inverses, remotes, or semantic Rust indexing.
