use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "overscroll-auto" => "overscroll-behavior: auto;",
    "overscroll-contain" => "overscroll-behavior: contain;",
    "overscroll-none" => "overscroll-behavior: none;",
    "overscroll-y-auto" => "overscroll-behavior-y: auto;",
    "overscroll-y-contain" => "overscroll-behavior-y: contain;",
    "overscroll-y-none" => "overscroll-behavior-y: none;",
    "overscroll-x-auto" => "overscroll-behavior-x: auto;",
    "overscroll-x-contain" => "overscroll-behavior-x: contain;",
    "overscroll-x-none" => "overscroll-behavior-x: none;",
};