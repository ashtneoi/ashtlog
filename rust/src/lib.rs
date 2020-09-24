use core::fmt;
use std::io;
use std::io::Write as io_Write;

// TODO: Better name?
pub trait LogReceiver: Sized {
    fn receive(&self, entry: fmt::Arguments, node: &LogNode<Self>);
}

#[derive(Debug)]
enum NameOrPath<'n> {
    Name(&'n str),
    _Path(String), // TODO
}

#[derive(Debug)]
pub struct LogNode<'n, R> {
    receiver: &'n R,
    parent: Option<&'n LogNode<'n, R>>,
    name_or_path: Option<NameOrPath<'n>>,
}

impl<'n, R> LogNode<'n, R> {
    pub fn new(receiver: &'n R) -> Self {
        Self {
            receiver,
            parent: None,
            name_or_path: None,
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
            name_or_path: None,
        }
    }

    pub fn child_shared<'a>(self: &'a LogNode<'n, R>, name: &'a str)
    -> LogNode<'a, R> {
        LogNode {
            receiver: self.receiver,
            parent: Some(&*self),
            name_or_path: Some(NameOrPath::Name(name)),
        }
    }
}

// TODO: Figure out a way to make this no_std.
// TODO: Add iterative alternative using Vec or something.
pub struct PlainLogReceiver;

impl LogReceiver for PlainLogReceiver {
    fn receive(&self, entry: fmt::Arguments, mut node: &LogNode<Self>) {
        let mut v = Vec::new();

        loop {
            match node.name_or_path {
                Some(NameOrPath::Name(s)) => {
                    let mut segment = String::new();
                    for c in s.chars() {
                        if c == '\\' || c == '/' || c == '>' {
                            segment.push('\\');
                        }
                        segment.push(c);
                    }
                    v.push(Some(segment));
                },
                None => v.push(None),
                _ => unimplemented!(),
            }

            match node.parent {
                Some(p) => { node = p; },
                None => break,
            }
        }

        let stdout_unlocked = io::stdout();
        let mut stdout = stdout_unlocked.lock();

        for _ in 0..v.len() {
            stdout.write_all(b"  ").unwrap();
        }

        stdout.write_all(b"[").unwrap();

        let mut prev_was_some = false;
        for segment in (&v).into_iter().rev() {
            match segment {
                Some(segment) => {
                    if prev_was_some {
                        stdout.write_all(b"/").unwrap();
                    }
                    stdout.write_all(segment.as_bytes()).unwrap();
                    prev_was_some = true;
                },
                None => {
                    stdout.write_all(b">").unwrap();
                    prev_was_some = false;
                },
            }
        }

        stdout.write_all(b"] ").unwrap();
        stdout.write_fmt(entry).unwrap();
        stdout.write_all(b"\n").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use core::fmt;
    use crate::{LogNode, LogReceiver};

    struct NullLogReceiver;

    impl LogReceiver for NullLogReceiver {
        fn receive<'n>(&self, _entry: fmt::Arguments, _node: &LogNode<Self>) {
            // nothing
        }
    }

    #[test]
    fn test_lifetimes() {
        let r = NullLogReceiver;
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
