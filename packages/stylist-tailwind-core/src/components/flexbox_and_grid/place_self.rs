use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "place-self-auto" => "place-self: auto;",
    "place-self-start" => "place-self: start;",
    "place-self-end" => "place-self: end;",
    "place-self-center" => "place-self: center;",
    "place-self-stretch" => "place-self: stretch;",
};