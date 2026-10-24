//! Configuration for stringlet

use super::Stringlet;

pub trait Kind {
    type ExtraLen: Copy + Clone;
    const UNI: bool = false;
    const VAR: bool = false;
    const BI: bool = false;
    const POLY: bool = false;
    const NAME: &str;
    const ABBR: u8;
}

/// Configure constructors of `Stringlet` to have only valid generic parameters.
#[diagnostic::on_unimplemented(
    message = "`VarStringlet<{SIZE}>` or `PolyStringlet<{SIZE}>` has excessive SIZE",
    label = "SIZE must be `0..=255` or `0..=64`",
    note = "`VarStringlet` cannot be longer than 255 bytes. Consider using `String`!",
    note = "`PolyStringlet` cannot be longer than 64 bytes. Consider using `VarStringlet`!"
)]
#[doc(hidden)]
pub trait Config<Kind, const SIZE: usize = 16> {}

impl<const SIZE: usize> Config<Uni, SIZE> for UniStringlet<SIZE> {}

#[diagnostic::on_unimplemented(
    message = "`VarStringlet<{SIZE}>` has excessive SIZE",
    label = "SIZE must be `0..=255`",
    note = "`VarStringlet` cannot be longer than 255 bytes. Consider using `String`!"
)]
pub trait VarConfig<const SIZE: usize> {}
// VarConfig implemented by macro below
impl<const SIZE: usize> Config<Var, SIZE> for VarStringlet<SIZE> where Self: VarConfig<SIZE> {}

impl<const SIZE: usize> Config<Bi, SIZE> for BiStringlet<SIZE> {}

#[diagnostic::on_unimplemented(
    message = "`PolyStringlet<{SIZE}>` has excessive SIZE",
    label = "SIZE must be `0..=64`",
    note = "`PolyStringlet` cannot be longer than 64 bytes. Consider using `VarStringlet`!"
)]
pub trait PolyConfig<const SIZE: usize> {}
// PolyConfig implemented by macro below
impl<const SIZE: usize> Config<Poly, SIZE> for PolyStringlet<SIZE> where Self: PolyConfig<SIZE> {}

macro_rules! config {
    (@@ $stringlet:ident $kind_config:ident $($size:tt)+) => {
        $(
            impl $kind_config<$size> for $stringlet<$size> {}
        )+
    };
    (@ $stringlet:ident) => {};
    (@ $stringlet:ident $kind_config:ident 64) => {
        config![@@ $stringlet $kind_config
            // for VarStringlet and PolyStringlet
            0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
            32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60
            61 62 63 64
        ];
    };
    (@ $stringlet:ident $kind_config:ident 255) => {
        config![@ $stringlet $kind_config 64];
        config![@@ $stringlet $kind_config
            // for VarStringlet
            65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93
            94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116
            117 118 119 120 121 122 123 124 125 126 127 128 129 130 131 132 133 134 135 136 137 138
            139 140 141 142 143 144 145 146 147 148 149 150 151 152 153 154 155 156 157 158 159 160
            161 162 163 164 165 166 167 168 169 170 171 172 173 174 175 176 177 178 179 180 181 182
            183 184 185 186 187 188 189 190 191 192 193 194 195 196 197 198 199 200 201 202 203 204
            205 206 207 208 209 210 211 212 213 214 215 216 217 218 219 220 221 222 223 224 225 226
            227 228 229 230 231 232 233 234 235 236 237 238 239 240 241 242 243 244 245 246 247 248
            249 250 251 252 253 254 255
        ];
    };
    ($msg1:literal $msg2:literal $stringlet:ident $kind:ident $const:ident $extra_len:tt $($kind_config:ident $size:tt)?) => {
        #[derive(Copy, Clone)]
        pub enum $kind {}

        impl Kind for $kind {
            const $const: bool = true;
            type ExtraLen = $extra_len;
            const NAME: &str = stringify!($stringlet);
            const ABBR: u8 = stringify!($kind).as_bytes()[0]; // TODO NAME[0]
        }

        #[doc = concat!($msg1, " length kind of `Stringlet`", $msg2)]
        $(#[doc = concat!("\n\nWhen instatiating this with a generic `SIZE`, you need to add this bound:\n```ignore\nwhere ",
            stringify!($stringlet), "<SIZE>: ", stringify!($kind_config), "<SIZE>\n```")])?
        pub type $stringlet<const SIZE: usize = 16> = Stringlet<$kind, SIZE>;
        $(config![@ $stringlet $kind_config $size];)?
    };
}

config!("Uni" ", i.e. bounds for array access are compiled in, hence it is fast.

This is also produced by [`stringlet!(…)`](stringlet!()) without a kind specifier."
    UniStringlet Uni UNI ());
config!("Variable" ", with one extra byte for the length.
    Speed differs for some content processing, where SIMD gives an advantage for multiples of some power of 2, e.g.
    `VarStringlet<32>`. While for copying the advantage can be at one less, e.g. `VarStringlet<31>`. Size must be `0..=255`.

This is also produced by [`stringlet!(…)`](stringlet!()) with a kind specifier of `var` or `v`."
    VarStringlet Var VAR u8 VarConfig 255);
config!("Trimmed" ", which optionally bis one last byte, useful for codes
    with minimal length variation like [ISO 639](https://www.iso.org/iso-639-language-code). This is achieved by tagging
    an unused last byte with a UTF-8 niche. The length gets calculated branchlessly with very few ops.

This is also produced by [`stringlet!(…)`](stringlet!()) with a kind specifier of `bi` or `t`."
    BiStringlet Bi BI ());
config!("Poly variable" ", uses a UTF-8 niche: It projects the length into 6 bits of the last byte, when content is less
    than full size. Length must be `0..=64`. Though it is done branchlessly, there are a few more ops for length calculation.
    Hence this is the slowest, albeit by a small margin.

This is also produced by [`stringlet!(…)`](stringlet!()) with a kind specifier of `poly` or `s`."
    PolyStringlet Poly POLY () PolyConfig 64);
