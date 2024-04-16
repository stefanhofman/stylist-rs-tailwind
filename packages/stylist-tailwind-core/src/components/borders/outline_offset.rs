use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "outline-offset-0" => "outline-offset: 0px;",
    "outline-offset-1" => "outline-offset: 1px;",
    "outline-offset-2" => "outline-offset: 2px;",
    "outline-offset-4" => "outline-offset: 4px;",
    "outline-offset-8" => "outline-offset: 8px;",
};