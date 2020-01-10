//! A macro for generating a merged struct from multiple sub-structs.
//!
//! # Example
//!
//! ```
//! use multi_structs::multi_structs;
//!
//! multi_structs! {
//!     /// The merged struct.
//!     #[derive(Debug)]
//!     struct Merged {
//!         /// Foo
//!         #[derive(Debug)]
//!         foo: struct Foo {
//!             /// a
//!             a: i32,
//!             /// b
//!             b: i64,
//!         }
//!         /// Bar
//!         #[derive(Debug)]
//!         bar: struct Bar {
//!             /// c
//!             c: usize,
//!             /// d
//!             d: String,
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let foo = Foo { a: 1, b: 2 };
//!     let bar = Bar { c: 3, d: "aaa".to_string() };
//!     println!("{:?}, {:?}", foo, bar);
//!     let merged = Merged::new(foo, bar);
//!     println!("{:?}", merged);
//!     let (foo, bar) = merged.split();
//!     println!("{:?}, {:?}", foo, bar);
//! }
//! ```
//!
//! See [`example_generated`](./example_generated/index.html) for
//! documentation of code generated by the above `multi_structs!`
//! expansion.
//!
//! # Visibility
//!
//! All structs and fields involved are currently `pub`. This will
//! likely gets changed in the future.
//!
//! # Attributes
//!
//! Attributes can be attached to any struct and field involved.
//!
//! # Methods
//!
//! The following methods are defined for the merged struct:
//!
//! - `new`: create the new merged struct from multiple sub-structs.
//! - `split`: split the merged struct into the sub-structs.

#![no_std]

#[cfg(feature = "example_generated")]
extern crate std;

#[macro_export]
macro_rules! multi_structs {
    {
        $(#[$($meta:tt)+])*
        struct $name:ident {
            $(
                $(#[$($sub_meta:tt)+])*
                $var:ident: struct $sub:ident {
                    $(
                        $(#[$($field_meta:tt)+])*
                        $field:ident: $ty:ty,
                    )+
                }
            )+
        }
    } => {
        $(
            $(#[$($sub_meta)+])*
            pub struct $sub {
                $(
                    $(#[$($field_meta)+])*
                    pub $field: $ty,
                )+
            }
        )+

        $(#[$($meta)+])*
        pub struct $name {
            $(
                $(
                    $(#[$($field_meta)+])*
                    pub $field: $ty,
                )+
            )+
        }

        impl $name {
            /// Create $name.
            pub fn new(
                $($var: $sub,)+
            ) -> Self {
                Self {
                    $(
                        $($field: $var.$field,)+
                    )+
                }
            }

            /// Split $name.
            pub fn split(self) -> ($($sub,)+) {
                (
                    $(
                        $sub {
                            $($field: self.$field,)+
                        },
                    )+
                )
            }
        }
    }
}

#[cfg(feature = "example_generated")]
pub mod example_generated;
