use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "skew-x-0" => "transform: skewX(0deg);",
    "skew-y-0" => "transform: skewY(0deg);",
    "skew-x-1" => "transform: skewX(1deg);",
    "skew-y-1" => "transform: skewY(1deg);",
    "skew-x-2" => "transform: skewX(2deg);",
    "skew-y-2" => "transform: skewY(2deg);",
    "skew-x-3" => "transform: skewX(3deg);",
    "skew-y-3" => "transform: skewY(3deg);",
    "skew-x-6" => "transform: skewX(6deg);",
    "skew-y-6" => "transform: skewY(6deg);",
    "skew-x-12" => "transform: skewX(12deg);",
    "skew-y-12" => "transform: skewY(12deg);",
};