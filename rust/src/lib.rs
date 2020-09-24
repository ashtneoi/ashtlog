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

impl PlainLogReceiver {
    // TODO: Do we need this?
    fn _print_prefix_recursive(&self, node: &LogNode<Self>) {
        let stdout_unlocked = io::stdout();
        let mut stdout = stdout_unlocked.lock();
        if let Some(p) = node.parent {
            self.print_prefix(p);
        }
        match node.name_or_path {
            Some(NameOrPath::Name(s)) => {
                if node.parent.map(|p| p.name_or_path.is_some()) == Some(true) {
                    stdout.write_all(b"/").unwrap();
                }
                let mut char_buf = [0; 4];
                for c in s.chars() {
                    if c == '\\' || c == '/' || c == '>' {
                        stdout.write_all(b"\\").unwrap();
                    }
                    stdout.write_all(
                        c.encode_utf8(&mut char_buf).as_bytes()
                    ).unwrap();
                }
            },
            None => stdout.write_all(b">").unwrap(),
            _ => unimplemented!(),
        }
    }

    fn print_prefix(&self, mut node: &LogNode<Self>) {
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
    }
}

impl LogReceiver for PlainLogReceiver {
    // FIXME: No unwrap.
    fn receive(&self, entry: fmt::Arguments, node: &LogNode<Self>) {
        {
            let stdout_unlocked = io::stdout();
            let mut stdout = stdout_unlocked.lock();
            stdout.write_all(b"[").unwrap();
        }

        self.print_prefix(node);

        {
            let stdout_unlocked = io::stdout();
            let mut stdout = stdout_unlocked.lock();
            stdout.write_all(b"] ").unwrap();
            stdout.write_fmt(entry).unwrap();
            stdout.write_all(b"\n").unwrap();
        }
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
