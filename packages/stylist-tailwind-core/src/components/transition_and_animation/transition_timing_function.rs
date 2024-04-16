use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "ease-linear" => "transition-timing-function: linear;",
    "ease-in" => "transition-timing-function: cubic-bezier(0.4, 0, 1, 1);",
    "ease-out" => "transition-timing-function: cubic-bezier(0, 0, 0.2, 1);",
    "ease-in-out" => "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);",
};