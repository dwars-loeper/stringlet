//! `stringlet!()`

#[doc(hidden)]
#[macro_export]
macro_rules! stringlet_base {
    // get size parameter, if not yet present
    (param$params:tt  :  $($rest:tt)+) => {
        $crate::stringlet_base!(size$params  $($rest)+)
    };
    (param($kind:tt)  $size:tt:  $($rest:tt)+) => {
        $crate::stringlet_base!(size($kind $size)  $($rest)+)
    };

    // add default size or was given?
    (size($kind:tt)  [$str:expr $(, $($rest:tt)*)?]) => {{
        const STR: &::core::primitive::str = $str;
        $crate::stringlet_base!([$crate::stringlet_base!(const $kind { STR.len() } STR)] $($($rest)*)?)
    }};
    (size($kind:tt $size:tt)  [$str:literal $(, $($rest:tt)*)?]) => {
        $crate::stringlet_base!([$crate::stringlet_base!(const $kind $size $str)] $($($rest)*)?)
    };
    (size($kind:tt $size:tt)  [$str:expr $(, $($rest:tt)*)?]) => {
        $crate::stringlet_base!([$crate::stringlet_base!(dyn $kind $size $str)] $($($rest)*)?)
    };

    (size($kind:tt)  $str:expr) => {
        const {
            const STR: &::core::primitive::str = $str;
            $crate::stringlet_base!(dyn $kind { STR.len() } STR)
        }
    };
    (size($kind:tt $size:tt)  $str:literal) => {
        $crate::stringlet_base!(const $kind $size $str)
    };
    (size($kind:tt $size:tt)  $str:expr) => {
        $crate::stringlet_base!(dyn $kind $size $str)
    };

    (dyn _ _ $str:expr) => {
        $crate::Stringlet::<_, _>::_from_macro($str)
    };
    (dyn $kind:tt $size:tt $str:expr) => {
        $crate::Stringlet::<$crate::$kind, $size>::_from_macro($str)
    };

    (const $kind:tt $size:tt $str:expr) => {
        const {
            $crate::stringlet_base!(dyn $kind $size $str)
        }
    };

    ([$($done:expr),*]  $str:literal $(, $($rest:tt)*)?) => {
        $crate::stringlet_base!([$($done),*, $crate::stringlet_base!(const _ _ $str)]  $($($rest)*)?)
    };
    /* ([$($done:expr),*]  $($str:literal),+ $(, $($rest:tt)*)?) => {
        $crate::stringlet_base!([$($done),*, $($crate::stringlet_base!(const _ _ $str)),+]  $($($rest)*)?)
    }; */
    ([$($done:expr),*]  $str:expr $(, $($rest:tt)*)?) => {
        $crate::stringlet_base!([$($done),*, $crate::stringlet_base!(dyn _ _ $str)]  $($($rest)*)?)
    };
    ($array:expr) => {
        $array
    };
}

/**
Turn a `str` expression into the smallest `Stringlet` that can contain it.
Or turn `[str, …]` into an array of the smallest `Stringlet` that can contain them.
You can explicitly ask for other kinds of stringlet. By default `SIZE` is the
length of the 1st `str` parameter, in which case that parameter must be `const`.
You can also give the size explicitly, or have it inferred from context along
with the kind.

You can test whether this managed to make your `Stringlet` be `const` by assigning it to a `const`. Where this is not
possible, because you are passing in dynamic data, beware that this will panic, if the SIZE is wrong!

The optional configuration is kind and/or size, if present followed by a colon:

|Long Spec \| |Short Spec \| |Type |
|:---|:---|:---|
| | |`UniStringlet<param.len()>`|
|SIZE: | |`UniStringlet<SIZE>`|
|var: |v: |`VarStringlet<param.len()>`|
|var SIZE: |v SIZE: |`VarStringlet<SIZE>`|
|bi: |t: |`BiStringlet<param.len()>`|
|bi SIZE: |t SIZE: |`BiStringlet<SIZE>`|
|poly: |s: |`PolyStringlet<param.len()>`|
|poly SIZE: |s SIZE: |`PolyStringlet<SIZE>`|
|_: | |`Stringlet<_, _>`|

These are equivalent:
```
# use crate::stringlet::{UniStringlet, stringlet};
let s1 = stringlet!("abc");
let s2: UniStringlet<3> = stringlet!(_: "abc");
const S3: UniStringlet<3> = stringlet!(3: " abc ".trim_ascii());
assert_eq!(s1, s2);
assert_eq!(s2, S3);
```
As are these:
```
# use crate::stringlet::{VarStringlet, stringlet};
let s1 = stringlet!(var: ["abcdefghijklmno", "xyz"]);
let s2: [VarStringlet<15>; 2] = stringlet!(_: [&String::from("abcdefghijklmno"), "xyz"]);
const S3: [VarStringlet<15>; 2] = stringlet!(v 15: [concat!("abcdefgh", 'i', "jklmno"), "xyz"]);
assert_eq!(s1, s2);
assert_eq!(s2, S3);
```

You can also provide several comma separated parameter-sets to get a tuple of stringlets:
```
# use crate::stringlet::{UniStringlet, stringlet};
let tuple = stringlet!(("abc"), (3: " abc ".trim_ascii()));
assert_eq!(tuple.0, tuple.1);
```
*/
#[macro_export]
macro_rules! stringlet {
    (_:  $($rest:tt)+) => {
        $crate::stringlet_base!(size(_ _)  $($rest)+)
    };

    (var  $($rest:tt)+) => {
        $crate::stringlet_base!(param(Var)  $($rest)+)
    };
    (v  $($rest:tt)+) => {
        $crate::stringlet!(var  $($rest)+)
    };

    (bi  $($rest:tt)+) => {
        $crate::stringlet_base!(param(Bi)  $($rest)+)
    };
    (b  $($rest:tt)+) => {
        $crate::stringlet!(bi  $($rest)+)
    };

    (poly $($rest:tt)+) => {
        $crate::stringlet_base!(param(Poly)  $($rest)+)
    };
    (p  $($rest:tt)+) => {
        $crate::stringlet!(poly  $($rest)+)
    };

    ($(($($slet:tt)+)),* $(,)?) => {
        ($($crate::stringlet!($($slet)+)),*)
    };

    ($size:tt:  $($rest:tt)+) => {
        $crate::stringlet_base!(size(Uni $size)  $($rest)+)
    };
    ($($rest:tt)+) => {
        $crate::stringlet_base!(size(Uni)  $($rest)+)
    };
}

#[cfg(doctest)]
mod doctests {
    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        let _: UniStringlet<2> = stringlet!(_: ""); // 0 is too short
    ```
    */
    fn macro_fixed_2_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        let _: [UniStringlet<2>; 2] = stringlet!(_: ["ok", ""]); // 0 is too short
    ```
    */
    fn macro_fixed_array_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(2: ""); // 0 is too short
    ```
    */
    fn macro_fixed_2_0_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(2: "more"); // 4 is too long
    ```
    */
    fn macro_fixed_2_4_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(var: "0123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_123456789_12345");
    ```
    */
    fn macro_var_256_compile_fail() {} // VarStringlet<256> is too long

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(bi 2: ""); // 0 is too short
    ```
    */
    fn macro_trim_2_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(bi: ["ok", ""]); // 0 is too short
    ```
    */
    fn macro_trim_array_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(bi 2: ["ok", ""]); // 0 is too short
    ```
    */
    fn macro_trim_array_2_compile_fail() {}

    /**
    ```compile_fail
    # use crate::stringlet::stringlet;
        println!("FAILED should not see this");
        stringlet!(poly: "0123456789_123456789_123456789_123456789_123456789_123456789_1234"); // 65 is too long
    ```
    */
    fn macro_slim_65_compile_fail() {} // PolyStringlet<65> is too long
}

#[cfg(test)]
mod tests {
    fn cmp<Slet: core::fmt::Debug>(slet: Slet, str: &str) {
        assert_eq!(format!("{:?}", slet), str);
    }
    #[test]
    fn all_types() {
        cmp(stringlet!("aha"), "UniStringlet<3> { str: \"aha\" }");
        cmp(stringlet!(var: "aha"), "VarStringlet<3> { str: \"aha\" }");
        cmp(stringlet!(bi: "aha"), "BiStringlet<3> { str: \"aha\" }");
        cmp(stringlet!(poly: "aha"), "PolyStringlet<3> { str: \"aha\" }");

        cmp(stringlet!(5: "aha45"), "UniStringlet<5> { str: \"aha45\" }");
        cmp(stringlet!(var 5: "aha"), "VarStringlet<5> { str: \"aha\" }");
        cmp(stringlet!(bi 4: "aha"), "BiStringlet<4> { str: \"aha\" }");
        cmp(
            stringlet!(poly 5: "aha"),
            "PolyStringlet<5> { str: \"aha\" }",
        );

        cmp(
            stringlet!(["aha", "oho"]),
            "[UniStringlet<3> { str: \"aha\" }, UniStringlet<3> { str: \"oho\" }]",
        );
        cmp(
            stringlet!(var: ["aha", "oh"]),
            "[VarStringlet<3> { str: \"aha\" }, VarStringlet<3> { str: \"oh\" }]",
        );
        cmp(
            stringlet!(bi: ["aha", "oh"]),
            "[BiStringlet<3> { str: \"aha\" }, BiStringlet<3> { str: \"oh\" }]",
        );
        cmp(
            stringlet!(poly: ["aha", "oh"]),
            "[PolyStringlet<3> { str: \"aha\" }, PolyStringlet<3> { str: \"oh\" }]",
        );

        cmp(
            stringlet!(3: ["aha", "oho"]),
            "[UniStringlet<3> { str: \"aha\" }, UniStringlet<3> { str: \"oho\" }]",
        );
        cmp(
            stringlet!(var 5: ["aha", "oh"]),
            "[VarStringlet<5> { str: \"aha\" }, VarStringlet<5> { str: \"oh\" }]",
        );
        cmp(
            stringlet!(bi 4: ["aha", "oho"]),
            "[BiStringlet<4> { str: \"aha\" }, BiStringlet<4> { str: \"oho\" }]",
        );
        cmp(
            stringlet!(poly 5: ["aha", "oh"]),
            "[PolyStringlet<5> { str: \"aha\" }, PolyStringlet<5> { str: \"oh\" }]",
        );

        cmp(
            stringlet!(("aha"), (bi 4: "aha"), (poly 5: ["aha", "oh"])),
            "(UniStringlet<3> { str: \"aha\" }, BiStringlet<4> { str: \"aha\" }, [PolyStringlet<5> { str: \"aha\" }, PolyStringlet<5> { str: \"oh\" }])",
        );
    }
}
