# rat 🐀

**A little Rusty rat chasing a C cat.**

`rat` is a tiny command-line utility written in Rust for exploring files and directories.

It started as a simple exercise in reimplementing `cat`, but quickly grew into something a little different: a tool that knows whether you're looking at a file or a directory and behaves accordingly.

No frameworks. No external dependencies. Just Rust and its standard library.

## Why rat?

The name is a little wordplay:

- **cat** is traditionally written in **C**.
- **rat** is written in **Rust**.
- Cats chase rats. This one chases `cat`.

One letter changes, one language changes, and the mouse gets its turn.

Unlike `cat`, which primarily prints file contents, `rat` also shows file metadata and explores directory structures.

It's not trying to replace `cat`, `tree`, or `ls`. It's just a small experiment in what happens when you combine a few of their ideas.

## Features

**Files**
- Display the path, type, and size.
- Print file contents byte-for-byte, without modification.
- Preserve whitespace and missing trailing newlines.

**Directories**
- Display directory metadata.
- Recursively explore subdirectories.
- Sort entries by filename.
- Indent entries to represent directory depth.
- Support a configurable maximum traversal depth in the source code.

**Under the hood**
- Written entirely in Rust.
- Zero external dependencies.
- A single Rust source file.
- Error handling using `Result` and meaningful exit codes.

## Getting Started

You'll need Rust and Cargo installed.

Clone the repository and build:

```bash
git clone https://github.com/NingAloha/rat
cd rat
cargo build --release
```

Or run directly with Cargo:

```bash
cargo run -- <path>
```

## Usage

### Inspect a file

```bash
cargo run -- Cargo.toml
```

Example output:

```text
Path: Cargo.toml
Type: Regular file
Size: 83 bytes
------------------------
[package]
name = "rat"
version = "0.1.0"
edition = "2024"

[dependencies]
```

`rat` copies file contents directly to standard output without changing the original bytes.

### Explore a directory

```bash
cargo run -- src
```

Example output:

```text
Path: src
Type: Directory
Size: 96 bytes
------------------------
main.rs
```

Directories are explored recursively, with indentation indicating nesting levels.

### ⚠️ A Note on Recursion Depth

**The current default is unlimited recursion (`max_depth = -1`).**

Running `rat` on a large directory, especially one containing `.git`, `target`, or `node_modules`, may produce an enormous amount of terminal output.

For now, the maximum recursion depth is configured directly in `src/main.rs`, rather than through a command-line option.

To limit the depth, locate the call to `print_directory_tree()` and change its final argument:

```rust
print_directory_tree(path, 0, -1) // Unlimited recursion
print_directory_tree(path, 0, 2)  // Limit recursion to 2 levels
```

The current depth convention is:

| Value | Behavior |
|---|---|
| `-1` | Unlimited recursion (current default) |
| `0` | Show immediate children only |
| `1` | Show immediate children and one additional level |
| `2` | Show immediate children and two additional levels |

**Tip:** Start with a small directory or set `max_depth` to `1` or `2` before exploring a large project.

A command-line option for controlling recursion depth may be added later.

### Handling errors

```bash
cargo run -- nonexistent.txt
```

```text
rat: nonexistent.txt: No such file or directory (os error 2)
```

`rat` returns a nonzero exit code when an operation fails.

## Philosophy

Keep it small. Keep it readable. Keep it Rusty.

This is a personal learning project, not an attempt to build the next great Unix utility.

The goal is to explore Rust through real filesystem operations, ownership, borrowing, error handling, iterators, and recursion.

A few principles guide the project:

- **Respect the bytes.** Never alter file contents, not even a single byte.
- **Let the path speak.** Files and directories should be handled according to what they are.
- **Stay simple.** Prefer the standard library and straightforward code over unnecessary abstractions.
- **Learn by building.** Understanding the implementation matters more than the feature count.

## Project Status

🚧 Early development.

Currently, `rat` accepts a single path argument and supports basic file inspection and recursive directory listing.

It's a small project, and that's the point.

---

*The cat has had its day. Now let the rat play.* 🐀