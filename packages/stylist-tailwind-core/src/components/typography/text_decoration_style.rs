use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "decoration-solid" => "text-decoration-style: solid;",
    "decoration-double" => "text-decoration-style: double;",
    "decoration-dotted" => "text-decoration-style: dotted;",
    "decoration-dashed" => "text-decoration-style: dashed;",
    "decoration-wavy" => "text-decoration-style: wavy;",
};