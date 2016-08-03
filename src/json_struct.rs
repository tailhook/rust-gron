use std::io::{self, Write};
use std::iter::Enumerate;
use std::collections::btree_map::Iter as MapIter;
use std::slice::Iter as VecIter;

use rustc_serialize::json::Json;


enum StackItem<'a> {
    MapIter(MapIter<'a, String, Json>),
    VecIter(Enumerate<VecIter<'a, Json>>),
}


/// Converts JSON structure from `rustc_serialize::json::Json` into gron format
///
/// # Example
///
/// ```
/// # extern crate gron;
/// # extern crate rustc_serialize;
/// #
/// # use std::io::stdout;
/// # use rustc_serialize::json::Json;
/// # use gron::json_to_gron;
/// #
/// # fn main() {
/// let json = Json::from_str(r#"{"x": [1,2]}"#).unwrap();
/// json_to_gron(&mut stdout(), "val", &json)
/// // Outputs to stdout:
/// //
/// //   val = {}
/// //   val.x = []
/// //   val.x[0] = 1
/// //   val.x[1] = 2
/// # }
///
/// ```
pub fn json_to_gron<W: Write>(out: &mut W, prefix: &str, json: &Json)
    -> io::Result<()>
{
    use self::StackItem::*;

    let mut stack = Vec::with_capacity(8);
    let mut namebuf = String::with_capacity(100);
    namebuf.push_str(prefix);
    match *json {
        Json::I64(value) => try!(writeln!(out, "{} = {}", namebuf, value)),
        Json::U64(value) => try!(writeln!(out, "{} = {}", namebuf, value)),
        Json::F64(value) => try!(writeln!(out, "{} = {}", namebuf, value)),
        Json::String(ref value) => {
            try!(writeln!(out, "{} = {:?}", namebuf, value));
        }
        Json::Boolean(value) => try!(writeln!(out, "{} = {}", namebuf, value)),
        Json::Array(ref vec) => {
            try!(writeln!(out, "{} = []", namebuf));
            stack.push((VecIter(vec.iter().enumerate()), namebuf.len()));
        }
        Json::Object(ref keys) => {
            try!(writeln!(out, "{} = {{}}", namebuf));
            stack.push((MapIter(keys.iter()), namebuf.len()));
        }
        Json::Null => try!(writeln!(out, "{} = null", namebuf)),
    }
    while stack.len() > 0 {
        let (kind, off) = stack.pop().unwrap();
        namebuf.truncate(off);
        match kind {
            MapIter(mut iter) => {
                let (key, json) = match iter.next() {
                    Some((key, json)) => (key, json),
                    None => continue,
                };
                stack.push((MapIter(iter), off));
                match *json {
                    Json::I64(value) => {
                        try!(writeln!(out, "{}.{} = {}",
                            namebuf, key, value));
                    }
                    Json::U64(value) => {
                        try!(writeln!(out, "{}.{} = {}",
                            namebuf, key, value));
                    }
                    Json::F64(value) => {
                        try!(writeln!(out, "{}.{} = {}",
                            namebuf, key, value));
                    }
                    Json::String(ref value) => {
                        try!(writeln!(out, "{}.{} = {:?}",
                            namebuf, key, value));
                    }
                    Json::Boolean(value) => {
                        try!(writeln!(out, "{}.{} = {}",
                            namebuf, key, value));
                    }
                    Json::Null => {
                        try!(writeln!(out, "{}.{} = null",
                            namebuf, key));
                    }
                    Json::Array(ref vec) => {
                        namebuf.push('.');
                        namebuf.push_str(key);
                        try!(writeln!(out, "{} = []", namebuf));
                        stack.push((
                            VecIter(vec.iter().enumerate()),
                            namebuf.len()));
                    }
                    Json::Object(ref keys) => {
                        namebuf.push('.');
                        namebuf.push_str(key);
                        try!(writeln!(out, "{} = {{}}", namebuf));
                        stack.push((MapIter(keys.iter()), namebuf.len()));
                    }
                }
            }
            VecIter(mut iter) => {
                let (index, json) = match iter.next() {
                    Some((index, json)) => (index, json),
                    None => continue,
                };
                stack.push((VecIter(iter), off));
                match *json {
                    Json::I64(value) => {
                        try!(writeln!(out, "{}[{}] = {}",
                            namebuf, index, value));
                    }
                    Json::U64(value) => {
                        try!(writeln!(out, "{}[{}] = {}",
                            namebuf, index, value));
                    }
                    Json::F64(value) => {
                        try!(writeln!(out, "{}[{}] = {}",
                            namebuf, index, value));
                    }
                    Json::String(ref value) => {
                        try!(writeln!(out, "{}[{}] = {:?}",
                            namebuf, index, value));
                    }
                    Json::Boolean(value) => {
                        try!(writeln!(out, "{}[{}] = {}",
                            namebuf, index, value));
                    }
                    Json::Null => {
                        try!(writeln!(out, "{}[{}] = null",
                            namebuf, index));
                    }
                    Json::Array(ref vec) => {
                        use std::fmt::Write;
                        write!(&mut namebuf, "[{}]", index).unwrap();
                        try!(writeln!(out, "{} = []", namebuf));
                        stack.push((
                            VecIter(vec.iter().enumerate()),
                            namebuf.len()));
                    }
                    Json::Object(ref keys) => {
                        use std::fmt::Write;
                        write!(&mut namebuf, "[{}]", index).unwrap();
                        try!(writeln!(out, "{} = {{}}", namebuf));
                        stack.push((MapIter(keys.iter()), namebuf.len()));
                    }
                }
            }
        }
    }
    Ok(())
}


#[cfg(test)]
mod test {
    use rustc_serialize::json::Json;

    use super::json_to_gron;

    fn assert_equal(json_src: &str, gron: &str) {
        let mut buf = Vec::new();
        json_to_gron(&mut buf, "json",
            &Json::from_str(json_src).unwrap()).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), gron);
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
