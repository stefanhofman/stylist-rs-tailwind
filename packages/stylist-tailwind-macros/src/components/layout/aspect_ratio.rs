use stylist::{css, Style};

/// Tailwind `aspect-auto`; css `aspect-ratio: auto;`
pub fn aspect_auto() -> Style {
    Style::new(css!("aspect-ratio: auto;")).unwrap()
}

/// Tailwind `aspect-square`; css `aspect-ratio: 1 / 1;`
pub fn aspect_square() -> Style {
    Style::new(css!("aspect-ratio: 1 / 1;")).unwrap()
}

/// Tailwind `aspect-video`; css `aspect-ratio: 16 / 9;`
pub fn aspect_video() -> Style {
    Style::new(css!("aspect-ratio: 16 / 9;")).unwrap()
}

