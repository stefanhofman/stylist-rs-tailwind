use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "grayscale-0" => "filter: grayscale(0);",
    "grayscale" => "filter: grayscale(100%);",
};