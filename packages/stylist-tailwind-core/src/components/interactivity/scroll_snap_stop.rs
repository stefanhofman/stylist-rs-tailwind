use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "snap-normal" => "scroll-snap-stop: normal;",
    "snap-always" => "scroll-snap-stop: always;",
};