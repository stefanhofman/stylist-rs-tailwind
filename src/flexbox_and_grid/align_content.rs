use stylist::{css, Style};

/// Tailwind `content-center`; css `align-content: center`
pub fn content_center() -> Style {
    Style::new(css!("align-content: center;")).unwrap()
}

/// Tailwind `content-start`; css `align-content: flex-start`
pub fn content_start() -> Style {
    Style::new(css!("align-content: flex-start;")).unwrap()
}

/// Tailwind `content-end`; css `align-content: flex-end`
pub fn content_end() -> Style {
    Style::new(css!("align-content: flex-end;")).unwrap()
}

/// Tailwind `content-between`; css `align-content: space-between`
pub fn content_between() -> Style {
    Style::new(css!("align-content: space-between;")).unwrap()
}

/// Tailwind `content-around`; css `align-content: space-around`
pub fn content_around() -> Style {
    Style::new(css!("align-content: space-around;")).unwrap()
}

/// Tailwind `content-evenly`; css `align-content: space-evenly`
pub fn content_evenly() -> Style {
    Style::new(css!("align-content: space-evenly;")).unwrap()
}
