use std::cmp;
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};

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

impl Read for MockIo {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Ok(guard) = self.inner.lock() {
            let len = cmp::min(buf.len(), guard.len());
            buf[..len].clone_from_slice(&guard[..len]);
            Ok(len)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Mock IO mutex poisoned"))
        }
    }
}

impl Write for MockIo {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Ok(mut guard) = self.inner.lock() {
            guard.extend_from_slice(buf);
            Ok(buf.len())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Mock IO mutex poisoned"))
        }
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}
