use std::fs::File;
use std::io::Write;
use std::process::Command;

use std::env;
use std::fs;

use std::thread;
use std::time::Duration;

const PATH: &str = "src/commands.txt";

fn main() {
    let mut count = 0;

    let mut is_host = 0;

    let args: Vec<String> = env::args().collect();
    let item = &args[1];
    if item == "help"
    {
        let _ = Command::new("kitty")
            .args(&["cargo", "run", "--", "else"])
            .spawn()
            .expect("Error Something Broke");
        is_host = 1;
    }
    
    loop
    {
        thread::sleep(Duration::from_secs(1));
        print_and_log(&mut count, &mut is_host);
        if count == 5
        {
            break;
        }
    }
}

fn
print_and_log(count: &mut u8, is_host: &mut i32)
{
    println!("{}", count);
    let val: u8 = count.clone();
    if *is_host == 1
    {
        write_to_file(PATH, val);
    }
    *count += 1;
}

fn
write_to_file(path: &str, val: u8)
{
    let mut f = File::create(path).unwrap();
    let val = val + 32;
    let _ = f.write_all(&[val.to_ascii_lowercase()]);
    let _ = f.flush();
}
