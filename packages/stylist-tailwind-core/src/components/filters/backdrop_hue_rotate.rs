use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "backdrop-hue-rotate-0" => "backdrop-filter: hue-rotate(0deg);",
    "backdrop-hue-rotate-15" => "backdrop-filter: hue-rotate(15deg);",
    "backdrop-hue-rotate-30" => "backdrop-filter: hue-rotate(30deg);",
    "backdrop-hue-rotate-60" => "backdrop-filter: hue-rotate(60deg);",
    "backdrop-hue-rotate-90" => "backdrop-filter: hue-rotate(90deg);",
    "backdrop-hue-rotate-180" => "backdrop-filter: hue-rotate(180deg);",
};