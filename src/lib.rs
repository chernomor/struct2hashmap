// vim: set expandtab ts=4 sw=4:
use std::collections::HashMap;
pub use struct2hashmap_derive::ToHashMap;

pub trait ToHashMap {
    fn to_hashmap(&self) -> HashMap<String, String>;
    fn to_hashmap_with_prefix(&self, prefix: &str) -> HashMap<String, String>;
}

