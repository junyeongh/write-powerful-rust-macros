# Write Powerful Rust Macros

Code for the book, organised as a Cargo workspace with one crate per chapter,
so each chapter carries only its own dependencies.

## Layout

```sh
Cargo.toml              # virtual manifest: members + shared dependency versions
docs/                   # chapter notes
chapters/
  ch01-introduction/    # one crate per chapter
```

Chapters that build a procedural macro get a sibling `chapters/chNN-<name>-macro`
crate with `proc-macro = true`, since a proc-macro crate cannot contain the code
that consumes it.

## Commands

| Goal              | Command                          |
| ----------------- | -------------------------------- |
| Run one chapter   | `cargo run -p ch01-introduction` |
| Test one chapter  | `cargo test -p ch01-introduction`|
| Test everything   | `cargo test --workspace`         |
| Add a chapter     | `cargo new chapters/chNN-<name>` |

Dependency versions are pinned once in the root `[workspace.dependencies]`; a
chapter opts in with `<dep>.workspace = true`. `members = ["chapters/*"]` is a
glob, so new chapter crates join the workspace automatically.

## Related

- [Introduction - The Little Book of Rust Macros](https://lukaswirth.dev/tlborm/introduction.html)
