## ArcticPalette :snowman:

A straightforward, lightweight version of clap, best for building a minimal and small CLI-like programs. [Available on Crates.io](https://crates.io/creates/arcticpalette)

```rust
use arcticpalette::{ArcticPalette, Command}

fn on_fallback(cmd: Command) {
  println!("Unknown command: {}", cmd.get_cmd());
}

fn on_greet(cmd: Command) {
  let name = cmd.join_args();
  println!("Hello, {}", name);
}

fn main() {
  let arctic = ArcticPalette::new();
  arctic.assign_fallback(on_fallback);
  arctic.assign("greet", on_greet);
  // TIP:
  // It is recommended to integrate arctic.apply(...);
  // with io::stdin().read_line(...);
  arctic.apply("greet John Doe");
  arctic.apply("notfound");
}
```

## Installation

To install ArcticPalette on your terminal, use `cargo add`.

```
cargo add arcticpalette
```

Moreover, to install ArcticPalette as a dependency, add the following code on you `Cargo.toml`:

```toml
[dependencies]
arcticpalette = "0.1.0"
```
