use stylist::{css, Style};

/// Tailwind `transition-none`; css `transition-property: none;`
pub fn transition_none() -> Style {
    Style::new(css!("transition-property: none;")).unwrap()
}

/// Tailwind `transition-all`; css `transition-property: all; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;`
pub fn transition_all() -> Style {
    Style::new(css!("transition-property: all; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;")).unwrap()
}

/// Tailwind `transition`; css `transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;`
pub fn transition() -> Style {
    Style::new(css!("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;")).unwrap()
}

/// Tailwind `transition-colors`; css `transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;`
pub fn transition_colors() -> Style {
    Style::new(css!("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;")).unwrap()
}

/// Tailwind `transition-opacity`; css `transition-property: opacity; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;`
pub fn transition_opacity() -> Style {
    Style::new(css!("transition-property: opacity; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;")).unwrap()
}

/// Tailwind `transition-shadow`; css `transition-property: box-shadow; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;`
pub fn transition_shadow() -> Style {
    Style::new(css!("transition-property: box-shadow; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;")).unwrap()
}

/// Tailwind `transition-transform`; css `transition-property: transform; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;`
pub fn transition_transform() -> Style {
    Style::new(css!("transition-property: transform; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;")).unwrap()
}

