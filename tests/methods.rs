//! Test functionality of the `methods` module.

use stringlet::prelude::*;

#[test]
fn big() {
    let _f: UniStringlet<1024>;
    let _v: VarStringlet<255>;
    let _s: PolyStringlet<64>;
}

#[test]
fn as_str() -> stringlet::Result<()> {
    let f: UniStringlet<7> = "A123456".try_into()?;
    assert_eq!(f.as_str(), "A123456");
    let v: VarStringlet = "A123456".try_into()?;
    assert_eq!(v.as_str(), "A123456");
    let s: PolyStringlet = "A123456".try_into()?;
    assert_eq!(s.as_str(), "A123456");
    Ok(())
}

fn all_lengths<const SIZE: usize>()
where
    VarStringlet<SIZE>: stringlet::VarConfig<SIZE>,
    PolyStringlet<SIZE>: stringlet::PolyConfig<SIZE>,
{
    let str64s: [&str; 3] = [
        "0123456789_123456789_123456789_123456789_123456789_123456789_123",
        str::from_utf8(&[0; 64]).unwrap(),
        str::from_utf8(&[0x7f_u8; 64]).unwrap(),
    ];
    let uni: UniStringlet<SIZE> = (&str64s[0][..SIZE]).try_into().unwrap();
    assert_eq!(uni.is_empty(), SIZE == 0);
    assert_eq!(uni.len(), SIZE);
    for len in 0..=SIZE {
        let str: VarStringlet<SIZE> = (&str64s[0][..len]).try_into().unwrap();
        assert_eq!(str.is_empty(), len == 0);
        assert_eq!(str.len(), len);
        for str64 in str64s {
            if len >= const { SIZE.saturating_sub(1) } {
                let str: BiStringlet<SIZE> = (&str64[..len]).try_into().unwrap();
                assert_eq!(str.is_empty(), len == 0);
                assert_eq!(str.len(), len);
            }
            let str: PolyStringlet<SIZE> = (&str64[..len]).try_into().unwrap();
            assert_eq!(str.is_empty(), len == 0);
            assert_eq!(str.len(), len);
        }
    }
}
#[test]
fn len() {
    macro_rules! all_lengths {
        ($($size:literal),+) => {
            $(all_lengths::<$size>();)+
        };
    }
    all_lengths![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    ];
}

#[test]
fn empty() {
    assert!(stringlet!("").is_empty());
    assert!(!stringlet!("a").is_empty());
    assert!(!stringlet!("ab").is_empty());

    assert!(stringlet!(bi: "").is_empty());
    assert!(stringlet!(bi 1: "").is_empty());
    assert!(!stringlet!(bi: "a").is_empty());
    assert!(!stringlet!(bi 2: "a").is_empty());
    assert!(!stringlet!(bi: "ab").is_empty());
    assert!(!stringlet!(bi 3: "ab").is_empty());

    assert!(stringlet!(var: "").is_empty());
    assert!(stringlet!(var 1: "").is_empty());
    assert!(stringlet!(var 2: "").is_empty());
    assert!(!stringlet!(var: "a").is_empty());
    assert!(!stringlet!(var: "ab").is_empty());

    assert!(stringlet!(poly: "").is_empty());
    assert!(stringlet!(poly 1: "").is_empty());
    assert!(stringlet!(poly 2: "").is_empty());
    assert!(!stringlet!(poly: "a").is_empty());
    assert!(!stringlet!(poly: "ab").is_empty());
}

#[test]
fn try_into() {
    let _: PolyStringlet<1> = stringlet!("").try_into().unwrap();
    let x: Result<PolyStringlet<0>, _> = stringlet!("x").try_into();
    assert!(x.is_err());
}
