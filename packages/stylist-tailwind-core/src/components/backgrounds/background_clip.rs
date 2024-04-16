use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "bg-clip-border" => "background-clip: border-box;",
    "bg-clip-padding" => "background-clip: padding-box;",
    "bg-clip-content" => "background-clip: content-box;",
    "bg-clip-text" => "background-clip: text;",
};