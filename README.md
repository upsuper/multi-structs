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
    struct Merged {
        /// Foo
        #[derive(Debug)]
        foo: struct Foo {
            /// a
            a: i32,
            /// b
            b: i64,
        }
        /// Bar
        #[derive(Debug)]
        bar: struct Bar {
            /// c
            c: usize,
            /// d
            d: String,
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
