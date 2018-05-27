//! This module shows an example of code generated by the macro.
//! **IT MUST NOT BE USED OUTSIDE THIS CRATE.**

use std::string::String;

multi_structs! {
    /// This is the same `Merged` struct defined in
    /// the [crate level example](../index.html#example).
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