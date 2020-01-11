# multi-structs

[![Build Status](https://travis-ci.org/upsuper/multi-structs.svg?branch=master)](https://travis-ci.org/upsuper/multi-structs)
[![Docs](https://docs.rs/multi-structs/badge.svg)](https://docs.rs/multi-structs)

A macro for generating a merged struct from multiple sub-structs.

## Example

```rust
#[macro_use]
extern crate multi_structs;

multi_structs! {
    /// The merged struct.
    #[derive(Debug)]
    pub struct Merged {
        /// Foo
        #[derive(Debug)]
        pub foo: struct Foo {
            /// a
            pub a: i32,
            /// b
            pub b: i64,
        }
        /// Bar
        #[derive(Debug)]
        pub bar: struct Bar {
            /// c
            pub c: usize,
            /// d
            pub d: String,
        }
    }
}

fn main() {
    let foo = Foo { a: 1, b: 2 };
    let bar = Bar { c: 3, d: "aaa".to_string() };
    println!("{:?}, {:?}", foo, bar);
    let merged = Merged::new(foo, bar);
    println!("{:?}", merged);
    let (foo, bar) = merged.split();
    println!("{:?}, {:?}", foo, bar);
}
```
