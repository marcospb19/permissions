//! Permission checks at some path using [`access_syscall`].
//!
//! Having permission of reading, writing, executing or deleting a file does not
//! guarantee success in doing so, it is unlikely but IO can fail.
//!
//! Also be careful with [`TOCTOU race conditions`], when you have outdated file
//! system information that has changed since the last check.
//!
//! [`TOCTOU race conditions`]: https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use

use std::{
    ffi::CString,
    io,
    os::{raw::c_int, unix::prelude::OsStrExt},
    path::Path,
};

/// Bitflags definitions used in the `access_syscall` bitfield.
#[cfg(unix)]
pub mod consts {
    /// File exists.
    pub const F_OK: libc::c_int = libc::F_OK;
    /// Read permission.
    pub const R_OK: libc::c_int = libc::R_OK;
    /// Write permission.
    pub const W_OK: libc::c_int = libc::W_OK;
    /// Execute permission.
    pub const X_OK: libc::c_int = libc::X_OK;
}

/// Check if current process has permission to remove file.
///
/// That is, if the current process has permission of `write` to the parent
/// directory.
///
/// Returns `false` if there's no parent directory (because can't delete the root directory).
///
/// This is the same as calling [`is_creatable`].
///
/// # Errors
///
/// - If [`Path::canonicalize`] fails.
/// - Same as [`access_syscall`].
///
/// # Examples
/// ```
/// use permissions::is_removable;
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     println!("{}", is_removable("src/lib.rs")?);
///     println!("{}", is_removable("/root")?);
///     println!("{}", is_removable("/")?);
///
///     // May return `Err(kind: PermissionDenied)`
///     // println!("{}", is_removable("/root/any")?);
///
///     Ok(())
/// }
/// ```
pub fn is_removable(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().canonicalize()?;
    let parent = match path.parent() {
        // Cannot delete '/' (root directory)
        None => return Ok(false),
        Some(parent) => parent,
    };
    access_syscall(&parent, consts::W_OK)
}

/// Check if current process has permission to create file.
///
/// That is, if the current process has permission of `write` to the parent
/// directory.
///
/// Returns `false` if there's no parent directory (because you can't create the root directory).
///
/// This is the same as calling [`is_removable`].
///
/// # Errors
///
/// - If [`Path::canonicalize`] fails.
/// - Same as [`access_syscall`].
///
/// # Examples
/// ```
/// use permissions::is_creatable;
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     println!("{}", is_creatable("src/lib.rs")?);
///     println!("{}", is_creatable("/root")?);
///     println!("{}", is_creatable("/")?);
///
///     // May return `Err(kind: PermissionDenied)`
///     // println!("{}", is_creatable("/root/any")?);
///
///     Ok(())
/// }
/// ```
pub fn is_creatable(path: impl AsRef<Path>) -> io::Result<bool> {
    is_removable(path.as_ref())
}

/// Check if current process has permission to read.
///
/// # Errors
/// Same as [`access_syscall`].
///
/// # Examples
/// ```
/// use permissions::is_readable;
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     println!("{}", is_readable("src/lib.rs")?);
///     println!("{}", is_readable("/root")?);
///     println!("{}", is_readable("/")?);
///
///     // may return `Err(kind: PermissionDenied)`
///     // println!("{}", is_readable("/root/any")?);
///
///     Ok(())
/// }
/// ```
pub fn is_readable(path: impl AsRef<Path>) -> io::Result<bool> {
    access_syscall(&path.as_ref(), consts::R_OK)
}

/// Check if current process has permission to write.
///
/// # Errors
/// Same as [`access_syscall`].
///
/// # Examples
/// ```
/// use permissions::is_writable;
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     println!("{}", is_writable("src/lib.rs")?);
///     println!("{}", is_writable("/root")?);
///     println!("{}", is_writable("/")?);
///
///     // may return `Err(kind: PermissionDenied)`
///     // println!("{}", is_writable("/root/any")?);
///
///     Ok(())
/// }
/// ```
pub fn is_writable(path: impl AsRef<Path>) -> io::Result<bool> {
    access_syscall(&path.as_ref(), consts::W_OK)
}

/// Check if current process has permission to execute.
///
/// If `path` points to a directory, you'll be checking if you have the right to
/// enter it.
///
/// # Errors
/// Same as [`access_syscall`].
///
/// # Examples
/// ```
/// use permissions::is_executable;
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     assert!(is_executable("/usr/bin/cat")?);
///     assert!(is_executable("/")?);
///     assert!(is_executable("src/")?);
///     assert!(!is_executable("src/lib.rs")?);
///     assert!(!is_executable("/root")?);
///
///     // may return `Err(kind: PermissionDenied)`
///     // println!("{}", is_executable("/root/any")?);
///
///     Ok(())
/// }
/// ```
#[cfg(unix)]
pub fn is_executable(path: impl AsRef<Path>) -> io::Result<bool> {
    access_syscall(&path.as_ref(), consts::X_OK)
}

/// Safe wrapper to the `libc::access` syscall.
///
/// See [`access man page`].
///
/// Used by:
/// - [`is_removable`]
/// - [`is_readable`]
/// - [`is_writable`]
/// - [`is_executable`]
///
/// This function requires a bitmask made of:
/// - [`consts::R_OK`] _(Read)_
/// - [`consts::W_OK`] _(Write)_
/// - [`consts::X_OK`] _(Execute)_
///
/// To check for each given `rwx` permission, or:
/// - [`consts::F_OK`] _(File exists)_
///
/// Otherwise, the function fails with [`Err(kind::InvalidInput)`](std::io::ErrorKind::InvalidInput)
///
/// # Examples:
///
/// ```
/// use permissions::access_syscall;
/// use permissions::consts::{R_OK, W_OK, X_OK, F_OK};
///
/// fn main() -> std::io::Result<()> {
///     assert!(access_syscall("src/lib.rs", R_OK | W_OK)?);
///     assert!(access_syscall("/", R_OK | X_OK)?);
///     assert!(access_syscall(".", F_OK)?);
///
///     assert!(!access_syscall("src/lib.rs", X_OK)?);
///     assert!(!access_syscall("/root", W_OK)?);
///
///     Ok(())
/// }
/// ```
///
/// # Errors
/// See [`access man page`].
///
/// [`io::Error`]: std::io::Error
/// [`access man page`]: https://man7.org/linux/man-pages/man2/access.2.html
pub fn access_syscall(path: impl AsRef<Path>, mode_mask: c_int) -> io::Result<bool> {
    let path = path.as_ref();

    let cstr = CString::new(path.as_os_str().as_bytes()).expect("Path had interior nul byte");

    // Safety:
    //   Pointee is a valid null-terminated string.
    let access_return_code = unsafe { libc::access(cstr.as_ptr(), mode_mask) };

    match access_return_code {
        0 => Ok(true),
        _ => {
            let err = io::Error::last_os_error();
            if err.raw_os_error().unwrap() == libc::EACCES {
                Ok(false) // Ok, no permission to delete
            } else {
                Err(err) // Syscall error
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use consts::{F_OK, R_OK, W_OK, X_OK};

    use super::*;

    #[test]
    fn test_access_syscall() {
        assert!(access_syscall("src/", F_OK | R_OK | W_OK | X_OK).unwrap());
        assert!(access_syscall("src/", F_OK).unwrap());
        assert!(access_syscall("src/", R_OK).unwrap());
        assert!(access_syscall("src/", W_OK).unwrap());
        assert!(access_syscall("src/", X_OK).unwrap());

        assert!(!access_syscall("src/lib.rs", X_OK).unwrap());
        assert!(!access_syscall("src/lib.rs", X_OK | R_OK).unwrap());
        assert!(!access_syscall("src/lib.rs", X_OK | W_OK).unwrap());
        assert!(!access_syscall("src/lib.rs", X_OK | F_OK).unwrap());

        assert!(access_syscall("path_doesnt_exist/", F_OK).is_err()); // invalid path
        assert!(access_syscall("src/", 0b11111).is_err()); // invalid number

        assert!(!access_syscall("src/lib.rs", X_OK).unwrap());
        assert!(!is_removable("/").unwrap());
    }
}
