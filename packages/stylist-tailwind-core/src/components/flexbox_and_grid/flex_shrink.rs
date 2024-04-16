use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "shrink" => "flex-shrink: 1;",
    "shrink-0" => "flex-shrink: 0;",
};