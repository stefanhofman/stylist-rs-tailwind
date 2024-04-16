use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "flex-1" => "flex: 1 1 0%;",
    "flex-auto" => "flex: 1 1 auto;",
    "flex-initial" => "flex: 0 1 auto;",
    "flex-none" => "flex: none;",
};