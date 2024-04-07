use stylist::{css, Style};

/// Tailwind `static`; css `position: static;`
pub fn r#static() -> Style {
    Style::new(css!("position: static;")).unwrap()
}

/// Tailwind `fixed`; css `position: fixed;`
pub fn fixed() -> Style {
    Style::new(css!("position: fixed;")).unwrap()
}

/// Tailwind `absolute`; css `position: absolute;`
pub fn absolute() -> Style {
    Style::new(css!("position: absolute;")).unwrap()
}

/// Tailwind `relative`; css `position: relative;`
pub fn relative() -> Style {
    Style::new(css!("position: relative;")).unwrap()
}

/// Tailwind `sticky`; css `position: sticky;`
pub fn sticky() -> Style {
    Style::new(css!("position: sticky;")).unwrap()
}

