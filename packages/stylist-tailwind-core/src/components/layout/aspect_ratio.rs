use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "aspect-auto" => "aspect-ratio: auto;",
    "aspect-square" => "aspect-ratio: 1 / 1;",
    "aspect-video" => "aspect-ratio: 16 / 9;",
};