# user/bin/initcode.rs
```rust
// user/bin/initcode.rs
#![no_std]
use ulib::sys;

static INIT: &str = "/init";
static ARGV: [&str; 1] = ["init"];

fn main() -> sys::Result<()> {
    sys::exec(INIT, &ARGV, None)?;
    sys::exit(0)
}

```
