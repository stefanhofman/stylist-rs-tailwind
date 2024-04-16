use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "scroll-auto" => "scroll-behavior: auto;",
    "scroll-smooth" => "scroll-behavior: smooth;",
};