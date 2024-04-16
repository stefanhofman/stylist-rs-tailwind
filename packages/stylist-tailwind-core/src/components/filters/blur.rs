use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "blur-none" => "filter: blur(0);",
    "blur-sm" => "filter: blur(4px);",
    "blur" => "filter: blur(8px);",
    "blur-md" => "filter: blur(12px);",
    "blur-lg" => "filter: blur(16px);",
    "blur-xl" => "filter: blur(24px);",
    "blur-2xl" => "filter: blur(40px);",
    "blur-3xl" => "filter: blur(64px);",
};