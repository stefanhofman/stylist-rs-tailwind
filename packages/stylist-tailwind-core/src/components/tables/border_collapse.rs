use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "border-collapse" => "border-collapse: collapse;",
    "border-separate" => "border-collapse: separate;",
};