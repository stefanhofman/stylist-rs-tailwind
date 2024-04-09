use stylist::{css, Style};

/// Tailwind `bg-origin-border`; css `background-origin: border-box;`
pub fn bg_origin_border() -> Style {
    Style::new(css!("background-origin: border-box;")).unwrap()
}

/// Tailwind `bg-origin-padding`; css `background-origin: padding-box;`
pub fn bg_origin_padding() -> Style {
    Style::new(css!("background-origin: padding-box;")).unwrap()
}

/// Tailwind `bg-origin-content`; css `background-origin: content-box;`
pub fn bg_origin_content() -> Style {
    Style::new(css!("background-origin: content-box;")).unwrap()
}

