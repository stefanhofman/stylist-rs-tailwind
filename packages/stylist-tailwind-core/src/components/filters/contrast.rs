use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "contrast-0" => "filter: contrast(0);",
    "contrast-50" => "filter: contrast(.5);",
    "contrast-75" => "filter: contrast(.75);",
    "contrast-100" => "filter: contrast(1);",
    "contrast-125" => "filter: contrast(1.25);",
    "contrast-150" => "filter: contrast(1.5);",
    "contrast-200" => "filter: contrast(2);",
};