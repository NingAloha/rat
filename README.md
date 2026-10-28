# rat 🐀

[简体中文](README.zh-CN.md)

**A little Rusty rat chasing a C cat.**

`rat` is a tiny command-line utility written in Rust for exploring files and directories.

It began as a simple exercise in reimplementing `cat`, but evolved into something a little different: a tool that automatically recognizes whether a path refers to a file or a directory and displays it accordingly.

One Rust source file. Zero external dependencies. Just a little rat doing its thing.

## Why rat?

The name is a small piece of wordplay:

- **cat** is traditionally written in **C**.
- **rat** is written in **Rust**.
- Cats chase rats. This one chases `cat`.

One letter changes, one language changes, and the mouse gets its turn.

`rat` borrows a few ideas from `cat`, `stat`, and `tree`, but isn't intended to replace any of them. It's a personal Rust learning project built around a simple idea:

**Give it a path, and let the rat figure out what to do.**

## Features

### File Inspection

- Display the path, type, and size.
- Print file contents byte-for-byte.
- Preserve original whitespace, encoding, and trailing newlines (or their absence).
- Report file access and I/O errors.

### Directory Exploration

- Automatically recognize directories.
- Traverse subdirectories recursively.
- Sort entries alphabetically by filename.
- Display directory hierarchy using indentation.
- Support a configurable maximum recursion depth in the source code.
- Skip unwanted files and directories using embedded ignore rules.

### Under the Hood

- Written in Rust (Edition 2024).
- Zero external dependencies.
- A single Rust source file.
- Uses Rust's standard filesystem and I/O APIs.
- Error handling with `Result`, `?`, and exit codes.

## Getting Started

### Requirements

Rust and Cargo.

Clone the repository and build:

```bash
git clone https://github.com/NingAloha/rat
cd rat
cargo build --release
```

The executable will be available at:

```text
target/release/rat
```

You can also run it directly with Cargo:

```bash
cargo run -- <path>
```

## Usage

### Inspect a File

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

File contents are copied directly to standard output without adding, removing, or modifying any bytes.

### Explore a Directory

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

For directories containing subdirectories, `rat` uses indentation to show their structure.

```text
project
    docs
        guide.md
    src
        main.rs
        utils.rs
    Cargo.toml
```

### ⚠️ Recursion Depth

**Directory traversal is currently unlimited by default (`max_depth = -1`).**

Large directories may produce enormous amounts of terminal output.

To change this behavior, edit the `print_directory_tree()` call in `src/main.rs`:

```rust
print_directory_tree(path, 0, -1)?;
```

Change the final argument to limit recursion:

| Value | Behavior |
|---|---|
| `-1` | Unlimited recursion (default) |
| `0` | Immediate children only |
| `1` | Immediate children and one additional level |
| `2` | Immediate children and two additional levels |

For example:

```rust
print_directory_tree(path, 0, 2)?;
```

Rebuild after changing the value.

**Tip:** If the rat gets carried away and floods your terminal, press `Ctrl + C` to stop it.

## What the Rat Doesn't Like

Rats can be picky.

`rat` has its own ignore list, stored in `.ratdontlike` at the project root.

The current list is:

```text
.git
.idea
target
.DS_Store
```

When exploring directories, `rat` automatically skips entries with these exact names, including their contents if they are directories.

### How It Works

The list is embedded directly into the executable at compile time:

```rust
const DISLIKES: &str = include_str!("../.ratdontlike");
```

This means:

- No runtime configuration file is required.
- Ignore rules work regardless of the current working directory.
- Rules are matched against entry names at every directory level.
- Empty lines and lines beginning with `#` are ignored.
- Matching uses exact names, not wildcard patterns.
- Modifying `.ratdontlike` requires rebuilding the executable.

For example:

```bash
rat .
```

will skip `.git`, `.idea`, `target`, and `.DS_Store` entries while traversing the directory.

However, explicitly requested paths are still accessible:

```bash
rat .git
```

The rat respects direct instructions. It just avoids things it doesn't like while exploring.

**Note:** The `.ratdontlike` source file is required when building, but not when running the compiled executable.

## Error Handling

If a path does not exist:

```bash
cargo run -- nonexistent.txt
```

Example output:

```text
rat: nonexistent.txt: No such file or directory (os error 2)
```

`rat` reports errors to standard error and returns a nonzero exit code when an operation fails.

Currently, the command accepts exactly one path argument.

## Philosophy

**Keep it small. Keep it readable. Keep it Rusty.**

`rat` is a personal learning project, not an attempt to build the next great Unix utility.

Its guiding principles are simple:

- **Respect the bytes.** Never change file contents, not even a single byte.
- **Let the path speak.** Handle files and directories according to their types.
- **Stay lightweight.** Prefer Rust's standard library over unnecessary dependencies.
- **Make it personal.** The tool should fit its author's workflow.
- **Learn by building.** Understanding the code matters more than adding features.

## Project Status

🚧 Early development.

Currently implemented:

- Single-path inspection
- File metadata and content output
- Recursive directory exploration
- Configurable recursion depth in source code
- Alphabetical directory listing
- Compile-time embedded ignore rules
- I/O error handling and exit codes

There is no formal release schedule or roadmap. Features are added whenever something seems interesting or useful.

It's a small project, and that's the point.

---

*The cat has had its day. Now let the rat play.* 🐀
