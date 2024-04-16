use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "auto-rows-auto" => "grid-auto-rows: auto;",
    "auto-rows-min" => "grid-auto-rows: min-content;",
    "auto-rows-max" => "grid-auto-rows: max-content;",
    "auto-rows-fr" => "grid-auto-rows: minmax(0, 1fr);",
};