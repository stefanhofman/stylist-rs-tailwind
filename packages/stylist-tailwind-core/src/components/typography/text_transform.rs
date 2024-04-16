use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "uppercase" => "text-transform: uppercase;",
    "lowercase" => "text-transform: lowercase;",
    "capitalize" => "text-transform: capitalize;",
    "normal-case" => "text-transform: none;",
};