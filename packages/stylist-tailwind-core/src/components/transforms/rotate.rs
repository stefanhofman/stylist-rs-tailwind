use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "rotate-0" => "transform: rotate(0deg);",
    "rotate-1" => "transform: rotate(1deg);",
    "rotate-2" => "transform: rotate(2deg);",
    "rotate-3" => "transform: rotate(3deg);",
    "rotate-6" => "transform: rotate(6deg);",
    "rotate-12" => "transform: rotate(12deg);",
    "rotate-45" => "transform: rotate(45deg);",
    "rotate-90" => "transform: rotate(90deg);",
    "rotate-180" => "transform: rotate(180deg);",
};