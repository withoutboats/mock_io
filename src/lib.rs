use std::io::{self, BufRead, Read, Write};

mod mock_io;
#[macro_use]
mod mock_stdio_macro;

pub use mock_io::{MockIo, MockIoLock};

pub trait Io<'a>: Send + Sync + 'a {
    type Input: Read + Lock<'a, Lock=Self::InputLock>;
    type InputLock: Read + BufRead + 'a;
    type Output: Write + Lock<'a, Lock=Self::OutputLock>;
    type OutputLock: Write + 'a;
    type Error: Write + Lock<'a, Lock=Self::ErrorLock>;
    type ErrorLock: Write + 'a;
    fn stdin() -> Self::Input;
    fn stdout() -> Self::Output;
    fn stderr() -> Self::Error;
    fn stdin_read_line(buf: &mut String) -> io::Result<usize>;
}

pub struct Stdio;

impl<'a> Io<'a> for Stdio {
    type Input = io::Stdin;
    type InputLock = io::StdinLock<'a>;
    type Output = io::Stdout;
    type OutputLock = io::StdoutLock<'a>;
    type Error = io::Stderr;
    type ErrorLock = io::StderrLock<'a>;
    fn stdin() -> io::Stdin { io::stdin() }
    fn stdout() -> io::Stdout { io::stdout() }
    fn stderr() -> io::Stderr { io::stderr() }
    fn stdin_read_line(buf: &mut String) -> io::Result<usize> { io::stdin().read_line(buf) }
}

pub trait Lock<'a> {
    type Lock: 'a;
    fn lock(&'a self) -> Self::Lock;
}

impl<'a> Lock<'a> for io::Stdin {
    type Lock = io::StdinLock<'a>;
    fn lock(&'a self) -> io::StdinLock<'a> {
        self.lock()
    }
}

impl<'a> Lock<'a> for io::Stdout {
    type Lock = io::StdoutLock<'a>;
    fn lock(&'a self) -> io::StdoutLock<'a> {
        self.lock()
    }
}

impl<'a> Lock<'a> for io::Stderr {
    type Lock = io::StderrLock<'a>;
    fn lock(&'a self) -> io::StderrLock<'a> {
        self.lock()
    }
}
