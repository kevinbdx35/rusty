# rusty

## Cargo
Cargo is the package manager for Rust

```bash
cargo new
```

```bash
cargo build
```

```bash
cargo run
```

```bash
cargo clean
```

```bash
cargo check
```

```bash
cargo doc
```

## Comments
Line comments
```rust
// This is a line comment
```

Multiline comments are allowed but rarely used
```rust
/* This is not
very common */
```

Doc comments
```rust
/// This is mainly used to document funtionality
```
```rust
//! This is mainly used to document crates
```

## Markdown
Heading
```rust
//! # Main heading {...}
```

## Printing Values
```rust
println!("Hello World!");
```

Formatting
```rust
println!("My name is {} and I'm {}", "Kevin", 29);
```
The exclamation mark is very important. This symbolizes that this is as macro OK.

Expressions
```rust
println!("a + b = {}", 4+2);
```

Positional arguments
```rust
println!("{0} has a {2} and {0} has a {1}", "Kevin", "cat", "dog");
```