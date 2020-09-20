use std::{ffi::CString, io, path::Path};

pub fn is_file_removable(path: impl AsRef<Path>) -> io::Result<bool> {
    let parent = match path.as_ref().parent() {
        Some(parent) => parent,
        None => return Ok(false), // Cannot delete '/' (root)
    };

    let bytes: Vec<u8> = parent.to_str().unwrap().bytes().collect();
    let cstring = CString::new(bytes).unwrap();
    let result = unsafe { libc::access(cstring.as_ptr(), libc::W_OK) };

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

// pub fn is_executable_by_someone(bits: bits_t) -> bool {
//     bits & 0o111 != 0
// }

// pub fn is_executable_by_someone(bits: bits_t) -> bool {
//     bits & 0o111 != 0
// }
