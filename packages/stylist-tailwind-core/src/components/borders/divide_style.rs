use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "divide-solid " => "border-style: solid;",
    "divide-dashed " => "border-style: dashed;",
    "divide-dotted " => "border-style: dotted;",
    "divide-double " => "border-style: double;",
    "divide-none " => "border-style: none;",
};