use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "text-left" => "text-align: left;",
    "text-center" => "text-align: center;",
    "text-right" => "text-align: right;",
    "text-justify" => "text-align: justify;",
    "text-start" => "text-align: start;",
    "text-end" => "text-align: end;",
};