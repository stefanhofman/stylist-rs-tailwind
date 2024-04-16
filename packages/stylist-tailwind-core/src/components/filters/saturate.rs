use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "saturate-0" => "filter: saturate(0);",
    "saturate-50" => "filter: saturate(.5);",
    "saturate-100" => "filter: saturate(1);",
    "saturate-150" => "filter: saturate(1.5);",
    "saturate-200" => "filter: saturate(2);",
};