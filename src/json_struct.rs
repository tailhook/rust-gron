use std::io::{self, Write};
use std::iter::Enumerate;
use std::collections::btree_map::Iter as MapIter;
use std::slice::Iter as VecIter;

use rustc_serialize::json::Json;


enum StackItem<'a> {
    MapIter(MapIter<'a, String, Json>),
    VecIter(Enumerate<VecIter<'a, Json>>),
}


pub fn json_to_gron<W: Write>(out: &mut W, name: &str, json: &Json)
    -> io::Result<()>
{
    use self::StackItem::*;

    let mut stack = Vec::with_capacity(8);
    let mut namebuf = String::with_capacity(100);
    namebuf.push_str(name);
    match *json {
        Json::I64(value) => try!(writeln!(out, "{} = {}", name, value)),
        Json::U64(value) => try!(writeln!(out, "{} = {}", name, value)),
        Json::F64(value) => try!(writeln!(out, "{} = {}", name, value)),
        Json::String(ref value) => {
            try!(writeln!(out, "{} = {:?}", name, value));
        }
        Json::Boolean(value) => try!(writeln!(out, "{} = {}", name, value)),
        Json::Array(ref vec) => {
            try!(writeln!(out, "{} = []", name));
            stack.push((VecIter(vec.iter().enumerate()), namebuf.len()));
        }
        Json::Object(ref keys) => {
            try!(writeln!(out, "{} = []", name));
            stack.push((MapIter(keys.iter()), namebuf.len()));
        }
        Json::Null => try!(writeln!(out, "{} = null", name)),
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
}
