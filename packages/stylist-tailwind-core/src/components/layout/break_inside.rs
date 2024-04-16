use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "break-inside-auto" => "break-inside: auto;",
    "break-inside-avoid" => "break-inside: avoid;",
    "break-inside-avoid-page" => "break-inside: avoid-page;",
    "break-inside-avoid-column" => "break-inside: avoid-column;",
};