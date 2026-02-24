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