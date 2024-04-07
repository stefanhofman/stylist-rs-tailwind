use stylist::{css, Style};

/// Tailwind `float-start`; css `float: inline-start;`
pub fn float_start() -> Style {
    Style::new(css!("float: inline-start;")).unwrap()
}

/// Tailwind `float-end`; css `float: inline-end;`
pub fn float_end() -> Style {
    Style::new(css!("float: inline-end;")).unwrap()
}

/// Tailwind `float-right`; css `float: right;`
pub fn float_right() -> Style {
    Style::new(css!("float: right;")).unwrap()
}

/// Tailwind `float-left`; css `float: left;`
pub fn float_left() -> Style {
    Style::new(css!("float: left;")).unwrap()
}

/// Tailwind `float-none`; css `float: none;`
pub fn float_none() -> Style {
    Style::new(css!("float: none;")).unwrap()
}

