use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "float-start" => "float: inline-start;",
    "float-end" => "float: inline-end;",
    "float-right" => "float: right;",
    "float-left" => "float: left;",
    "float-none" => "float: none;",
};