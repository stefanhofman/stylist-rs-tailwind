use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "backdrop-grayscale-0" => "backdrop-filter: grayscale(0);",
    "backdrop-grayscale" => "backdrop-filter: grayscale(100%);",
};