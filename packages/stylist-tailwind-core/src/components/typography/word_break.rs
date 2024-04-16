use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "break-normal" => "overflow-wrap: normal; word-break: normal;",
    "break-words" => "overflow-wrap: break-word;",
    "break-all" => "word-break: break-all;",
    "break-keep" => "word-break: keep-all;",
};