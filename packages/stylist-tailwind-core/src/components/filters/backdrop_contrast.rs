use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "backdrop-contrast-0" => "backdrop-filter: contrast(0);",
    "backdrop-contrast-50" => "backdrop-filter: contrast(.5);",
    "backdrop-contrast-75" => "backdrop-filter: contrast(.75);",
    "backdrop-contrast-100" => "backdrop-filter: contrast(1);",
    "backdrop-contrast-125" => "backdrop-filter: contrast(1.25);",
    "backdrop-contrast-150" => "backdrop-filter: contrast(1.5);",
    "backdrop-contrast-200" => "backdrop-filter: contrast(2);",
};