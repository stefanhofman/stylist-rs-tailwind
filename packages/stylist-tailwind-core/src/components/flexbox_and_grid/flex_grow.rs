use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "grow" => "flex-grow: 1;",
    "grow-0" => "flex-grow: 0;",
};