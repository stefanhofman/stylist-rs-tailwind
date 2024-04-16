use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "bg-auto" => "background-size: auto;",
    "bg-cover" => "background-size: cover;",
    "bg-contain" => "background-size: contain;",
};