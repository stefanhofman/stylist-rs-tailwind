use stylist::{css, Style};

/// Tailwind `font-thin`; css `font-weight: 100;`
pub fn font_thin() -> Style {
    Style::new(css!("font-weight: 100;")).unwrap()
}

/// Tailwind `font-extralight`; css `font-weight: 200;`
pub fn font_extralight() -> Style {
    Style::new(css!("font-weight: 200;")).unwrap()
}

/// Tailwind `font-light`; css `font-weight: 300;`
pub fn font_light() -> Style {
    Style::new(css!("font-weight: 300;")).unwrap()
}

/// Tailwind `font-normal`; css `font-weight: 400;`
pub fn font_normal() -> Style {
    Style::new(css!("font-weight: 400;")).unwrap()
}

/// Tailwind `font-medium`; css `font-weight: 500;`
pub fn font_medium() -> Style {
    Style::new(css!("font-weight: 500;")).unwrap()
}

/// Tailwind `font-semibold`; css `font-weight: 600;`
pub fn font_semibold() -> Style {
    Style::new(css!("font-weight: 600;")).unwrap()
}

/// Tailwind `font-bold`; css `font-weight: 700;`
pub fn font_bold() -> Style {
    Style::new(css!("font-weight: 700;")).unwrap()
}

/// Tailwind `font-extrabold`; css `font-weight: 800;`
pub fn font_extrabold() -> Style {
    Style::new(css!("font-weight: 800;")).unwrap()
}

/// Tailwind `font-black`; css `font-weight: 900;`
pub fn font_black() -> Style {
    Style::new(css!("font-weight: 900;")).unwrap()
}

