use stylist::{css, Style};

/// Tailwind `bg-fixed`; css `background-attachment: fixed;`
pub fn bg_fixed() -> Style {
    Style::new(css!("background-attachment: fixed;")).unwrap()
}

/// Tailwind `bg-local`; css `background-attachment: local;`
pub fn bg_local() -> Style {
    Style::new(css!("background-attachment: local;")).unwrap()
}

/// Tailwind `bg-scroll`; css `background-attachment: scroll;`
pub fn bg_scroll() -> Style {
    Style::new(css!("background-attachment: scroll;")).unwrap()
}

