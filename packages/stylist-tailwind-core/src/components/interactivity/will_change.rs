use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "will-change-auto" => "will-change: auto;",
    "will-change-scroll" => "will-change: scroll-position;",
    "will-change-contents" => "will-change: contents;",
    "will-change-transform" => "will-change: transform;",
};