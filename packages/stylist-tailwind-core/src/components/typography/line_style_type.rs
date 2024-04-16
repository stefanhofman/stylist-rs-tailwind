use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "list-none" => "list-style-type: none;",
    "list-disc" => "list-style-type: disc;",
    "list-decimal" => "list-style-type: decimal;",
};