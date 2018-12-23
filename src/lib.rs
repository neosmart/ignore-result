#![no_std]

#[cfg(test)]
mod tests;

/// The `Ignore` trait is used to consume the result of a function call
/// in cases where a function's success or failure is irrelevant, i.e.
/// in "best effort" scenarios. By consuming the original `Result` and
/// swallowing its `Ok()` result, it guarantees correctness.
///
/// A call to [`Ignore::ignore()`] avoids compiler warnings about unused
/// results, without requiring a possibly unsafe call to `.unwrap()` if
/// the success of the preceding function call is not guaranteed.
pub trait Ignore {
    fn ignore(&self) -> () {}
}

impl<T, E> Ignore for Result<T, E> {}
