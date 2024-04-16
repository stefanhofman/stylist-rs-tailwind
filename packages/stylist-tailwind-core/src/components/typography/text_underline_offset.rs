use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "underline-offset-auto" => "text-underline-offset: auto;",
    "underline-offset-0" => "text-underline-offset: 0px;",
    "underline-offset-1" => "text-underline-offset: 1px;",
    "underline-offset-2" => "text-underline-offset: 2px;",
    "underline-offset-4" => "text-underline-offset: 4px;",
    "underline-offset-8" => "text-underline-offset: 8px;",
};