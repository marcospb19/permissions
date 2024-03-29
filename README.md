# NOTE

If you want Windows support, check [faccess] instead.

# permissions

[![Crates.io](https://img.shields.io/crates/v/permissions.svg)](https://crates.io/crates/permissions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/marcospb19/permissions/blob/main/LICENSE)
[![Docs.rs](https://docs.rs/permissions/badge.svg)](https://docs.rs/permissions)

Useful filesystem queries for Unix  file permissions:

See [`functions`](https://docs.rs/permissions/latest/permissions/functions/index.html).
- [`is_executable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_executable.html)
- [`is_readable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_readable.html)
- [`is_writable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_writable.html)
- [`is_removable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_removable.html)
- [`is_creatable`](https://docs.rs/permissions/latest/permissions/functions/fn.is_creatable.html)
- [`access_syscall`](https://docs.rs/permissions/latest/permissions/functions/fn.access_syscall.html) supports a custom bitfield of permissions.

## Examples:
```rust
use permissions::*;

fn main() -> std::io::Result<()> {
   // Functions accept `AsRef<Path>`
   assert!(is_readable("src/")?);
   assert!(is_writable("src/")?);
   assert!(is_writable("src/lib.rs")?);
   assert!(is_executable("/usr/bin/cat")?);
   assert!(is_removable("src/lib.rs")?);
   assert!(is_creatable("src/file.rs")?);

   Ok(())
}
```

# Alternatives
- [faccess] - it provides Windows support and a trait-based API.

[faccess]: https://github.com/Freaky/faccess
