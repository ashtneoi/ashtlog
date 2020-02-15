use ashtlog;
use core::fmt;
use std::io;
use std::io::prelude::*;

struct StdoutSharedWriter;

// Only sound because we're single-threaded.
unsafe impl ashtlog::SharedWrite for StdoutSharedWriter {
    fn write_str(&self, s: &str) -> fmt::Result {
        print!("{}", s);
        Ok(())
    }

    fn write_char(&self, c: char) -> fmt::Result {
        print!("{}", c);
        Ok(())
    }

    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        match io::stdout().write_fmt(args) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

fn main() {
    let mut root = ashtlog::LogRoot::new(StdoutSharedWriter);
    let mut node = root.node();

    node.put(format_args!("Going to create some (fake) threads"));
    node.put(format_args!("They'll each print some nested messages"));
    let mut c1 = node.named_child(
        "/thread 1".to_string(), format_args!("Thread 1")
    ).unwrap();
    let mut c2 = node.named_child(
        "/thread 2".to_string(), format_args!("Thread 2")
    ).unwrap();
    c1.put(format_args!("Hi! I'm thread 1."));
    c2.put(format_args!("Hi! I'm thread 2."));
    let mut c1x = c1.child(format_args!("Making a child"));
    c1x.put(format_args!("I'm the child"));
    let mut c2x = c2.child(format_args!("Making a child"));
    c2x.put(format_args!("I'm the child"));
}
