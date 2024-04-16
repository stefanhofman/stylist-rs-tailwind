use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "align-baseline" => "vertical-align: baseline;",
    "align-top" => "vertical-align: top;",
    "align-middle" => "vertical-align: middle;",
    "align-bottom" => "vertical-align: bottom;",
    "align-text-top" => "vertical-align: text-top;",
    "align-text-bottom" => "vertical-align: text-bottom;",
    "align-sub" => "vertical-align: sub;",
    "align-super" => "vertical-align: super;",
};