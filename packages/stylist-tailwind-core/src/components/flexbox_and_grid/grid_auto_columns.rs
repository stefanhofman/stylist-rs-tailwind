use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "auto-cols-auto" => "grid-auto-columns: auto;",
    "auto-cols-min" => "grid-auto-columns: min-content;",
    "auto-cols-max" => "grid-auto-columns: max-content;",
    "auto-cols-fr" => "grid-auto-columns: minmax(0, 1fr);",
};