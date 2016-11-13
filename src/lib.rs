//! Convertor of JSON text into gron format
//!
//! Gron is a representation that is easy to grep. Similarly to JSON
//! it can be easily evaluated with javascript interpreter.
//!
//! * [Original gron](https://github.com/tomnomnom/gron)
//! * [Documentation](https://tailhook.github.io/rust-gron/)
#![warn(missing_docs)]

extern crate rustc_serialize;
extern crate serde_json;

mod for_rustc_serialize;
mod for_serde;

use std::io::{self, Write};


/// An (internal) interface to gronify of json like data types
///
/// You probably should not implement it yourself, it's here just to unify
/// serde and rustc_serialize json structure.
pub trait ToGron {
    /// Write gron style representation of `self` to `out` with `prefix` in
    /// front of.
    ///
    /// Returns `()` when write to `out` was successful.
    fn to_gron<W: Write>(&self, out: &mut W, prefix: &str) -> io::Result<()>;
}

/// Converts JSON structure from into gron format.
///
/// It works both on `serde_json::Value` and on `rustc_serialize::json::Json`
///
/// # Example
///
/// ```
/// extern crate gron;
/// extern crate serde_json;
///
/// use std::io::stdout;
/// use serde_json::value::Value;
/// use serde_json::de;
/// use gron::json_to_gron;
///
/// # fn main() {
/// let json: Value = de::from_str(r#"{"x": [1,2]}"#).unwrap();
/// json_to_gron(&mut stdout(), "val", &json);
/// // Outputs to stdout:
/// //
/// //   val = {}
/// //   val.x = []
/// //   val.x[0] = 1
/// //   val.x[1] = 2
/// # }
///
/// ```
pub fn json_to_gron<W: Write, T: ToGron>(out: &mut W, prefix: &str, json: &T)
    -> io::Result<()>
{
    json.to_gron(out, prefix)
}


#[cfg(test)]
mod test {
    use serde_json::de;
    use serde_json::value::Value;

    use rustc_serialize::json::Json;

    use ToGron;

    fn assert_equal(json_src: &str, gron: &str) {
        let mut serde_buf = Vec::new();
        let serde_json = &de::from_str::<Value>(json_src).unwrap();
        assert_eq!(serde_json.to_gron(&mut serde_buf, "json").is_ok(), true);
        assert_eq!(String::from_utf8(serde_buf).unwrap(), gron);

        let mut rustc_serialize_buf = Vec::new();
        let rustc_serialize_json = &Json::from_str(json_src).unwrap();
        assert_eq!(rustc_serialize_json.to_gron(&mut rustc_serialize_buf, "json").is_ok(), true);
        assert_eq!(String::from_utf8(rustc_serialize_buf).unwrap(), gron);
    }

    #[test]
    fn test_simple() {
        assert_equal(r#""x""#, "json = \"x\"\n");
        assert_equal(r#"1"#, "json = 1\n");
        assert_equal(r#"-1"#, "json = -1\n");
        assert_equal(r#"1.5"#, "json = 1.5\n");
        assert_equal(r#"null"#, "json = null\n");
        assert_equal(r#"true"#, "json = true\n");
    }

    #[test]
    fn test_map() {
        assert_equal(
            r#"{"x": 1, "y": 2}"#, "\
            json = {}\n\
            json.x = 1\n\
            json.y = 2\n");
    }

    #[test]
    fn test_vec() {
        assert_equal(
            r#"[1, 2, 3]"#, "\
            json = []\n\
            json[0] = 1\n\
            json[1] = 2\n\
            json[2] = 3\n\
            ");
    }

    #[test]
    fn test_obj_in_list() {
        assert_equal(
            r#"[1, {"x": 1, "y": 2}, 3]"#, "\
            json = []\n\
            json[0] = 1\n\
            json[1] = {}\n\
            json[1].x = 1\n\
            json[1].y = 2\n\
            json[2] = 3\n\
            ");
    }

    #[test]
    fn test_list_in_obj() {
        assert_equal(
            r#"{"a": 1, "x": [7, 8], "y": 2}"#, "\
            json = {}\n\
            json.a = 1\n\
            json.x = []\n\
            json.x[0] = 7\n\
            json.x[1] = 8\n\
            json.y = 2\n\
            ");
    }
}

