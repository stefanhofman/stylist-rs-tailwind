use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "backdrop-blur-none" => "backdrop-filter: blur(0);",
    "backdrop-blur-sm" => "backdrop-filter: blur(4px);",
    "backdrop-blur" => "backdrop-filter: blur(8px);",
    "backdrop-blur-md" => "backdrop-filter: blur(12px);",
    "backdrop-blur-lg" => "backdrop-filter: blur(16px);",
    "backdrop-blur-xl" => "backdrop-filter: blur(24px);",
    "backdrop-blur-2xl" => "backdrop-filter: blur(40px);",
    "backdrop-blur-3xl" => "backdrop-filter: blur(64px);",
};