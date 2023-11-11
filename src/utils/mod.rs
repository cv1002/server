#![allow(unused)]

pub mod async_collect;
pub mod durations;
pub mod inspect;
pub mod intos;
pub mod lazy;
pub mod logger;
pub mod transform;

/// Simulation of try block...
/// # Examples
///
/// ```rust
/// use utils::try_do;
///
/// let ret = try_do(|| {
///     None?;
///     Some(1)
/// });
/// assert_eq!(ret, None);
/// ```
#[inline(always)]
pub fn try_do<R>(f: impl FnOnce() -> R) -> R {
    f()
}
