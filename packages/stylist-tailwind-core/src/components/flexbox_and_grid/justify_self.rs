use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "justify-self-auto" => "justify-self: auto;",
    "justify-self-start" => "justify-self: start;",
    "justify-self-end" => "justify-self: end;",
    "justify-self-center" => "justify-self: center;",
    "justify-self-stretch" => "justify-self: stretch;",
};