use crate::ModeBits;

use std::{io, os::raw::c_int, path::Path};

/// Check if current process has permission to remove file/directory at
/// path.
///
/// That is, if the current process has permission of `write` to the parent
/// directory.
///
/// Note that having the permission to remove a file does not guarantee that the
/// removal operation will be successful, it only means that it is very probably
/// to succeed. Be aware of [TOCTOU](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use)
/// race conditions, and any other `io::Error` that can occur.
pub fn is_file_removable(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref().canonicalize()?;
    let parent = match path.parent() {
        // Cannot delete '/' (root)
        None => return Ok(false),
        Some(parent) => parent,
    };

    access(&parent, ModeBits::WRITE)
}

/// Check if current process has permission to read file/directory at path.
///
/// Err() can be returned if there's no read permission in parent directories of
/// `path`.
pub fn is_file_readable(path: impl AsRef<Path>) -> io::Result<bool> {
    access(&path.as_ref(), ModeBits::READ)
}

/// Check if current process has permission to write to file/directory at path.
///
/// Err() can be returned if there's no read permission in parent directories of
/// `path`.
pub fn is_file_writable(path: impl AsRef<Path>) -> io::Result<bool> {
    access(&path.as_ref(), ModeBits::WRITE)
}

/// Check if current process has permission to execute file/directory at path.
///
/// Err() can be returned if there's no read permission in parent directories of
/// `path`.
pub fn is_file_executable(path: impl AsRef<Path>) -> io::Result<bool> {
    access(&path.as_ref(), ModeBits::EXECUTE)
}

/// Safe function that wraps `libc::access` syscall and uses ModeBits.
///
/// `mode` is a mask consisting of the following `ModeBits` flags:
///
/// `READ`
/// `WRITE`
/// `EXECUTE`
////
/// if `mode == ModeBits::Null`, this function only checks if the file exists.
///
/// See access_syscall.
pub fn access(path: impl AsRef<Path>, modes: ModeBits) -> io::Result<bool> {
    // Translating ModeBits to c_int
    let mut mode_mask = 0;
    if modes.is_read_set() {
        mode_mask |= libc::R_OK;
    }
    if modes.is_write_set() {
        mode_mask |= libc::W_OK;
    }
    if modes.is_execute_set() {
        mode_mask |= libc::X_OK;
    }
    if mode_mask == 0 {
        mode_mask |= libc::F_OK;
    }

    access_syscall(path, mode_mask)
}

/// Safe function that wraps `libc::access` syscall.
///
/// `mode: ModeBits` can be a mask consisting of the bitwise _OR_ of one or more
/// of `R_OK`, `W_OK`, and `X_OK`.
///
/// If `mode` is an empty mask, this function passes `F_OK` and only checks if
/// the file exists.
///
/// References:
/// `man access`,
/// `man 2 access`, or
/// [online man page](https://man7.org/linux/man-pages/man2/access.2.html)
pub fn access_syscall(path: impl AsRef<Path>, mode_mask: c_int) -> io::Result<bool> {
    let path = path.as_ref();

    // https://stackoverflow.com/a/59224987/9982477
    let mut buf = Vec::new();
    let buf_ptr;
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        buf.extend(path.as_os_str().as_bytes());
        buf.push(0u8);
        buf_ptr = buf.as_ptr() as *const libc::c_char;
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        buf.extend(
            path.as_os_str()
                .encode_wide()
                .chain(Some(0u16))
                .map(|b| {
                    let b = b.to_ne_bytes();
                    b.get(0).map(|s| *s).into_iter().chain(b.get(1).map(|s| *s))
                })
                .flatten(),
        );
        buf_ptr = buf.as_ptr() as *const libc::c_wchar_t;
    }

    let result = unsafe { libc::access(buf_ptr, mode_mask) };

    if result == 0 {
        Ok(true)
    } else {
        assert!(result == -1);
        let err = io::Error::last_os_error();
        if err.raw_os_error().unwrap() == libc::EACCES {
            Ok(false) // No permission to delete
        } else {
            Err(err) // Could not check permission to delete, error
        }
    }
}
