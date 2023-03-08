Usage:

```rust
pub use console_decorate::prelude::*;

fn main() {
    println!(
        "{}",
        decorate!("YES!", BLUE, BOLD, UNDERLINE, ITALICS, STRIKETHROUGH)
    );
}
```
