#![no_std]

extern crate alloc;

use alloc::string::String;
use core::fmt;

// TODO: Better name?
pub trait LogReceiver: Sync + Sized {
    fn receive(&self, entry: fmt::Arguments, node: &LogNode<Self>);
}

// TODO: Depending on 'alloc' feature, this is either String or
// &'static str. (I think.)
type LogNodeName = String;

#[derive(Debug)]
enum Parent<'n, R: LogReceiver> {
    Mut(&'n mut LogNode<'n, R>),
    Shared(&'n LogNode<'n, R>),
    None,
}

#[derive(Debug)]
pub struct LogNode<'n, R: LogReceiver> {
    receiver: &'n R,
    parent: Parent<'n, R>,
    name: Option<LogNodeName>,
}

impl<'n, R: LogReceiver> LogNode<'n, R> {
    pub fn new(receiver: &'n R) -> Self {
        Self {
            receiver,
            parent: Parent::None,
            name: None,
        }
    }

    pub fn put(&mut self, entry: fmt::Arguments) {
        self.receiver.receive(entry, self);
    }

    pub fn child<'a>(&'a mut self, entry: fmt::Arguments) -> Self {
        self.put(entry);
        LogNode {
            receiver: self.receiver,
            parent: Parent::Mut(self),
            name: None,
        }
    }

    pub fn child_shared<'a>(&'a self, name: LogNodeName) -> Self {
        LogNode {
            receiver: self.receiver,
            parent: Parent::Shared(self),
            name: Some(name),
        }
    }
}

mod tests {
    use alloc::string::ToString;
    use crate::{LogNode, LogReceiver};
    use core::fmt;

    impl LogReceiver for () {
        fn receive<'n>(&self, _entry: fmt::Arguments, _node: &LogNode<Self>) {
        }
    }

    #[test]
    fn test_lifetimes() {
        let r = ();
        let mut n = LogNode::new(&r);
        {
            n.child_shared("hi".to_string());
        }
        n.put(format_args!("hi"));
    }
}
