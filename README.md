# `mock_io` - mocking stdio through dependency injection

This library allows you to mock stdin, stdout, and stderr during testing. The
syntax needed to do so is unfortunately verbose, but in the unmocked case the
compiled output is the same.

Supposing you have an object which reads from stdin as a part of its operation,
you can declare it with a type parameter, and use the associated functions on
that type.

```rust
extern crate mock_io;

use std::marker::PhantomData;
use mock_io::{Io, Stdio};

struct Foo<T=Stdio> where T: for<'a> Io<'a> {
    ...
    _spoopy: PhantomData<T>,
}

impl<T> Foo<T> where T: for<'a> Io<'a> {

    fn bar(&mut self) -> io::Result<()> {
        ...
        let mut buf = [0; 1024];
        let len = T::stdin().read(&mut buf)?;
        T::stdout().write(&buf[..len]);
        ...
    }
}
```

In testing, you can do this to substitute a buffer for the actual stdio
handles:

```rust
#![feature(const_fn, static_mutex)]

#[macro_use]
extern crate mock_io;

#[test]
fn test_foo_bar() {
    const RELOCATE_TO_SF: &'static [u8] = b"It is a truth universally acknowledged that a single "
                                           "man in possession of a good fortune must be willing "
                                           "to relocate to San Francisco.";
    use_mock_stdio!();
    ...
    mock_stdio::set_stdin(RELOCATE_TO_SF);
    let foo = Foo::<mock_stdio::MockStdio>::new();
    assert!(foo.bar().is_ok());
    mock_stdio::check_stdout(|stdout| assert_eq!(stdout, RELOCATE_TO_SF);
    ...
}
```

The macro declaration here is important. Every time it is invoked, it
introduces a new `mock_stdio` in the current scope. Declare this macro inside
every test function which tests stdio actions, so that each test will be
interacting with its own buffer. Because tests are run in parallel, attempting
to share a set of `mock_stdio` buffers will produce flakey and unpredictable
results.

# Problems with this library

* It makes extensive use of the poor hygiene of `macro_rules!` macros at the
item level. `macro!` macros may probably not be able to replicate this when
they land.
* It only runs on nightly and it requires you to active nightly features
because it expands code which uses those features inside your tests.
* The syntax of using it is quite arcane, requiring both `PhantomData` _and_
higher-ranked lifetimes.
* It uses unsafe, maybe its unsound.

# License

`mock_io` is distributed under the terms of both the MIT license and the Apache
License (Version 2.0).

See COPYING-APACHE and COPYING-MIT for more details.
