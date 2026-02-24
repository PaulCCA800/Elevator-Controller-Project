use std::fs::File;
use std::io::Write;
use std::os::unix::fs::FileExt;
use std::str;
use std::process::{Command, Stdio};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs;

pub fn generate_id () -> u64 {
    let id_string: String = fs::read_to_string("/etc/machine-id").expect("Could not find machine-id.");
    let mut id = id_string.as_str();
    id = id.trim();

    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    hasher.finish()
}

pub fn
write_to_file(path: &str, val: u8)
{    
    let mut file = File::create(path).unwrap();
    file.write_all(&[val]).unwrap();
    file.flush().unwrap();
}

pub fn
read_from_file(path: &str) -> u8
{
    let buf= &mut [0];
    let file = File::open(path).unwrap();
    match file.read_at(buf, 0)
    {
        Ok(_) => return buf[0],
        _ => return 0,
    }
}

pub fn
spawn_process()
{
    Command::new("kitty")
        .stdin (Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("--detach")
        .args  (&["cargo", "run"])
        .spawn ().unwrap();
}