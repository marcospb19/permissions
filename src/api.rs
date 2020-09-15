// This is the basic API we are trying to achieve!
// Simbora cara, vai dar certo

// Tem mais coisa além disso aqui, na verdade, aqui tá o básico da ideia

/*
Possible changes here

Should I change:

ModeBits to Bits?
ModeBits::AllSet to {AllModes or AllBits}?
Classes::AllClasses to {AllClasses or Any or All}?
*/

// Importando os cabra bom tudo exibido
use unix_file_permissions::{
    Classes::{AllClasses, Group, Other, Owner},
    ModeBits::{AllSet, Execute, Null, Read, Write},
    PermissionBits,
};

// use std::fs::File;

// fn main() -> std::io::Result<()> {
//     let file = File::open("foo.txt")?;
//     // Troca isso por um wrapper
//     // let mut perms = file.metadata()?.permissions();
//     let permissions = PermissionBits::from_file(file)?;

//     if permissions.get(Owner, Execute) {
//         println!("Owner of file can execute");
//     }

//     if permissions.get(Other, Read | Write) {
//         println!("Other users can read and write to this file!");
//     }

//     // Let's change permissions with set and unset
//     let permissions = permissions
//         .unset(Owner, Execute)
//         .set(Group, Read | Write)
//         .reset(Other, Read);

//     perms.set_readonly(true);
//     file.set_permissions(perms)?;
//     Ok(())
// }

// Pending
// is set_readonly() == .unset(AllClasses, Write) ?????
