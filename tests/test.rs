#![feature(const_fn, static_mutex)]

#[macro_use]
extern crate mock_io;

use std::io::{Read, Write};
use std::marker::PhantomData;

use mock_io::{Io, Stdio};

#[test]
fn test_stdin() {
    use_mock_stdio!();
    mock_stdio::set_stdin(b"YELLOW SUBMARINE");
    let mut buf = vec![0; 16];
    mock_stdio::stdin().read(&mut buf).unwrap();
    assert_eq!(&buf, b"YELLOW SUBMARINE");
}

#[test]
fn test_stdout() {
    use_mock_stdio!();
    mock_stdio::stdout().write(b"Hello, world!").unwrap();
    mock_stdio::check_stdout(|stdout| {
        assert_eq!(stdout, b"Hello, world!");
    });
}

#[test]
fn test_stderr() {
    use_mock_stdio!();
    mock_stdio::stderr().write(b"Danger, Will Robinson!").unwrap();
    mock_stdio::check_stderr(|stderr| {
        assert_eq!(stderr, b"Danger, Will Robinson!");
    });
}

#[test]
fn test_isolation() {
    fn first_test() {
        use_mock_stdio!();
        mock_stdio::set_stdin(b"YELLOW SUBMARINE");
    }

    fn second_test() {
        use_mock_stdio!();
        let mut buf = vec![0; 16];
        mock_stdio::stdin().read_to_end(&mut buf).unwrap();
        assert_eq!(&buf, &[0; 16]);
    }

    first_test();
    second_test();
}

struct Foo<T: Io=Stdio> {
    _spoopy: PhantomData<T>
}

impl<T=Stdio> Foo<T> where T: Io {
    fn print(data: &[u8]) {
        T::stdout().write(data).unwrap();
    }
}

#[test]
fn test_in_type_parameters() {
    use_mock_stdio!();
    Foo::<mock_stdio::MockStdio>::print(b"Hello, world!");
    mock_stdio::check_stdout(|stdout| assert_eq!(stdout, b"Hello, world!"));
}

#[test]
#[should_panic]
fn test_without_mock_stdio() {
    use_mock_stdio!();
    Foo::<Stdio>::print(b"Hello, world!");
    mock_stdio::check_stdout(|stdout| assert_eq!(stdout, b"Hello, world!"));
}

fn echo<T: Io>() {
    let mut buf = [0; 128];
    let len = T::stdin().read(&mut buf).unwrap();
    T::stdout().write(&buf[..len]).unwrap();
}

#[test]
fn test_in_functions() {
    const HELLO_WORLD: &'static [u8] = b"Hello, world!\n";
    use_mock_stdio!();
    mock_stdio::set_stdin(HELLO_WORLD);
    echo::<mock_stdio::MockStdio>();
    mock_stdio::check_stdout(|stdout| assert_eq!(stdout, HELLO_WORLD));
}
