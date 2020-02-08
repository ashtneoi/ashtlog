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

// TODO: Once specialization is stabilized, blanket-impl SharedWrite for
// Mutex<W: Write>, if that actually makes sense.

pub struct LogRoot<W: SharedWrite> {
    writer: W,
}

impl<W: SharedWrite> LogRoot<W> {
    fn node<'a>(&'a mut self) -> LogNode<'a, W> {
        unimplemented!();
    }
}

pub struct LogNode<'n, W: SharedWrite> {
    parent: PhantomData<&'n mut ()>,
    path: LogPath<'n>,
    root: *const LogRoot<W>, // TODO: show that this is sound
    indent: usize,
}

impl<'n, W: SharedWrite> LogNode<'n, W> {
    pub fn put(&mut self, entry: &str) {
        unimplemented!();
    }

    pub fn child<'a>(&'a mut self, entry: &str) -> LogNode<'a, W> {
        unimplemented!();
    }

    // TODO: Should `name` be &str, String, or LogPath?
    pub fn named_child(&self, name: &str, entry: &str) -> LogNode<'n, W> {
        unimplemented!();
    }
}

// Depending on 'alloc' feature, this contains either String or &'static str.
// (I think.)
//pub struct LogPath(String);
pub enum LogPath<'p> {
    Here(String),
    NotHere(&'p str),
}
