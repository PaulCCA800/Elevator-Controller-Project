use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};

pub fn generate_id_from_machine_id () -> u16{
    let id_string: String = fs::read_to_string("/etc/machine-id").expect("Could not find machine-id.");
    let mut id = id_string.as_str();
    id = id.trim();

    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    let hashed = hasher.finish();
    (hashed ^ (hashed >> 16) ^ (hashed >> 32) ^ (hashed >> 48)) as u16
}

pub fn generate_id () -> u16 {
    let args: Vec<String> = env::args().collect();

    match args.get(1){
        Some(arg) => match arg.parse::<u16>(){
            Ok(id) => id,
            Err(_) => generate_id_from_machine_id(),
        }
        None => generate_id_from_machine_id(),
    }
}



