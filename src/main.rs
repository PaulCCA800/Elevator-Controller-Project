use std::process::Command;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let item = &args[1];
    if item == "help"
    {
        let _ = Command::new("kitty")
            .args(&["cargo", "run", "--", "else"])
            .spawn()
            .expect("Error Something Broke");
    }
    
    loop
    {
        println!("Test");
    }
}
