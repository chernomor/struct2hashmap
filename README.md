Procedural macro `ToHashMap` to convert struct with named fields to `HashMap<String,String>`

Main purpose - convert numeric fields to map of strings. Other types will be converted to strings (it's must implementes Display trait).

```
$ cargo run --example main 
Struct C: {"field3": "0", "field2": "2.2", "field1": "1", "a": "la"}
```

Debug:
```
cargo install cargo-expand
cargo expand --example main
```
