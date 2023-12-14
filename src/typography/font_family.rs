use stylist::{css, Style};

/// Tailwind `font-sans`; css `font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";`
pub fn font_sans() -> Style {
    Style::new(css!("font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, \"Noto Sans\", sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color Emoji\";")).unwrap()
}

/// Tailwind `font-serif`; css `font-family: ui-serif, Georgia, Cambria, "Times New Roman", Times, serif;`
pub fn font_serif() -> Style {
    Style::new(css!(
        "font-family: ui-serif, Georgia, Cambria, \"Times New Roman\", Times, serif;"
    ))
    .unwrap()
}

/// Tailwind `font-mono`; css `font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;`
pub fn font_mono() -> Style {
    Style::new(css!("font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace;")).unwrap()
}
