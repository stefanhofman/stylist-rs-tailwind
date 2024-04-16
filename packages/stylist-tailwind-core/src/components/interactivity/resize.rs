use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "resize-none" => "resize: none;",
    "resize-y" => "resize: vertical;",
    "resize-x" => "resize: horizontal;",
    "resize" => "resize: both;",
};