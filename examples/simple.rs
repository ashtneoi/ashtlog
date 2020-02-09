use ashtlog;
use ashtlog::SharedWrite;
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
    node.put(format_args!("{}", "wat"));
    let mut child = node.child(format_args!("{}", "here comes a special boy"));
    child.put(format_args!("i'm the boy"));
    node.put(format_args!("okay enough of {}", "that"));
}
