use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "text-wrap" => "text-wrap: wrap;",
    "text-nowrap" => "text-wrap: nowrap;",
    "text-balance" => "text-wrap: balance;",
    "text-pretty" => "text-wrap: pretty;",
};