use stylist::{css, Style};

/// Tailwind `snap-normal`; css `scroll-snap-stop: normal;`
pub fn snap_normal() -> Style {
    Style::new(css!("scroll-snap-stop: normal;")).unwrap()
}

/// Tailwind `snap-always`; css `scroll-snap-stop: always;`
pub fn snap_always() -> Style {
    Style::new(css!("scroll-snap-stop: always;")).unwrap()
}

