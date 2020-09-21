use crate::ModeBits;

use std::{ffi::CString, io, path::Path};

/// Check if current process has permission to remove file/directory at
/// path.
///
/// To remove a file/directory in `Unix`, you'd need `W_OK` permission on
/// theparent directory, this function wraps the call of `access(parent_dir,
/// W_OK)`
///
/// Note that having the permission to remove a file does not guarantee that the
/// _I/O_ operation will be successful, it only means that it is very probably
/// to succeed. Be aware of [TOCTOU](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use)
/// race conditions, and any other `io::Error` that can occur.
pub fn is_file_removable(path: &impl AsRef<Path>) -> io::Result<bool> {
    let parent = match path.as_ref().parent() {
        // Cannot delete '/' (root)
        None => return Ok(false),
        Some(parent) => parent,
    };

    access(&parent, ModeBits::WRITE)
}

/// Check if current process has permission to read file/directory at path.
pub fn is_file_readable(path: &impl AsRef<Path>) -> io::Result<bool> {
    access(&path.as_ref(), ModeBits::READ)
}

/// Check if current process has permission to write to file/directory at path.
pub fn is_file_writable(path: &impl AsRef<Path>) -> io::Result<bool> {
    access(&path.as_ref(), ModeBits::WRITE)
}

/// Check if current process has permission to execute file/directory at path.
pub fn is_file_executable(path: &impl AsRef<Path>) -> io::Result<bool> {
    access(&path.as_ref(), ModeBits::EXECUTE)
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
pub fn access(path: &impl AsRef<Path>, modes: ModeBits) -> io::Result<bool> {
    // Let's translate ModeBits to the c_int
    let mut temp = 0;
    if modes.is_read_set() {
        temp |= libc::R_OK;
    }
    if modes.is_write_set() {
        temp |= libc::W_OK;
    }
    if modes.is_execute_set() {
        temp |= libc::X_OK;
    }
    // If nothing was set before, just check if the file exists with F_OK
    // See references at function doc for more explanation
    if temp == 0 {
        temp |= libc::F_OK;
    }

    let modes = temp;

    let bytes: Vec<u8> = path.as_ref().to_str().unwrap().bytes().collect();
    let cstring = CString::new(bytes).unwrap();
    let result = unsafe { libc::access(cstring.as_ptr(), modes) };

    if result == 0 {
        Ok(true) // Permission
    } else {
        assert!(result == -1);

        // From errno
        let err = io::Error::last_os_error();
        if err.raw_os_error().unwrap() == libc::EACCES {
            Ok(false) // No permission to delete
        } else {
            Err(err) // Could not check permission to delete, error
        }
    }
}
