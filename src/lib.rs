//! Convertor of JSON text into gron format
//!
//! Gron is a representation that is easy to grep. Similarly to JSON
//! it can be easily evaluated with javascript interpreter.
//!
//! * [Original gron](https://github.com/tomnomnom/gron)
//! * [Documentation](https://tailhook.github.io/gron/gron/index.html)

extern crate rustc_serialize;

mod json_struct;

pub use json_struct::json_to_gron;
