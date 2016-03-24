use std::io::{self, Read, Write};

mod mock_io;
#[macro_use]
mod mock_stdio;

pub use mock_io::MockIo;

pub trait Io {
    type Input: Read;
    type Output: Write;
    type Error: Write;
    fn stdin() -> Self::Input;
    fn stdout() -> Self::Output;
    fn stderr() -> Self::Error;
}

pub struct Stdio;

impl Io for Stdio {
    type Input = io::Stdin;
    type Output = io::Stdout;
    type Error = io::Stderr;
    fn stdin() -> io::Stdin { io::stdin() }
    fn stdout() -> io::Stdout { io::stdout() }
    fn stderr() -> io::Stderr { io::stderr() }
}
