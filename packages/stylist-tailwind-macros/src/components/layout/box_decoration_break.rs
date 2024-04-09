use stylist::{css, Style};

/// Tailwind `box-decoration-clone`; css `box-decoration-break: clone;`
pub fn box_decoration_clone() -> Style {
    Style::new(css!("box-decoration-break: clone;")).unwrap()
}

/// Tailwind `box-decoration-slice`; css `box-decoration-break: slice;`
pub fn box_decoration_slice() -> Style {
    Style::new(css!("box-decoration-break: slice;")).unwrap()
}

