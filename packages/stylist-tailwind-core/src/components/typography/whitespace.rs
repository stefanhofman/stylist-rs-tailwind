use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "whitespace-normal" => "white-space: normal;",
    "whitespace-nowrap" => "white-space: nowrap;",
    "whitespace-pre" => "white-space: pre;",
    "whitespace-pre-line" => "white-space: pre-line;",
    "whitespace-pre-wrap" => "white-space: pre-wrap;",
    "whitespace-break-spaces" => "white-space: break-spaces;",
};