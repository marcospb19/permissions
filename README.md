# permissions

This crate provide a friendly interface for permissions of the current
process.

`is_file_removable`, and any `is_file` function, checks if the current has
the permissions required to conclude the operation.

Functions available:
```rust
is_file_executable
is_file_readable
is_file_writable
is_file_removable
```

## Example:
```rust
use permissions::is_file_removable;

use std::{fs, io, path::PathBuf};

fn main() -> io::Result<()> {
    let path = "path";
    fs::File::create(path)?;

    if is_file_removable(path)? {
        fs::remove_file(path)?;
    }

    Ok(())
}
```

---

This crate also offers ModeBits for friendly `rwx` permissions, yet to be
documented
