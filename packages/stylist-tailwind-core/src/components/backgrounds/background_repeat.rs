use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "bg-repeat" => "background-repeat: repeat;",
    "bg-no-repeat" => "background-repeat: no-repeat;",
    "bg-repeat-x" => "background-repeat: repeat-x;",
    "bg-repeat-y" => "background-repeat: repeat-y;",
    "bg-repeat-round" => "background-repeat: round;",
    "bg-repeat-space" => "background-repeat: space;",
};