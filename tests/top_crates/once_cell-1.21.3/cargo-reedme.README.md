# Overview

`once_cell` provides two new cell-like types, [`unsync::OnceCell`] and
[`sync::OnceCell`]. A `OnceCell` might store arbitrary non-`Copy` types, can
be assigned to at most once and provides direct access to the stored
contents. The core API looks *roughly* like this (and there’s much more
inside, read on!):

```rust
impl<T> OnceCell<T> {
    const fn new() -> OnceCell<T> { ... }
    fn set(&self, value: T) -> Result<(), T> { ... }
    fn get(&self) -> Option<&T> { ... }
}
```

Note that, like with [`RefCell`] and [`Mutex`], the `set` method requires
only a shared reference. Because of the single assignment restriction `get`
can return a `&T` instead of `Ref<T>` or `MutexGuard<T>`.

The `sync` flavor is thread-safe (that is, implements the [`Sync`] trait),
while the `unsync` one is not.

[`unsync::OnceCell`]: unsync/struct.OnceCell.html
[`sync::OnceCell`]: sync/struct.OnceCell.html
[`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html

# Recipes

`OnceCell` might be useful for a variety of patterns.

## Safe Initialization of Global Data

```rust
use std::{env, io};

use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Logger {
    // ...
}
static INSTANCE: OnceCell<Logger> = OnceCell::new();

impl Logger {
    pub fn global() -> &'static Logger {
        INSTANCE.get().expect("logger is not initialized")
    }

    fn from_cli(args: env::Args) -> Result<Logger, std::io::Error> {
       // ...
    }
}

fn main() {
    let logger = Logger::from_cli(env::args()).unwrap();
    INSTANCE.set(logger).unwrap();
    // use `Logger::global()` from now on
}
```

## Lazy Initialized Global Data

This is essentially the `lazy_static!` macro, but without a macro.

```rust
use std::{sync::Mutex, collections::HashMap};

use once_cell::sync::OnceCell;

fn global_data() -> &'static Mutex<HashMap<i32, String>> {
    static INSTANCE: OnceCell<Mutex<HashMap<i32, String>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(13, "Spica".to_string());
        m.insert(74, "Hoyten".to_string());
        Mutex::new(m)
    })
}
```

There are also the [`sync::Lazy`] and [`unsync::Lazy`] convenience types to
streamline this pattern:

```rust
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

fn main() {
    println!("{:?}", GLOBAL_DATA.lock().unwrap());
}
```

Note that the variable that holds `Lazy` is declared as `static`, *not*
`const`. This is important: using `const` instead compiles, but works wrong.

[`sync::Lazy`]: sync/struct.Lazy.html
[`unsync::Lazy`]: unsync/struct.Lazy.html

## General purpose lazy evaluation

Unlike `lazy_static!`, `Lazy` works with local variables.

```rust
use once_cell::unsync::Lazy;

fn main() {
    let ctx = vec![1, 2, 3];
    let thunk = Lazy::new(|| {
        ctx.iter().sum::<i32>()
    });
    assert_eq!(*thunk, 6);
}
```

If you need a lazy field in a struct, you probably should use `OnceCell`
directly, because that will allow you to access `self` during
initialization.

```rust
use std::{fs, path::PathBuf};

use once_cell::unsync::OnceCell;

struct Ctx {
    config_path: PathBuf,
    config: OnceCell<String>,
}

impl Ctx {
    pub fn get_config(&self) -> Result<&str, std::io::Error> {
        let cfg = self.config.get_or_try_init(|| {
            fs::read_to_string(&self.config_path)
        })?;
        Ok(cfg.as_str())
    }
}
```

## Lazily Compiled Regex

This is a `regex!` macro which takes a string literal and returns an
*expression* that evaluates to a `&'static Regex`:

```rust
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}
```

This macro can be useful to avoid the “compile regex on every loop
iteration” problem.

## Runtime `include_bytes!`

The `include_bytes` macro is useful to include test resources, but it slows
down test compilation a lot. An alternative is to load the resources at
runtime:

```rust
use std::path::Path;

use once_cell::sync::OnceCell;

pub struct TestResource {
    path: &'static str,
    cell: OnceCell<Vec<u8>>,
}

impl TestResource {
    pub const fn new(path: &'static str) -> TestResource {
        TestResource { path, cell: OnceCell::new() }
    }
    pub fn bytes(&self) -> &[u8] {
        self.cell.get_or_init(|| {
            let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let path = Path::new(dir.as_str()).join(self.path);
            std::fs::read(&path).unwrap_or_else(|_err| {
                panic!("failed to load test resource: {}", path.display())
            })
        }).as_slice()
    }
}

static TEST_IMAGE: TestResource = TestResource::new("test_data/lena.png");

#[test]
fn test_sobel_filter() {
    let rgb: &[u8] = TEST_IMAGE.bytes();
    // ...
}
```

## `lateinit`

`LateInit` type for delayed initialization. It is reminiscent of Kotlin’s
`lateinit` keyword and allows construction of cyclic data structures:


```rust
use once_cell::sync::OnceCell;

pub struct LateInit<T> { cell: OnceCell<T> }

impl<T> LateInit<T> {
    pub fn init(&self, value: T) {
        assert!(self.cell.set(value).is_ok())
    }
}

impl<T> Default for LateInit<T> {
    fn default() -> Self { LateInit { cell: OnceCell::default() } }
}

impl<T> std::ops::Deref for LateInit<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.cell.get().unwrap()
    }
}

#[derive(Default)]
struct A<'a> {
    b: LateInit<&'a B<'a>>,
}

#[derive(Default)]
struct B<'a> {
    a: LateInit<&'a A<'a>>
}


fn build_cycle() {
    let a = A::default();
    let b = B::default();
    a.b.init(&b);
    b.a.init(&a);

    let _a = &a.b.a.b.a;
}
```

# Comparison with std

|`!Sync` types         | Access Mode            | Drawbacks                                     |
|----------------------|------------------------|-----------------------------------------------|
|`Cell<T>`             | `T`                    | requires `T: Copy` for `get`                  |
|`RefCell<T>`          | `RefMut<T>` / `Ref<T>` | may panic at runtime                          |
|`unsync::OnceCell<T>` | `&T`                   | assignable only once                          |

|`Sync` types          | Access Mode            | Drawbacks                                     |
|----------------------|------------------------|-----------------------------------------------|
|`AtomicT`             | `T`                    | works only with certain `Copy` types          |
|`Mutex<T>`            | `MutexGuard<T>`        | may deadlock at runtime, may block the thread |
|`sync::OnceCell<T>`   | `&T`                   | assignable only once, may block the thread    |

Technically, calling `get_or_init` will also cause a panic or a deadlock if
it recursively calls itself. However, because the assignment can happen only
once, such cases should be more rare than equivalents with `RefCell` and
`Mutex`.

# Minimum Supported `rustc` Version

If only the `std`, `alloc`, or `race` features are enabled, MSRV will be
updated conservatively, supporting at least latest 8 versions of the compiler.
When using other features, like `parking_lot`, MSRV might be updated more
frequently, up to the latest stable. In both cases, increasing MSRV is *not*
considered a semver-breaking change and requires only a minor version bump.

# Implementation details

The implementation is based on the
[`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs/) and
[`lazy_cell`](https://github.com/indiv0/lazycell/) crates and
[`std::sync::Once`]. In some sense, `once_cell` just streamlines and unifies
those APIs.

To implement a sync flavor of `OnceCell`, this crates uses either a custom
re-implementation of `std::sync::Once` or `parking_lot::Mutex`. This is
controlled by the `parking_lot` feature (disabled by default). Performance
is the same for both cases, but the `parking_lot` based `OnceCell<T>` is
smaller by up to 16 bytes.

This crate uses `unsafe`.

[`std::sync::Once`]: https://doc.rust-lang.org/std/sync/struct.Once.html

# F.A.Q.

**Should I use the sync or unsync flavor?**

Because Rust compiler checks thread safety for you, it’s impossible to
accidentally use `unsync` where `sync` is required. So, use `unsync` in
single-threaded code and `sync` in multi-threaded. It’s easy to switch
between the two if code becomes multi-threaded later.

At the moment, `unsync` has an additional benefit that reentrant
initialization causes a panic, which might be easier to debug than a
deadlock.

**Does this crate support async?**

No, but you can use
[`async_once_cell`](https://crates.io/crates/async_once_cell) instead.

**Does this crate support `no_std`?**

Yes, but with caveats. `OnceCell` is a synchronization primitive which
_semantically_ relies on blocking. `OnceCell` guarantees that at most one
`f` will be called to compute the value. If two threads of execution call
`get_or_init` concurrently, one of them has to wait.

Waiting fundamentally requires OS support. Execution environment needs to
understand who waits on whom to prevent deadlocks due to priority inversion.
You _could_ make code to compile by blindly using pure spinlocks, but the
runtime behavior would be subtly wrong.

Given these constraints, `once_cell` provides the following options:

- The `race` module provides similar, but distinct synchronization primitive
  which is compatible with `no_std`. With `race`, the `f` function can be
  called multiple times by different threads, but only one thread will win
  to install the value.
- `critical-section` feature (with a `-`, not `_`) uses `critical_section`
  to implement blocking.

**Can I bring my own mutex?**

There is [generic_once_cell](https://crates.io/crates/generic_once_cell) to
allow just that.

**Should I use `std::cell::OnceCell`, `once_cell`, or `lazy_static`?**

If you can use `std` version (your MSRV is at least 1.70, and you don’t need
extra features `once_cell` provides), use `std`. Otherwise, use `once_cell`.
Don’t use `lazy_static`.

# Related crates

* Most of this crate’s functionality is available in `std` starting with
  Rust 1.70. See `std::cell::OnceCell` and `std::sync::OnceLock`.
* [double-checked-cell](https://github.com/niklasf/double-checked-cell)
* [lazy-init](https://crates.io/crates/lazy-init)
* [lazycell](https://crates.io/crates/lazycell)
* [mitochondria](https://crates.io/crates/mitochondria)
* [lazy_static](https://crates.io/crates/lazy_static)
* [async_once_cell](https://crates.io/crates/async_once_cell)
* [generic_once_cell](https://crates.io/crates/generic_once_cell) (bring
  your own mutex)