use stylist::{css, Style};

/// Tailwind `antialiased`; css `-webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale;`
pub fn antialiased() -> Style {
    Style::new(css!("-webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale;")).unwrap()
}

/// Tailwind `subpixel-antialiased`; css `-webkit-font-smoothing: auto; -moz-osx-font-smoothing: auto;`
pub fn subpixel_antialiased() -> Style {
    Style::new(css!("-webkit-font-smoothing: auto; -moz-osx-font-smoothing: auto;")).unwrap()
}

