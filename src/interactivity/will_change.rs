use stylist::{css, Style};

/// Tailwind `will-change-auto`; css `will-change: auto;`
pub fn will_change_auto() -> Style {
    Style::new(css!("will-change: auto;")).unwrap()
}

/// Tailwind `will-change-scroll`; css `will-change: scroll-position;`
pub fn will_change_scroll() -> Style {
    Style::new(css!("will-change: scroll-position;")).unwrap()
}

/// Tailwind `will-change-contents`; css `will-change: contents;`
pub fn will_change_contents() -> Style {
    Style::new(css!("will-change: contents;")).unwrap()
}

/// Tailwind `will-change-transform`; css `will-change: transform;`
pub fn will_change_transform() -> Style {
    Style::new(css!("will-change: transform;")).unwrap()
}

