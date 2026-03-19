use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs;

pub fn generate_id () -> u16 {
    let id_string: String = fs::read_to_string("/etc/machine-id").expect("Could not find machine-id.");
    let mut id = id_string.as_str();
    id = id.trim();

    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    let hashed = hasher.finish();
    return (hashed ^ (hashed >> 16) ^(hashed >> 32) ^(hashed >> 48)) as u16
}

