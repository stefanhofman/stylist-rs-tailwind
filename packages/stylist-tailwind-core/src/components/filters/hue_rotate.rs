use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "hue-rotate-0" => "filter: hue-rotate(0deg);",
    "hue-rotate-15" => "filter: hue-rotate(15deg);",
    "hue-rotate-30" => "filter: hue-rotate(30deg);",
    "hue-rotate-60" => "filter: hue-rotate(60deg);",
    "hue-rotate-90" => "filter: hue-rotate(90deg);",
    "hue-rotate-180" => "filter: hue-rotate(180deg);",
};