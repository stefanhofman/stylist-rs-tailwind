use stylist::{css, Style};

/// Tailwind `justify-start`; css `justify-content: flex-start`
pub fn justify_start() -> Style {
    Style::new(css!("justify-content: flex-start;")).unwrap()
}

/// Tailwind `justify-end`; css `justify-content: flex-end`
pub fn justify_end() -> Style {
    Style::new(css!("justify-content: flex-end;")).unwrap()
}

/// Tailwind `justify-center`; css `justify-content: center`
pub fn justify_center() -> Style {
    Style::new(css!("justify-content: center;")).unwrap()
}

/// Tailwind `justify-between`; css `justify-content: space-between`
pub fn justify_between() -> Style {
    Style::new(css!("justify-content: space-between;")).unwrap()
}

/// Tailwind `justify-around`; css `justify-content: space-around`
pub fn justify_around() -> Style {
    Style::new(css!("justify-content: space-around;")).unwrap()
}

/// Tailwind `justify-evenly`; css `justify-content: space-evenly`
pub fn justify_evenly() -> Style {
    Style::new(css!("justify-content: space-evenly;")).unwrap()
}
