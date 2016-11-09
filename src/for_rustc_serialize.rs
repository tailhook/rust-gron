use std::io::{self, Write};
use std::iter::Enumerate;
use std::collections::btree_map::Iter as MapIter;
use std::slice::Iter as VecIter;

use rustc_serialize::json::Json;

use super::ToGron;


enum StackItem<'a> {
    MapIter(MapIter<'a, String, Json>),
    VecIter(Enumerate<VecIter<'a, Json>>),
}


impl ToGron for Json {
    fn to_gron<W: Write>(&self, out: &mut W, prefix: &str)
        -> io::Result<()>
    {
        use self::StackItem::*;

        let mut stack = Vec::with_capacity(8);
        let mut namebuf = String::with_capacity(100);
        namebuf.push_str(prefix);
        match *self {
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
}
