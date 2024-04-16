use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "items-start" => "align-items: flex-start;",
    "items-end" => "align-items: flex-end;",
    "items-center" => "align-items: center;",
    "items-baseline" => "align-items: baseline;",
    "items-stretch" => "align-items: stretch;",
};