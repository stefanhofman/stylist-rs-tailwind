use stylist::{css, Style};

/// Tailwind `snap-start`; css `scroll-snap-align: start;`
pub fn snap_start() -> Style {
    Style::new(css!("scroll-snap-align: start;")).unwrap()
}

/// Tailwind `snap-end`; css `scroll-snap-align: end;`
pub fn snap_end() -> Style {
    Style::new(css!("scroll-snap-align: end;")).unwrap()
}

/// Tailwind `snap-center`; css `scroll-snap-align: center;`
pub fn snap_center() -> Style {
    Style::new(css!("scroll-snap-align: center;")).unwrap()
}

/// Tailwind `snap-align-none`; css `scroll-snap-align: none;`
pub fn snap_align_none() -> Style {
    Style::new(css!("scroll-snap-align: none;")).unwrap()
}

