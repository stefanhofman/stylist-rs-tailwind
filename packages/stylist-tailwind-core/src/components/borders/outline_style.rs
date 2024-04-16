use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "outline-none" => "outline: 2px solid transparent; outline-offset: 2px;",
    "outline" => "outline-style: solid;",
    "outline-dashed" => "outline-style: dashed;",
    "outline-dotted" => "outline-style: dotted;",
    "outline-double" => "outline-style: double;",
};