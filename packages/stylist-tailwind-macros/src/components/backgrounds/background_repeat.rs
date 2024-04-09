use stylist::{css, Style};

/// Tailwind `bg-repeat`; css `background-repeat: repeat;`
pub fn bg_repeat() -> Style {
    Style::new(css!("background-repeat: repeat;")).unwrap()
}

/// Tailwind `bg-no-repeat`; css `background-repeat: no-repeat;`
pub fn bg_no_repeat() -> Style {
    Style::new(css!("background-repeat: no-repeat;")).unwrap()
}

/// Tailwind `bg-repeat-x`; css `background-repeat: repeat-x;`
pub fn bg_repeat_x() -> Style {
    Style::new(css!("background-repeat: repeat-x;")).unwrap()
}

/// Tailwind `bg-repeat-y`; css `background-repeat: repeat-y;`
pub fn bg_repeat_y() -> Style {
    Style::new(css!("background-repeat: repeat-y;")).unwrap()
}

/// Tailwind `bg-repeat-round`; css `background-repeat: round;`
pub fn bg_repeat_round() -> Style {
    Style::new(css!("background-repeat: round;")).unwrap()
}

/// Tailwind `bg-repeat-space`; css `background-repeat: space;`
pub fn bg_repeat_space() -> Style {
    Style::new(css!("background-repeat: space;")).unwrap()
}

