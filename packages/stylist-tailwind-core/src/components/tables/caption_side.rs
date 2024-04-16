use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "caption-top" => "caption-side: top;",
    "caption-bottom" => "caption-side: bottom;",
};