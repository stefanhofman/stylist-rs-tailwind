use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "tracking-tighter" => "letter-spacing: -0.05em;",
    "tracking-tight" => "letter-spacing: -0.025em;",
    "tracking-normal" => "letter-spacing: 0em;",
    "tracking-wide" => "letter-spacing: 0.025em;",
    "tracking-wider" => "letter-spacing: 0.05em;",
    "tracking-widest" => "letter-spacing: 0.1em;",
};