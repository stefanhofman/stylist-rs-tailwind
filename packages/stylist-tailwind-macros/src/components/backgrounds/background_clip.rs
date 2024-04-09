use stylist::{css, Style};

/// Tailwind `bg-clip-border`; css `background-clip: border-box;`
pub fn bg_clip_border() -> Style {
    Style::new(css!("background-clip: border-box;")).unwrap()
}

/// Tailwind `bg-clip-padding`; css `background-clip: padding-box;`
pub fn bg_clip_padding() -> Style {
    Style::new(css!("background-clip: padding-box;")).unwrap()
}

/// Tailwind `bg-clip-content`; css `background-clip: content-box;`
pub fn bg_clip_content() -> Style {
    Style::new(css!("background-clip: content-box;")).unwrap()
}

/// Tailwind `bg-clip-text`; css `background-clip: text;`
pub fn bg_clip_text() -> Style {
    Style::new(css!("background-clip: text;")).unwrap()
}

