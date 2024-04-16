use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "truncate" => "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
    "text-ellipsis" => "text-overflow: ellipsis;",
    "text-clip" => "text-overflow: clip;",
};