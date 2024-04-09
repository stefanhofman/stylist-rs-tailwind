use stylist::{css, Style};

/// Tailwind `content-none`; css `content: none;`
pub fn content_none() -> Style {
    Style::new(css!("content: none;")).unwrap()
}

