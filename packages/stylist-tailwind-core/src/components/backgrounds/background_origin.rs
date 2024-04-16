use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "bg-origin-border" => "background-origin: border-box;",
    "bg-origin-padding" => "background-origin: padding-box;",
    "bg-origin-content" => "background-origin: content-box;",
};