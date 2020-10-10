use core::fmt;
use std::io;
use std::io::Write as io_Write;

#[cfg(test)]
mod tests {
    use crate::{log, log_child, LogNode, NullLogBackend};

    #[test]
    fn test_macros() {
        let b = NullLogBackend;
        let mut log = LogNode::new(&b);
        log!(log, "hi");
        {
            let mut log = log_child!(log, "child!");
            log!((log), "{}", ("bye"));
        }
    }

    #[test]
    fn test_lifetimes() {
        let b = NullLogBackend;
        let mut p = LogNode::new(&b);
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

pub trait LogBackend: Sized {
    fn put(&self, entry: fmt::Arguments, node: &LogNode<Self>);
}

#[derive(Debug)]
enum NameOrPath<'n> {
    Name(&'n str),
    _Path(String), // TODO
}

#[derive(Debug)]
pub struct LogNode<'n, R> {
    backend: &'n R,
    parent: Option<&'n LogNode<'n, R>>,
    name_or_path: Option<NameOrPath<'n>>,
}

impl<'n, R> LogNode<'n, R> {
    pub fn new(backend: &'n R) -> Self {
        Self {
            backend,
            parent: None,
            name_or_path: None,
        }
    }

    pub fn child_shared<'a>(self: &'a LogNode<'n, R>, name: &'a str)
    -> LogNode<'a, R> {
        LogNode {
            backend: self.backend,
            parent: Some(&*self),
            name_or_path: Some(NameOrPath::Name(name)),
        }
    }
}

impl<'n, R: LogBackend> LogNode<'n, R> {
    pub fn put(&mut self, entry: fmt::Arguments) {
        self.backend.put(entry, self);
    }

    pub fn child<'a>(self: &'a mut LogNode<'n, R>, entry: fmt::Arguments)
    -> LogNode<'a, R> {
        self.put(entry);
        LogNode {
            backend: self.backend,
            parent: Some(&*self),
            name_or_path: None,
        }
    }
}

pub struct NullLogBackend;

impl LogBackend for NullLogBackend {
    fn put<'n>(&self, _entry: fmt::Arguments, _node: &LogNode<Self>) {
        // nothing
    }
}

// TODO: Figure out a way to make this no_std.
pub struct PlainLogBackend;

impl LogBackend for PlainLogBackend {
    fn put(&self, entry: fmt::Arguments, mut node: &LogNode<Self>) {
        let mut v = Vec::new();

        while let Some(parent) = node.parent {
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

            node = parent;
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

#[macro_export]
macro_rules! log {
    ($logger:expr, $format:tt $(, $args:tt)* $(,)?) => (
        $logger.put(format_args!($format $(, $args)*))
    );
}

#[macro_export]
macro_rules! log_child {
    ($logger:expr, $format:tt $(, $args:tt)* $(,)?) => (
        $logger.child(format_args!($format $(, $args)*))
    );
}
