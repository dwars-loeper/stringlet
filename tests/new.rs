//! Test functionality of the `new` module.

use stringlet::prelude::*;

#[test]
fn new() {
    let s = UniStringlet::<0>::new();
    assert!(s.is_empty());
    let s = BiStringlet::<0>::new();
    assert!(s.is_empty());
    let s = BiStringlet::<1>::new();
    assert!(s.is_empty());
    let s = VarStringlet::<8>::new();
    assert!(s.is_empty());
    let s = PolyStringlet::<8>::new();
    assert!(s.is_empty());
}

#[test]
fn default() {
    let s: UniStringlet<0> = Default::default();
    assert!(s.is_empty());
    let s: BiStringlet<0> = Default::default();
    assert!(s.is_empty());
    let s: BiStringlet<1> = Default::default();
    assert!(s.is_empty());
    let s: VarStringlet = Default::default();
    assert!(s.is_empty());
    let s: PolyStringlet = Default::default();
    assert!(s.is_empty());
}

#[test]
fn from_string() {
    let s: PolyStringlet<4> = String::from("hey").try_into().unwrap();
    assert_eq!(s.as_ref(), "hey");
}

#[test]
fn from_long_str() {
    let s: VarStringlet<16> = "Rustacean".try_into().unwrap();
    assert_eq!(&s, "Rustacean");
}

#[test]
#[should_panic]
fn panics_when_too_long() {
    let _s: VarStringlet<2> = "hello world".try_into().unwrap();
}

#[test]
fn try_from() {
    let s = PolyStringlet::<8>::try_from("hello").unwrap();
    assert_eq!(s.as_ref(), "hello");
}

/// This takes a tuple of stringlets and converts them to various types. The middle ones shall succeed, the others don’t fit.
macro_rules! from_stringlet {
    ($slet:ident ($($short:ty),+) ($($ok:ty),+) ($($long:ty),*)) => {
        $(assert_eq!(<$short>::from_stringlet($slet).unwrap_err(), TooShort, "TooShort {} {:?}", stringify!($short), $slet);)+
        $(assert_eq!(<$ok>::from_stringlet($slet).unwrap(), $slet, "Ok {} {:?}", stringify!($ok), $slet);)+
        $(assert_eq!(<$long>::from_stringlet($slet).unwrap_err(), TooLong, "TooLong {} {:?}", stringify!($long), $slet);)*
    };
    (($($slet:tt,)+) $short:tt $ok:tt $long:tt) => {
        $(
            let slet = stringlet!$slet;
            from_stringlet!(slet $short $ok $long);
        )+
    };
}

#[test]
fn from_stringlet() {
    use stringlet::Error::{TooLong, TooShort};
    from_stringlet! {
        (
            (""),
            (v: ""),
            (v 1: ""),
            (v 2: ""),
            (b: ""),
            (b 1: ""),
            (p: ""),
            (p 1: ""),
            (p 2: ""),
        )
        (UniStringlet<1>, BiStringlet<2>)
        (UniStringlet<0>, VarStringlet<0>, VarStringlet<255>, BiStringlet<0>, BiStringlet<1>, PolyStringlet<0>, PolyStringlet<1>, PolyStringlet<64>)
        ()
    }
    from_stringlet! {
        (
            ("x"),
            (v: "x"),
            (v 1: "\0"),
            (v 1: "x"),
            (v 2: "x"),
            (v 3: "x"),
            (b: "x"),
            (b 2: "x"),
            (b 2: "\0"),
            (p: "x"),
            (p 2: "x"),
            (p 2: "\0"),
            (p 3: "x"),
            ("y"),
            (v: "y"),
            (v 2: "y"),
            (v 3: "y"),
            (b: "y"),
            (b 2: "y"),
            (p: "y"),
            (p 2: "y"),
            (p 3: "y"),
        )
        (UniStringlet<2>, BiStringlet<3>)
        (UniStringlet<1>, VarStringlet<1>, VarStringlet<255>, BiStringlet<1>, BiStringlet<2>, PolyStringlet<1>, PolyStringlet<2>, PolyStringlet<64>)
        (UniStringlet<0>, VarStringlet<0>, BiStringlet<0>, PolyStringlet<0>)
    }
    from_stringlet! {
        (
            (65: &"x".repeat(65)),
        )
        (UniStringlet<66>, BiStringlet<67>)
        (UniStringlet<65>, VarStringlet<65>, BiStringlet<65>)
        (UniStringlet<64>, VarStringlet<64>, BiStringlet<64>, PolyStringlet<64>)
    }
    from_stringlet! {
        (
            (256: &"x".repeat(256)),
        )
        (UniStringlet<257>, BiStringlet<258>)
        (UniStringlet<256>, BiStringlet<256>)
        (UniStringlet<255>, VarStringlet<255>, BiStringlet<255>)
    }
}
