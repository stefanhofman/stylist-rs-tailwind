use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "pointer-events-none" => "pointer-events: none;",
    "pointer-events-auto" => "pointer-events: auto;",
};