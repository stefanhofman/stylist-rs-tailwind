use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "brightness-0" => "filter: brightness(0);",
    "brightness-50" => "filter: brightness(.5);",
    "brightness-75" => "filter: brightness(.75);",
    "brightness-90" => "filter: brightness(.9);",
    "brightness-95" => "filter: brightness(.95);",
    "brightness-100" => "filter: brightness(1);",
    "brightness-105" => "filter: brightness(1.05);",
    "brightness-110" => "filter: brightness(1.1);",
    "brightness-125" => "filter: brightness(1.25);",
    "brightness-150" => "filter: brightness(1.5);",
    "brightness-200" => "filter: brightness(2);",
};