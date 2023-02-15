use std::{fs, os::unix::prelude::PermissionsExt};
use std::env;

fn num_to_per(perm: u32) -> String {
    let mut perms: Vec<char> = vec![];
    let mut perm = perm & 511;
    let mut count: i32 = 0;
    while perm > 0 {
        if perm & 1 == 1 {
            match count % 3 {
                0 => perms.push('x'),
                1 => perms.push('w'),
                _ => perms.push('r')
            };
        } else {
            perms.push('-');
        }
        count = count + 1;
        perm = perm >> 1;
    }
    perms.iter().cloned().rev().collect::<String>()
}
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let root_dir = match args.len() {
        1 => String::from("."),
        _ => args[1].clone(),
    };
    let paths = fs::read_dir(root_dir)?;
    for path in paths {
        let file = path?;
        let metadata = file.metadata()?;
        let permissions = metadata.permissions();
        let name = file.file_name();
        if metadata.is_dir() {
            println!("d{} {}", num_to_per(permissions.mode()), name.to_string_lossy());
        } else {
            println!("-{} {}", num_to_per(permissions.mode()), name.to_string_lossy());
        }
    }
    Ok(())
}
