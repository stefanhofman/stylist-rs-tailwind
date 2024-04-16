use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "object-contain" => "object-fit: contain;",
    "object-cover" => "object-fit: cover;",
    "object-fill" => "object-fit: fill;",
    "object-none" => "object-fit: none;",
    "object-scale-down" => "object-fit: scale-down;",
};