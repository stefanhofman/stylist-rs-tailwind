use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "sepia-0" => "filter: sepia(0);",
    "sepia" => "filter: sepia(100%);",
};