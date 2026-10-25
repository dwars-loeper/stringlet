#![doc = include_str!("../README.md")]
#![deny(clippy::alloc_instead_of_core, clippy::std_instead_of_core)]
#![allow(clippy::wildcard_imports)] // only our own

/* #![no_std]
extern crate alloc; */

use core::marker::PhantomData;

mod cmp;
mod config;
mod error;
mod fmt;
mod macros;
mod methods;
mod new;
mod refs;
mod traits;
mod workaround;

pub mod prelude {
    pub use crate::{BiStringlet, PolyStringlet, UniStringlet, VarStringlet, stringlet};
}

pub use config::{
    Bi, BiStringlet, Config, Kind, Poly, PolyConfig, PolyStringlet, Uni, UniStringlet, Var,
    VarConfig, VarStringlet,
};

pub use error::Error;

/// Specialized `Result` type with `stringlet::Error`
pub type Result<T> = core::result::Result<T, Error>;
pub(crate) use Error::*;

/**
Magic sauce for a UTF-8 hack: a byte containing two high bits is not a valid last byte.
Use this a a marker to distinguish whether we use the full length. Otherwise the lower bits contain
the length of the unused tail. At full length there is no tagged last byte, so we only need to encode
64 lengths. Which is where this crate’s length limit comes from.

To enable simple eq-test, always put the same value on all unused bytes! Counting from the end, i.e.
the length of the unused tail, makes the branchless implementation of `len()` more efficient.
*/
pub(crate) const TAG: u8 = 0b11_000000;

/** An inline String of varying size bounds, which can be handled like a primitive type.
This is the underlying type, which you would not use directly. Instead use one of:

- **[`UniStringlet`], [`stringlet!(…)`](stringlet!())**: This fixes length to size, i.e. bounds for array access are
  compiled in, hence fast.

- **[`VarStringlet`], `stringlet!(var …)`, `stringlet!(v …)`**: This adds one byte for the length – still pretty fast.
  Speed differs for some content processing, where SIMD gives an advantage for multiples of some power of 2, e.g.
  `VarStringlet<32>`. While for copying the advantage can be at one less, e.g. `VarStringlet<31>`. Size must be `0..=255`.

- **[`BiStringlet`], `stringlet!(bi …)`, `stringlet!(b …)`**: This can optionally bi one last byte, useful for codes
  with minimal length variation like [ISO 639](https://www.iso.org/iso-639-language-code). This is achieved by tagging
  an unused last byte with a UTF-8 niche. The length gets calculated branchlessly with very few ops.

- **[`PolyStringlet`], `stringlet!(poly …)`, `stringlet!(p …)`**: This uses the same UTF-8 niche, but fully: It projects
  the length into 6 bits of the last byte, when content is less than full size. Length must be `0..=64`. Though it is
  done branchlessly, there are a few more ops for length calculation. Hence this is the slowest, albeit by a small
  margin.

If you want to create either of `VarStringlet` or `PolyStringlet` generically, you must specify their bounds:
```
use stringlet::{VarStringlet, VarConfig, PolyStringlet, PolyConfig, Result};
fn create<const SIZE: usize>() -> Result<()>
where
    VarStringlet<SIZE>: VarConfig<SIZE>,
    PolyStringlet<SIZE>: PolyConfig<SIZE>,
{
    let _var = VarStringlet::<SIZE>::try_from("var")?;
    let _slim: PolyStringlet<SIZE> = "poly".try_into()?;
    Ok(())
}
_ = create::<5>();
```
*/
// Workaround for [u8; SIZE + Kind::EXTRA_LEN], as long as “generic parameters may not be used in const operations”
// Adapted from CAD97 https://internals.rust-lang.org/t/what-s-where-size-kind-extra/23987/9
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Stringlet<Kind: crate::Kind, const SIZE: usize> {
    /// The actual payload – if it is shorter than SIZE, its last bytes will be tagged.
    pub(crate) str: [u8; SIZE],
    pub(crate) extra_len: Kind::ExtraLen,
    pub(crate) _kind: PhantomData<Kind>,
}

/** Impl `SomeTrait` for `Self`, hiding repeated boilerplate caused by lack of nested impls.
```ignore
impl_for! { SomeTrait }
impl_for! { SomeTrait: impl_body }
impl_for! { <'a, Config, 2> SomeTrait<self2!()>: impl_body }
```
where `'a`, `Config`, `2` and impl_body are all optional, `2` meaning 2<sup>nd</sup> generic stringlet.
*/
macro_rules! impl_for {
    // Split this rule, otherwise compiler says optional <…> is ambiguous
    // $two should match nothing, but marks that 2 was matched.
    (<$($lt:lifetime $(,)?)? $($config:ident $(,)?)? $(2 $($two:literal)?)?> $trait:ty $(: $($rest:tt)+)?) => {
        impl_for!(@ $(2 $($two)?)? ($($lt)?) {$($config)?} [] $trait: $($($rest)+)?);
    };
    ($trait:ty $(: $($rest:tt)+)?) => {
        impl_for!(@ () {} [] $trait: $($($rest)+)?);
    };

    (@ ($($lt:lifetime)?) {$($config:ident)?} [$($gen:tt)*] $trait:ty: $($rest:tt)*) => {
        impl<$($lt,)? Kind: crate::Kind, const SIZE: usize, $($gen)*> $trait
        for Stringlet<Kind, SIZE>
        $(where Self: $config<Kind, SIZE>)?
        {
            $($rest)*
        }
    };
    (@ 2 $lt:tt $config:tt [] $trait:ty: $($rest:tt)*) => {
        impl_for!(@
            $lt
            $config
            [Kind2: crate::Kind, const SIZE2: usize]
            $trait:
            $($rest)*
        );
    };
}

/// A 2<sup>nd</sup> generic `Stringlet`.
macro_rules! self2 {
    () => {
        Stringlet<Kind2, SIZE2>
    };
}

pub(crate) use impl_for;
pub(crate) use self2;
