use stylist::{css, Style};

/// Tailwind `text-left`; css `text-align: left;`
pub fn text_left() -> Style {
    Style::new(css!("text-align: left;")).unwrap()
}

/// Tailwind `text-center`; css `text-align: center;`
pub fn text_center() -> Style {
    Style::new(css!("text-align: center;")).unwrap()
}

/// Tailwind `text-right`; css `text-align: right;`
pub fn text_right() -> Style {
    Style::new(css!("text-align: right;")).unwrap()
}

/// Tailwind `text-justify`; css `text-align: justify;`
pub fn text_justify() -> Style {
    Style::new(css!("text-align: justify;")).unwrap()
}
