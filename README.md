# Rust

## Just programs that I wrote to start learning Rust

Not my favorite language, but whatever. (I hate the syntax and it has quirks that I don't like which I think Golang does a lot better)

### Hello world:

Your typical "Hello world!" program.

To run:

```bash
cargo new hello-world
cd hello-world
cargo run
```

To build:

```bash
cargo build --release
cargo run --release
```

### Ferris says:

I guess the mascot, but Rust version of cowsay (cowsay is superior, but whatever).

To create project:

```bash
cargo new ferris-say
cd ferris-say
```

To install the library:

```bash
cargo add ferris-says
```

This would add the library to the Cargo.toml and I prefer this method over adding manually.

The code:

```rust
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello, I am Ferris the Rustacean!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
```

Example straight out of the docs, but I guess the message is referencing (borrowing). If you know anything about pointers, this makes sense what I am trying to write here. If not, that is fine. Pointers are a bit confusing and I still struggle with pointers because I suck at understanding memory.

To run:

```bash
cargo run
```

To build:

```bash
cargo build --release
cargo run --release
```