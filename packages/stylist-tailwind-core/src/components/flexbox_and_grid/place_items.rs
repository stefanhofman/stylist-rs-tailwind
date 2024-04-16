use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "place-items-start" => "place-items: start;",
    "place-items-end" => "place-items: end;",
    "place-items-center" => "place-items: center;",
    "place-items-baseline" => "place-items: baseline;",
    "place-items-stretch" => "place-items: stretch;",
};