use stylist::{css, Style};

/// Tailwind `box-border`; css `box-sizing: border-box;`
pub fn box_border() -> Style {
    Style::new(css!("box-sizing: border-box;")).unwrap()
}

/// Tailwind `box-content`; css `box-sizing: content-box;`
pub fn box_content() -> Style {
    Style::new(css!("box-sizing: content-box;")).unwrap()
}

