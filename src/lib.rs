use core::fmt;

// TODO: Better name?
pub trait LogReceiver: Sync + Sized {
    fn receive(&self, entry: fmt::Arguments, node: &LogNode<Self>);
}

// TODO: Depending on 'alloc' feature, this is either String or
// &'static str. (I think.)
type LogNodeName = &'static str;

#[derive(Debug)]
pub struct LogNode<'n, R> {
    receiver: &'n R,
    parent: Option<&'n LogNode<'n, R>>,
    name: Option<LogNodeName>,
}

impl<'n, R> LogNode<'n, R> {
    pub fn new(receiver: &'n R) -> Self {
        Self {
            receiver,
            parent: None,
            name: None,
        }
    }
}

impl<'n, R: LogReceiver> LogNode<'n, R> {
    pub fn put(&mut self, entry: fmt::Arguments) {
        self.receiver.receive(entry, self);
    }

    pub fn child<'a>(self: &'a mut LogNode<'n, R>, entry: fmt::Arguments)
    -> LogNode<'a, R> {
        self.put(entry);
        LogNode {
            receiver: self.receiver,
            parent: Some(&*self),
            name: None,
        }
    }

    pub fn child_shared<'a>(self: &'a LogNode<'n, R>, name: LogNodeName)
    -> LogNode<'a, R> {
        LogNode {
            receiver: self.receiver,
            parent: Some(&*self),
            name: Some(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::fmt;
    use crate::{LogNode, LogReceiver};

    struct SimplePrintlnLogDumper;

    impl LogReceiver for SimplePrintlnLogDumper {
        fn receive<'n>(&self, entry: fmt::Arguments, _node: &LogNode<Self>) {
            let mut s = String::new();
            fmt::write(&mut s, entry).unwrap();
            println!("{}", s);
        }
    }

    #[test]
    fn test_lifetimes() {
        let r = SimplePrintlnLogDumper;
        let mut p = LogNode::new(&r);
        p.put(format_args!("outer 1"));
        {
            let mut c = p.child_shared("1");
            let mut d = p.child_shared("2");
            c.put(format_args!("inner"));
            d.put(format_args!("inner"));
        }
        {
            let mut c = p.child(format_args!("child"));
            c.put(format_args!("inner"));
        }
        p.put(format_args!("outer 2"));
    }
}
