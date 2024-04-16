use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "stroke-0" => "stroke-width: 0;",
    "stroke-1" => "stroke-width: 1;",
    "stroke-2" => "stroke-width: 2;",
};