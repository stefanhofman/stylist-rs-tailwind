use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "font-thin" => "font-weight: 100;",
    "font-extralight" => "font-weight: 200;",
    "font-light" => "font-weight: 300;",
    "font-normal" => "font-weight: 400;",
    "font-medium" => "font-weight: 500;",
    "font-semibold" => "font-weight: 600;",
    "font-bold" => "font-weight: 700;",
    "font-extrabold" => "font-weight: 800;",
    "font-black" => "font-weight: 900;",
};