# Tan CLI

A CLI for Tan.

## Setup

```sh
cargo install --path .
```

## Commands

- .. (repl)
- run
- build
- fmt
- lint
- pkg
- version
- help

## TODO

### REPL

- custom prelude for RELP, with `help`, `config`, etc.
- use custom unicode chars in the prompt, e.g. -> , <-
- mark entries with indexes (0, 1, 2), allow refs, e.g. $0 $1, etc.
  - use In and Out arrays/lists like Wolfram.
- allow to skip top-level parens
- allow multi-line entry
- find better name, e.g. `shell`?
- extract to separate crate
- autocomplete
- syntax high-lighting

#### REPL Done

- show detailed error (use pretty error formatter)
- NO: quit the repl with `\q` (escape), similar to postgres. <-- No, just quit with (exit) or CTRL+D

