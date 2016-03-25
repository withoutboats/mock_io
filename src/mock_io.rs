use std::cmp;
use std::io::{self, BufRead, Read, Write};
use std::sync::{Arc, Mutex, MutexGuard};

use Lock;

#[derive(Clone)]
pub struct MockIo {
    inner: Arc<Mutex<Vec<u8>>>
}

impl MockIo {

    pub fn new() -> MockIo {
        MockIo {
            inner: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn get_data(&self) -> io::Result<Vec<u8>> {
        if let Ok(guard) = self.inner.lock() {
            Ok(guard.clone())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Mock IO mutex poisoned"))
        }
    }

    pub fn set_data(&self, data: &[u8]) -> io::Result<()> {
        if let Ok(mut guard) = self.inner.lock() {
            guard.clear();
            guard.extend_from_slice(data);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Mock IO mutex poisoned"))
        }
    }

}

impl<'a> Lock<'a> for MockIo {
    type Lock = MockIoLock<'a>;
    fn lock(&'a self) -> MockIoLock<'a> {
        MockIoLock {
            inner: self.inner.lock().expect("Mock IO mutex poisoned")
        }
    }
}

impl Read for MockIo {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Ok(mut guard) = self.inner.lock() { read(&mut guard, buf) }
        else { Err(io::Error::new(io::ErrorKind::Other, "Mock IO mutex poisoned")) }
    }
}

impl Write for MockIo {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Ok(mut guard) = self.inner.lock() { write(&mut guard, buf) }
        else { Err(io::Error::new(io::ErrorKind::Other, "Mock IO mutex poisoned")) }
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

pub struct MockIoLock<'a> {
    inner: MutexGuard<'a, Vec<u8>>,
}

impl<'a> BufRead for MockIoLock<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Ok(&self.inner)
    }
    fn consume(&mut self, amt: usize) {
        self.inner.drain(..amt).fold((), |_, _| ());
    }
}

impl<'a> Read for MockIoLock<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        read(&mut self.inner, buf)
    }
}

impl<'a> Write for MockIoLock<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        write(&mut self.inner, buf)
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

fn read(mock: &mut MutexGuard<Vec<u8>>, buf: &mut [u8]) -> io::Result<usize> {
    let len = cmp::min(buf.len(), mock.len());
    buf[..len].clone_from_slice(&mock[..len]);
    mock.drain(..len).fold((), |_, _| ());
    Ok(len)
}

fn write(mock: &mut MutexGuard<Vec<u8>>, buf: &[u8]) -> io::Result<usize> {
    mock.extend_from_slice(buf);
    Ok(buf.len())
}
