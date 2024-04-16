use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "isolate" => "isolation: isolate;",
    "isolation-auto" => "isolation: auto;",
};