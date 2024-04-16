use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "box-border" => "box-sizing: border-box;",
    "box-content" => "box-sizing: content-box;",
};