use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "antialiased" => "-webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale;",
    "subpixel-antialiased" => "-webkit-font-smoothing: auto; -moz-osx-font-smoothing: auto;",
};