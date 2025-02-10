# user/bin/echo.rs
```rust
// user/bin/echo.rs
#![no_std]
use ulib::{env, print, println};

fn main() {
    let args = env::args();
    for arg in args.skip(1) {
        print!("{} ", arg);
    }
    println!("");
}

```
