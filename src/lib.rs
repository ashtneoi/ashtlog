#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt; // :(
use core::marker::PhantomData;

pub unsafe trait SyncWrite {
    fn write_str(&self, s: &str) -> Result<(), fmt::Error>;

    // TODO: Do we want the rest of the fmt::Write methods?
}

// TODO: Impl SyncWrite for things in std.

// TODO: Once specialization is stabilized, blanket-impl SyncWrite for
// Mutex<W: Write>, if that actually makes sense.

pub struct LogRoot<W: SyncWrite> {
    path: LogPath,
    writer: W,
}

impl<W: SyncWrite> LogRoot<W> {
    fn node<'a>(&'a mut self) -> LogNode<'a, 'a, W> {
        LogNode {
            parent: PhantomData,
            path: &self.path,
            root: self as *const Self,
            indent: 0,
        }
    }
}

pub struct LogNode<'n, 'p, W: SyncWrite> {
    parent: PhantomData<&'n mut ()>,
    path: &'p LogPath,
    root: *const LogRoot<W>, // TODO: show that this is sound
    indent: usize,
}

impl<'n, 'p, W: SyncWrite> LogNode<'n, 'p, W> {
    pub fn put(&mut self, entry: &str) {
        unsafe {
            let root = &*self.root;
            root.writer.write_str(entry).unwrap(); // XXX
        }
    }

    pub fn child<'a>(&'a mut self, entry: &str) -> LogNode<'a, 'p, W> {
        LogNode {
            parent: PhantomData,
            path: &*self.path,
            root: self.root,
            indent: self.indent + 1,
        }
    }
}

pub struct LogPath(Vec<String>);
