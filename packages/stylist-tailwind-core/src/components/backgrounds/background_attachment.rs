use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "bg-fixed" => "background-attachment: fixed;",
    "bg-local" => "background-attachment: local;",
    "bg-scroll" => "background-attachment: scroll;",
};