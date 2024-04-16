use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "list-inside" => "list-style-position: inside;",
    "list-outside" => "list-style-position: outside;",
};