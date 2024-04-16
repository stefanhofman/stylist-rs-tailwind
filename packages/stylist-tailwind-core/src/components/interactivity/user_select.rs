use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "select-none" => "user-select: none;",
    "select-text" => "user-select: text;",
    "select-all" => "user-select: all;",
    "select-auto" => "user-select: auto;",
};