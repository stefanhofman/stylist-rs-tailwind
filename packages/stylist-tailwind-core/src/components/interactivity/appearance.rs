use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "appearance-none" => "appearance: none;",
    "appearance-auto" => "appearance: auto;",
};