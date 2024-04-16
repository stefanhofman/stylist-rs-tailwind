use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "underline" => "text-decoration-line: underline;",
    "overline" => "text-decoration-line: overline;",
    "line-through" => "text-decoration-line: line-through;",
    "no-underline" => "text-decoration-line: none;",
};