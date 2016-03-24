#[macro_export]
macro_rules! use_mock_stdio {
    () => {
        #[allow(dead_code)]
        mod mock_stdio {
            use std::cell::Cell;
            use std::io::Write;
            use std::ptr;
            use std::sync::StaticMutex;
            
            use mock_io::{Io, MockIo};

            pub fn stdin() -> MockIo {
                MockStdio::stdin()
            }

            pub fn stdout() -> MockIo {
                MockStdio::stdout()
            }

            pub fn stderr() -> MockIo {
                MockStdio::stderr()
            }
            
            pub fn set_stdin(data: &[u8]) {
                MOCK_STDIO.input().set_data(data).expect("Stdin mutex poisoned");
            }
        
            pub fn extend_stdin(data: &[u8]) {
                MOCK_STDIO.input().write(data).expect("Stdin mutex poisoned");
            }
        
            pub fn check_stdout<F>(check: F) where F: FnOnce(&[u8]) {
                check(&MOCK_STDIO.output().get_data().expect("Stdout mutex poisoned"));
            }
        
            pub fn check_stderr<F>(check: F) where F: FnOnce(&[u8]) {
                check(&MOCK_STDIO.error().get_data().expect("Stdout mutex poisoned"));
            }
            
            pub struct MockStdio {
                lock: StaticMutex,
                input: Cell<*mut MockIo>,
                output: Cell<*mut MockIo>,
                error: Cell<*mut MockIo>,
                init: fn() -> MockIo,
            }
    
            unsafe impl Sync for MockStdio { }
            
            impl MockStdio {

                fn input(&'static self) -> MockIo {
                    self.get(&self.input)
                }
                fn output(&'static self) -> MockIo {
                    self.get(&self.output)
                }
                fn error(&'static self) -> MockIo {
                    self.get(&self.error)
                }
                fn get(&'static self, cell: &Cell<*mut MockIo>) -> MockIo {
                    let _g = self.lock.lock();
                    let ptr = cell.get();
                    unsafe {
                        if ptr.is_null() {
                            cell.set(Box::into_raw(Box::new((self.init)())));
                            (*cell.get()).clone()
                        } else {
                            (*ptr).clone()
                        }
                    }
                }
            }
            
            impl Io for MockStdio {
                type Input = MockIo;
                type Output = MockIo;
                type Error = MockIo;
                fn stdin() -> MockIo {
                    MOCK_STDIO.input()
                }
                fn stdout() -> MockIo {
                    MOCK_STDIO.output()
                }
                fn stderr() -> MockIo {
                    MOCK_STDIO.error()
                }
            }
            
            pub static MOCK_STDIO: MockStdio = MockStdio {
                lock: StaticMutex::new(),
                input: Cell::new(ptr::null_mut()),
                output: Cell::new(ptr::null_mut()),
                error: Cell::new(ptr::null_mut()),
                init: MockIo::new,
            };
        }
    };
}

