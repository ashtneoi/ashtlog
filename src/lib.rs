#![no_std]

extern crate alloc;

use alloc::string::String;
use core::fmt;
use core::marker::PhantomData;
use core::ops::Deref;

pub unsafe trait SharedWrite {
    fn write_str(&self, s: &str) -> fmt::Result;
    fn write_char(&self, c: char) -> fmt::Result;
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
}

// TODO: Impl SharedWrite for things in std.

// TODO: Blanket-impl SharedWrite for Mutex<W: Write>, if that actually makes
// sense.

#[derive(Debug)]
pub struct LogRoot<W: SharedWrite> {
    writer: W,
}

impl<W: SharedWrite> LogRoot<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn node<'a>(&'a mut self) -> LogNode<'a, W> {
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
    root: &'n LogRoot<W>,
    indent: usize,
}

impl<'n, W: SharedWrite> LogNode<'n, W> {
    pub fn put(&mut self, entry: fmt::Arguments) {
        for _ in 0..self.indent {
            self.root.writer.write_char(' ').unwrap();
        }
        self.put_path();
        self.root.writer.write_fmt(entry).unwrap();
        self.root.writer.write_char('\n').unwrap();
    }

    fn put_path(&mut self) {
        let s = &*self.path;
        self.root.writer.write_char(0 as char).unwrap();
        self.root.writer.write_str(s).unwrap();
        self.root.writer.write_char(0 as char).unwrap();
    }

    pub fn child<'a>(&'a mut self, entry: fmt::Arguments) -> LogNode<'a, W> {
        self.put(entry);
        LogNode {
            parent: PhantomData,
            path: LogPath::NotHere(&*self.path),
            root: self.root,
            indent: self.indent + 1,
        }
    }

    pub fn named_child(&mut self, name: LogPathString, entry: fmt::Arguments)
    -> Result<LogNode<'n, W>, NamedLogNodeError> {
        self.put(entry);
        if !(
            name.starts_with(&*self.path)
            && name[self.path.len()..].starts_with("/")
        ) {
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

// TODO: Depending on 'alloc' feature, this is either String or
// &'static str. (I think.)
type LogPathString = String;

#[derive(Debug)]
pub enum LogPath<'p> {
    Here(LogPathString),
    NotHere(&'p str),
}

impl<'p> Deref for LogPath<'p> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Here(ref s) => s,
            Self::NotHere(s) => s,
        }
    }
}
