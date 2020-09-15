// main just for testing
#[allow(unused_imports)]
use unix_file_permissions::{
    Classes::{AllClasses, Group, Other, Owner},
    ModeBits::{AllBits, Execute, Null, Read, Write},
    PermissionBits,
};

use std::{fs, io, os::unix::fs::PermissionsExt};

fn main() -> io::Result<()> {
    let a = fs::File::open("src/api.rs")?.metadata()?.permissions();
    let _b = PermissionBits::from(a.mode());
    let c = PermissionBits::blank().set(AllClasses, Read | Execute);

    println!("{:#?}", c);
    Ok(())
}
