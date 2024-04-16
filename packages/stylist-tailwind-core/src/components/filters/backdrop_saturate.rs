use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "backdrop-saturate-0" => "backdrop-filter: saturate(0);",
    "backdrop-saturate-50" => "backdrop-filter: saturate(.5);",
    "backdrop-saturate-100" => "backdrop-filter: saturate(1);",
    "backdrop-saturate-150" => "backdrop-filter: saturate(1.5);",
    "backdrop-saturate-200" => "backdrop-filter: saturate(2);",
};