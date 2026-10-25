//! Test functionality of the `cmp` module.

use stringlet::prelude::*;

/// Compare all kinds and various sizes with self and every other
macro_rules! cmp_all {
    ($fn:ident: $op:tt) => {
        #[test]
        fn $fn() {
            // These combinatorics are very heavy, specially under BorrowSanitizer.
            cmp_all!($op:
                u = (""),
                v = (v: ""),
                v1 = (v 1: ""),
                v2 = (v 2: ""),
                b = (b: ""),
                b1 = (b 1: ""),
                p = (p: ""),
                p1 = (p 1: ""),
                p2 = (p 2: ""),
                ux = ("x"),
                vx = (v: "x"),
                v10 = (v 1: "\0"),
                v2x = (v 2: "x"),
                v3x= (v 3: "x"),
                bx = (b: "x"),
                b2x = (b 2: "x"),
                b20 = (b 2: "\0"),
                px = (p: "x"),
                p2x = (p 2: "x"),
                p20 = (p 2: "\0"),
                p3x = (p 3: "x"),
                uy = ("y"),
                vy = (v: "y"),
                v2y = (v 2: "y"),
                v3y = (v 3: "y"),
                by = (b: "y"),
                b2y = (b 2: "y"),
                py = (p: "y"),
                p2y = (p 2: "y"),
                p3y = (p 3: "y"),
                uxy = ("xy"),
                vxy = (v: "xy"),
                v3xy = (v 3: "xy"),
                // v4xy = (v 4: "xy"),
                bxy = (b: "xy"),
                b3xy = (b 3: "xy"),
                pxy = (p: "xy"),
                p3xy = (p 3: "xy"),
                /* These do not really improve coverage, but explode combinatorics:
                p4xyz = (p 4: "xy"),
                uxyz = ("xyz"),
                vxyz = (v: "xyz"),
                v4xyz = (v 4: "xyz"),
                v5xyz = (v 5: "xyz"),
                bxyz = (b: "xyz"),
                b4xyz = (b 4: "xyz"),
                pxyz = (p: "xyz"),
                p4xyz = (p 4: "xyz"),
                p5xyz = (p 5: "xyz"), */
            );
        }
    };
    (@ $op:tt: $v:ident = $s:tt, $($rest:tt)*) => {
        let $v = stringlet!$s;
        {
            let c = const { stringlet!$s };
            // This macro is expensive, so gen it once for tuples of cases.
            assert_eq!(
                (
                    $v $op $v.clone(),
                    //$v.as_str() $op $v,
                    $v $op $v.as_str(),
                    $v $op c,
                ),
                (
                    $v.as_str() $op $v.as_str(),
                    $v.as_str() $op $v.as_str(),
                    $v.as_str() $op c.as_str(),
                ),
                "{:#?} {c:#?}", $v);
        }
        cmp_all!(@ $op: $($rest)*);
    };
    (@ $op:tt:) => {};
    (@@ $op:tt: $a:ident = $_a:tt, $($b:ident = $_b:tt,)+) => {
        $(
            let ab = $a.as_str() $op $b.as_str();
            let ba = $b.as_str() $op $a.as_str();
            assert_eq!(
                (
                    $a $op $b,
                    //$a.as_str() $op $b,
                    $a $op $b.as_str(),
                    $b $op $a,
                    //$b.as_str() $op $a,
                    $b $op $a.as_str(),
                ),
                (ab, ab, ba, ba),
                "{:#?} {:#?}", $a, $b);
        )+
        cmp_all!(@@ $op: $($b = $_b,)+);
    };
    (@@ $_:tt: $a:ident = $_a:tt,) => {};

    ($op:tt: $($rest:tt)+) => {
        cmp_all!(@ $op: $($rest)+);
        cmp_all!(@@ $op: $($rest)+);
    };
}

cmp_all!(eq: ==);
cmp_all!(lt: <);
cmp_all!(le: <=);
cmp_all!(gt: >);
cmp_all!(ge: >=);

#[test]
#[allow(clippy::op_ref)]
fn cmp_wrappers() {
    let slet = stringlet!("wow");
    let string = String::from("wow");
    assert!(slet == "wow");
    assert!(slet >= "wow");

    assert!(slet == &slet);
    assert!(slet >= &slet);
    assert!(slet.cmp(&slet) == std::cmp::Ordering::Equal);

    assert!(slet == string);
    assert!(slet >= string);

    assert!(slet == &string);
    assert!(slet >= &string);
}
