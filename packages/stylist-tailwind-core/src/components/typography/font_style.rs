use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "italic" => "font-style: italic;",
    "not-italic" => "font-style: normal;",
};