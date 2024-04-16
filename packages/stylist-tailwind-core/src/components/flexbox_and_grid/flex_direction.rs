use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "flex-row" => "flex-direction: row;",
    "flex-row-reverse" => "flex-direction: row-reverse;",
    "flex-col" => "flex-direction: column;",
    "flex-col-reverse" => "flex-direction: column-reverse;",
};