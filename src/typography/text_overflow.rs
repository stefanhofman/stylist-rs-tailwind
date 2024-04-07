use stylist::{css, Style};

/// Tailwind `truncate`; css `overflow: hidden; text-overflow: ellipsis; white-space: nowrap;`
pub fn truncate() -> Style {
    Style::new(css!("overflow: hidden; text-overflow: ellipsis; white-space: nowrap;")).unwrap()
}

/// Tailwind `text-ellipsis`; css `text-overflow: ellipsis;`
pub fn text_ellipsis() -> Style {
    Style::new(css!("text-overflow: ellipsis;")).unwrap()
}

/// Tailwind `text-clip`; css `text-overflow: clip;`
pub fn text_clip() -> Style {
    Style::new(css!("text-overflow: clip;")).unwrap()
}

