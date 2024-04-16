use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "justify-items-start" => "justify-items: start;",
    "justify-items-end" => "justify-items: end;",
    "justify-items-center" => "justify-items: center;",
    "justify-items-stretch" => "justify-items: stretch;",
};