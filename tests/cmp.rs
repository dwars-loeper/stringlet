//! Test functionality of the `cmp` module.

use stringlet::prelude::*;

/// Compare all kinds and various sizes with self and every other
macro_rules! cmp_all {
    ($fn:ident: $op:tt) => {
        #[test]
        fn $fn() {
            cmp_all!($op:
                (""),
                (v: ""),
                (v 1: ""),
                (v 2: ""),
                (b: ""),
                (b 1: ""),
                (p: ""),
                (p 1: ""),
                (p 2: ""),
                ("x"),
                (v: "x"),
                (v 1: "\0"),
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
                ("xy"),
                (v: "xy"),
                (v 3: "xy"),
                //(v 4: "xy"),
                (b: "xy"),
                (b 3: "xy"),
                (p: "xy"),
                (p 3: "xy"),
                /* These do not really improve coverage, but explode combinatorics:
                (p 4: "xy"),
                ("xyz"),
                (v: "xyz"),
                (v 4: "xyz"),
                (v 5: "xyz"),
                (b: "xyz"),
                (b 4: "xyz"),
                (p: "xyz"),
                (p 4: "xyz"),
                (p 5: "xyz"), */
            );
        }
    };
    ($op:tt: $a:tt, $($rest:tt,)+) => {
        let a = stringlet!$a;
        assert_eq!(a $op a.clone(), a.as_str() $op a.as_str(), "{a:#?}");
        //assert_eq!(a.as_str() $op a, a.as_str() $op a.as_str(), "{a:#?}");
        assert_eq!(a $op a.as_str(), a.as_str() $op a.as_str(), "{a:#?}");
        let ac = const { stringlet!$a };
        assert_eq!(a $op ac, a.as_str() $op ac.as_str(), "{a:#?} {ac:#?}");
        $(
            let b = stringlet!$rest;
            assert_eq!(a $op b, a.as_str() $op b.as_str(), "{a:#?} {b:#?}");
            //assert_eq!(a.as_str() $op b, a.as_str() $op b.as_str(), "{a:#?} {b:#?}");
            assert_eq!(a $op b.as_str(), a.as_str() $op b.as_str(), "{a:#?} {b:#?}");
            assert_eq!(b $op a, b.as_str() $op a.as_str(), "{a:#?} {b:#?}");
            //assert_eq!(b.as_str() $op a, b.as_str() $op a.as_str(), "{a:#?} {b:#?}");
            assert_eq!(b $op a.as_str(), b.as_str() $op a.as_str(), "{a:#?} {b:#?}");
        )+
        cmp_all!($op: $($rest,)+);
    };
    ($op:tt: $a:tt,) => {};
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
