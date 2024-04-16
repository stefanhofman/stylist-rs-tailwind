use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "backdrop-invert-0" => "backdrop-filter: invert(0);",
    "backdrop-invert" => "backdrop-filter: invert(100%);",
};