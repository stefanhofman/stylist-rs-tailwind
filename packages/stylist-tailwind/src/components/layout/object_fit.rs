use stylist::{css, Style};

/// Tailwind `object-contain`; css `object-fit: contain;`
pub fn object_contain() -> Style {
    Style::new(css!("object-fit: contain;")).unwrap()
}

/// Tailwind `object-cover`; css `object-fit: cover;`
pub fn object_cover() -> Style {
    Style::new(css!("object-fit: cover;")).unwrap()
}

/// Tailwind `object-fill`; css `object-fit: fill;`
pub fn object_fill() -> Style {
    Style::new(css!("object-fit: fill;")).unwrap()
}

/// Tailwind `object-none`; css `object-fit: none;`
pub fn object_none() -> Style {
    Style::new(css!("object-fit: none;")).unwrap()
}

/// Tailwind `object-scale-down`; css `object-fit: scale-down;`
pub fn object_scale_down() -> Style {
    Style::new(css!("object-fit: scale-down;")).unwrap()
}

