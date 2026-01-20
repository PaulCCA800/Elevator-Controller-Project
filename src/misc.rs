use std::fs::File;
use std::io::Write;
use std::os::unix::fs::FileExt;
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
    let _ = Command::new("kitty")
        .args(&["cargo", "run"])
        .spawn()
        .expect("Spawn failed, its so over");
}