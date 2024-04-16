use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "forced-color-adjust-auto" => "forced-color-adjust: auto;",
    "forced-color-adjust-none" => "forced-color-adjust: none;",
};