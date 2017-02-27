use std::io::{self, Write};
use std::iter::Enumerate;
use serde_json::map::Iter as MapIter;
use std::slice::Iter as VecIter;

use serde_json::value::Value;

use super::ToGron;

enum StackItem<'a> {
    MapIter(MapIter<'a>),
    VecIter(Enumerate<VecIter<'a, Value>>),
}


impl ToGron for Value {
    fn to_gron<W: Write>(&self, out: &mut W, prefix: &str)
        -> io::Result<()>
    {
        use self::StackItem::*;

        let mut stack = Vec::with_capacity(8);
        let mut namebuf = String::with_capacity(100);
        namebuf.push_str(prefix);
        match *self {
            Value::Number(ref value) => try!(writeln!(out, "{} = {}", namebuf, value)),
            Value::String(ref value) => {
                try!(writeln!(out, "{} = {:?}", namebuf, value));
            }
            Value::Bool(value) => try!(writeln!(out, "{} = {}", namebuf, value)),
            Value::Array(ref vec) => {
                try!(writeln!(out, "{} = []", namebuf));
                stack.push((VecIter(vec.iter().enumerate()), namebuf.len()));
            }
            Value::Object(ref keys) => {
                try!(writeln!(out, "{} = {{}}", namebuf));
                stack.push((MapIter(keys.iter()), namebuf.len()));
            }
            Value::Null => try!(writeln!(out, "{} = null", namebuf)),
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
                        Value::Number(ref value) => {
                            try!(writeln!(out, "{}.{} = {}",
                                namebuf, key, value));
                        }
                        Value::String(ref value) => {
                            try!(writeln!(out, "{}.{} = {:?}",
                                namebuf, key, value));
                        }
                        Value::Bool(value) => {
                            try!(writeln!(out, "{}.{} = {}",
                                namebuf, key, value));
                        }
                        Value::Null => {
                            try!(writeln!(out, "{}.{} = null",
                                namebuf, key));
                        }
                        Value::Array(ref vec) => {
                            namebuf.push('.');
                            namebuf.push_str(key);
                            try!(writeln!(out, "{} = []", namebuf));
                            stack.push((
                                VecIter(vec.iter().enumerate()),
                                namebuf.len()));
                        }
                        Value::Object(ref keys) => {
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
                        Value::Number(ref value) => {
                            try!(writeln!(out, "{}[{}] = {}",
                                namebuf, index, value));
                        }
                        Value::String(ref value) => {
                            try!(writeln!(out, "{}[{}] = {:?}",
                                namebuf, index, value));
                        }
                        Value::Bool(value) => {
                            try!(writeln!(out, "{}[{}] = {}",
                                namebuf, index, value));
                        }
                        Value::Null => {
                            try!(writeln!(out, "{}[{}] = null",
                                namebuf, index));
                        }
                        Value::Array(ref vec) => {
                            use std::fmt::Write;
                            write!(&mut namebuf, "[{}]", index).unwrap();
                            try!(writeln!(out, "{} = []", namebuf));
                            stack.push((
                                VecIter(vec.iter().enumerate()),
                                namebuf.len()));
                        }
                        Value::Object(ref keys) => {
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
}
