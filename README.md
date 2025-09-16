# Rust

## Just programs that I wrote to start learning Rust

Not my favorite language, but whatever. (I hate the syntax and it has quirks that I don't like which I think Golang does a lot better)

## Notes:

This is just general notes like getting rust-analyzer to work properly in my VS Codium (VS Code).

Have to create a Cargo.toml in folders like for example in the folder "Basics":

```bash
cd Basics/
vim Cargo.toml
...
[workspace]
members = [
  "ferris-say",
  "guess-rand",
  "hello-world"
]

resolver = "3"
```

This I guess is how workspaces work with Cargo. I am not exactly sure what this is for, but will probably learn when I code more Rust.

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

### Guess random number:

This was a bit of a doozy, but wanted to play around with functions, return, if-else, etc.

Just randomly generates a number from 1 to 101 and prints it, but you gotta guess. Printing to STDOUT is to make sure this program works. After you type in a number or letter, it checks to see if valid or not. If isn't a valid input, it does throw an error and gracefully returns/exits.

I realized that `.expect()` just panicks which is unacceptable and wanted to handle errors/exceptions properly.

Here is an example in `main()`:

```rust
    // Try parsing input.
    // Handle invalid input gracefully.
    guess = match input.trim().to_string().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return; // Exit early without panicking
        }
    };
```

Probably will grow on me and looks clean enough.

To create:

```bash
cargo new guess-rand
```

Install the rand library:

```bash
cargo add rand
```

To run:

```bash
cargo run
```

To build:

```bash
cargo build --release
cargo run --release
```
