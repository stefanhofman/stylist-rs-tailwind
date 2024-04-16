use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "backdrop-brightness-0" => "backdrop-filter: brightness(0);",
    "backdrop-brightness-50" => "backdrop-filter: brightness(.5);",
    "backdrop-brightness-75" => "backdrop-filter: brightness(.75);",
    "backdrop-brightness-90" => "backdrop-filter: brightness(.9);",
    "backdrop-brightness-95" => "backdrop-filter: brightness(.95);",
    "backdrop-brightness-100" => "backdrop-filter: brightness(1);",
    "backdrop-brightness-105" => "backdrop-filter: brightness(1.05);",
    "backdrop-brightness-110" => "backdrop-filter: brightness(1.1);",
    "backdrop-brightness-125" => "backdrop-filter: brightness(1.25);",
    "backdrop-brightness-150" => "backdrop-filter: brightness(1.5);",
    "backdrop-brightness-200" => "backdrop-filter: brightness(2);",
};