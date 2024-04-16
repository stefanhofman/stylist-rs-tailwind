use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "table-auto" => "table-layout: auto;",
    "table-fixed" => "table-layout: fixed;",
};