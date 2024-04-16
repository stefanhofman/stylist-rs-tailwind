use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "clear-start" => "clear: inline-start;",
    "clear-end" => "clear: inline-end;",
    "clear-left" => "clear: left;",
    "clear-right" => "clear: right;",
    "clear-both" => "clear: both;",
    "clear-none" => "clear: none;",
};