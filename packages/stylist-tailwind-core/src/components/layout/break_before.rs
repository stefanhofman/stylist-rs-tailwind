use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "break-before-auto" => "break-before: auto;",
    "break-before-avoid" => "break-before: avoid;",
    "break-before-all" => "break-before: all;",
    "break-before-avoid-page" => "break-before: avoid-page;",
    "break-before-page" => "break-before: page;",
    "break-before-left" => "break-before: left;",
    "break-before-right" => "break-before: right;",
    "break-before-column" => "break-before: column;",
};