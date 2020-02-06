use std::collections::HashMap;
pub use struct2hashmap_derive::ToHashMap;

pub trait ToHashMap {
	fn to_hashmap(&self) -> HashMap<String, String>;
}

