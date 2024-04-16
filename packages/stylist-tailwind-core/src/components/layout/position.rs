use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "static" => "position: static;",
    "fixed" => "position: fixed;",
    "absolute" => "position: absolute;",
    "relative" => "position: relative;",
    "sticky" => "position: sticky;",
};