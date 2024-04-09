use stylist::{css, Style};

/// Tailwind `caption-top`; css `caption-side: top;`
pub fn caption_top() -> Style {
    Style::new(css!("caption-side: top;")).unwrap()
}

/// Tailwind `caption-bottom`; css `caption-side: bottom;`
pub fn caption_bottom() -> Style {
    Style::new(css!("caption-side: bottom;")).unwrap()
}

