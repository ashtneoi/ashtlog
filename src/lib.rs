#![no_std]

extern crate alloc;

use alloc::sync::Arc;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt; // :(
use core::marker::PhantomData;

pub unsafe trait SharedWrite {
    fn write_str(&self, s: &str) -> Result<(), fmt::Error>;

    // TODO: Do we want the rest of the fmt::Write methods?
}

// TODO: Impl SharedWrite for things in std.

// TODO: Blanket-impl SharedWrite for Mutex<W: Write>, if that actually makes
// sense.

#[derive(Debug)]
pub struct LogRoot<W: SharedWrite> {
    writer: W,
}

impl<W: SharedWrite> LogRoot<W> {
    fn node<'a>(&'a mut self) -> LogNode<'a, W> {
        LogNode {
            parent: PhantomData,
            path: LogPath::Here("".into()),
            root: self,
            indent: 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum NamedLogNodeError {
    DifferentPrefix(String),
}

// TODO: Show that `root` being a raw pointer is sound. Or maybe find a safe
// alternative.
#[derive(Debug)]
pub struct LogNode<'n, W: SharedWrite> {
    parent: PhantomData<&'n mut ()>,
    path: LogPath<'n>,
    root: *const LogRoot<W>,
    indent: usize,
}

impl<'n, W: SharedWrite> LogNode<'n, W> {
    pub fn put(&mut self, entry: &str) {
        unimplemented!();
    }

    pub fn child<'a>(&'a mut self, entry: &str) -> LogNode<'a, W> {
        self.put(entry);
        LogNode {
            parent: PhantomData,
            path: match self.path {
                LogPath::Here(ref s) => LogPath::NotHere(s),
                LogPath::NotHere(s) => LogPath::NotHere(s),
            },
            root: self.root,
            indent: self.indent + 1,
        }
    }

    pub fn named_child(&self, name: String, entry: &str)
    -> Result<LogNode<'n, W>, NamedLogNodeError> {
        // TODO: This is a common pattern. Just impl Deref on LogPath or
        // something.
        let p = match self.path {
            LogPath::Here(ref s) => s,
            LogPath::NotHere(s) => s,
        };
        if !(name.starts_with(p) && name[p.len()..].starts_with("/")) {
            return Err(NamedLogNodeError::DifferentPrefix(name));
        }
        Ok(LogNode {
            parent: PhantomData,
            path: LogPath::Here(name),
            root: self.root,
            indent: self.indent + 1,
        })
    }
}

#[derive(Debug)]
pub enum LogPath<'p> {
    // TODO: Depending on 'alloc' feature, this contains either String or
    // &'static str. (I think.)
    Here(String),
    NotHere(&'p str),
}
