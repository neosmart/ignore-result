# `Result-Ignore` for rust
_Safely ignore errors in function return values when the result isn't critical_

This crate adds a `.ignore()` function to `Result` instances that ignores both the `Ok` and `Err`
variants of the result, silencing compiler warnings about unused errors without needing to resort to
empty `match` blocks or panicking in case you call `.unwrap()` on what turns out to be an error.

## Why do you need this?

There are cases involving correct error handling where it is desirable to call a method and "keep on
keepin' on" even if it does not succeed, something often seen in FFI when interfacing with C APIs or
on embedded systems.

As a concrete example, imagine that you have trapped an error and need to bubble it up to the
caller, but first, you need to do your best to clean up some resource allocations, etc.

If you did this:

```rust
fn my_func() -> Result<(), E> {
    let foo = something_important()?;
    if foo.is_err() {
        // Oh no! We can't continue, need to report this error,
        // but first, try to clean up some state
        cleanup()?;
    }
    return foo;
}
```

The original error contained in `foo` would be lost if `cleanup()` failed (as the `?` would trigger
an early abort with the return value of `cleanup()` and not the original error `Err(e)` in `foo`).

And if you omitted the `?` from `cleanup()?`, the compiler would warn you that there's a result
that's not checked/used, because `cleanup()` can fail.

This is a "best effort" scenario where you _hope_ that the function will succeed, there's nothing
you can do if it doesn't, and there's a higher priority result (error) that should be persisted even
if this step fails.

So you can either write this:

```rust
let foo = something_import();
if foo.is_err() {
    match cleanup() {
        _ => {}
    };

    return foo;
}
```

Or use this crate and write this instead:

```rust
use ignore_result::Ignore;

if foo.is_err() {
    cleanup.ignore();
    return foo;
}
```

and be on your merry way.

## Why is this safe?

Calling `foo().ignore()` returns `()` regardless of whether `foo()` evaluated to `Ok(_)` or `Err(_)`
-- meaning it guarantees that you are not (incorrectly) relying on an `Ok(_)` result. In an ideal
world, the implementation of `fn ignore(..)` would be `fn ignore(Self)` so that the `Result<_, _>`
is fully consumed (i.e. there's no way to get at the original contents after calling
`result.ignore()`) but unfortunately as a result of language limitations pertaining to the use of
traits to extend existing objects, that is not possible.
