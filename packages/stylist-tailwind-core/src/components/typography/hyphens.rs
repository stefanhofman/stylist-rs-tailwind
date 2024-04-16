use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "hyphens-none" => "hyphens: none;",
    "hyphens-manual" => "hyphens: manual;",
    "hyphens-auto" => "hyphens: auto;",
};