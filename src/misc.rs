use std::fs::File;
use std::io::Write;
use std::str;

use std::process::Command;

pub fn
write_to_file(path: &str, val: u8)
{    
    let mut file = File::create(path).unwrap();
    file.write_all(&[val]).unwrap();
    file.flush().unwrap();
}

pub fn
start_backup()
{
    let _ = Command::new("kitty")
        .args(&["cargo", "run"])
        .spawn()
        .expect("Spawn failed, its so over");
}