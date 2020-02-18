// vim: set expandtab ts=4 sw=4:
extern crate struct2hashmap;

use std::collections::HashMap;
use std::fmt;
use struct2hashmap::ToHashMap;

#[derive(Debug,ToHashMap)]
struct A {
    key: String,
}

impl fmt::Display for A {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.key)
    }
}

#[derive(Debug,ToHashMap)]
struct C {
    field1: usize,
    field2: f32,
    field3: u8,
    a: A,
}

fn main() {
    let c = C {
        field1: 1,
        field2: 2.2,
        field3: 0,
        a: A { key: "la".into() }, // struct will be converted to string via Display::fmt
    };
    println!("Struct C: {:?}", c.to_hashmap());
}
