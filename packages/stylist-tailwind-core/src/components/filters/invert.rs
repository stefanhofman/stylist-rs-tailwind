use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "invert-0" => "filter: invert(0);",
    "invert" => "filter: invert(100%);",
};