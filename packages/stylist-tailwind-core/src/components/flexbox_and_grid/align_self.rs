use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "self-auto" => "align-self: auto;",
    "self-start" => "align-self: flex-start;",
    "self-end" => "align-self: flex-end;",
    "self-center" => "align-self: center;",
    "self-stretch" => "align-self: stretch;",
    "self-baseline" => "align-self: baseline;",
};