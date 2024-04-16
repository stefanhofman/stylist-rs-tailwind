use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "snap-start" => "scroll-snap-align: start;",
    "snap-end" => "scroll-snap-align: end;",
    "snap-center" => "scroll-snap-align: center;",
    "snap-align-none" => "scroll-snap-align: none;",
};