// vim: set expandtab ts=4 sw=4:
extern crate struct2hashmap;

use std::collections::HashMap;
use struct2hashmap::ToHashMap;

#[derive(Debug,ToHashMap)]
struct C {
	field1: usize,
	field2: u8,
}
#[test]
fn to_hashmap() {
	let c = C {
		field1: 1,
		field2: 0,
	};
    let m = c.to_hashmap();
    assert_eq!(Some(&"1".to_string()), m.get("field1"));
    assert_eq!(Some(&"0".to_string()), m.get("field2"));
}
#[test]
fn to_hashmap_prefix() {
	let c = C {
		field1: 1,
		field2: 0,
	};
    let m = c.to_hashmap_with_prefix("pref.");
    assert_eq!(Some(&"1".to_string()), m.get("pref.field1"));
    assert_eq!(Some(&"0".to_string()), m.get("pref.field2"));
}
