use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "flex-wrap" => "flex-wrap: wrap;",
    "flex-wrap-reverse" => "flex-wrap: wrap-reverse;",
    "flex-nowrap" => "flex-wrap: nowrap;",
};