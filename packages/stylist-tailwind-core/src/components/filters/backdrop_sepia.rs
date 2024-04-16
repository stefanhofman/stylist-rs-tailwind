use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "backdrop-sepia-0" => "backdrop-filter: sepia(0);",
    "backdrop-sepia" => "backdrop-filter: sepia(100%);",
};