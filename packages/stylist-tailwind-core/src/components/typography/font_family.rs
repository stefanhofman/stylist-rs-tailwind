use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "font-sans" => "font-family: ui-sans-serif, system-ui, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color Emoji\";",
    "font-serif" => "font-family: ui-serif, Georgia, Cambria, \"Times New Roman\", Times, serif;",
    "font-mono" => "font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace;",
};