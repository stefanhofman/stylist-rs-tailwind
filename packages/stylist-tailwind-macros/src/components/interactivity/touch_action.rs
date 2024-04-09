use stylist::{css, Style};

/// Tailwind `touch-auto`; css `touch-action: auto;`
pub fn touch_auto() -> Style {
    Style::new(css!("touch-action: auto;")).unwrap()
}

/// Tailwind `touch-none`; css `touch-action: none;`
pub fn touch_none() -> Style {
    Style::new(css!("touch-action: none;")).unwrap()
}

/// Tailwind `touch-pan-x`; css `touch-action: pan-x;`
pub fn touch_pan_x() -> Style {
    Style::new(css!("touch-action: pan-x;")).unwrap()
}

/// Tailwind `touch-pan-left`; css `touch-action: pan-left;`
pub fn touch_pan_left() -> Style {
    Style::new(css!("touch-action: pan-left;")).unwrap()
}

/// Tailwind `touch-pan-right`; css `touch-action: pan-right;`
pub fn touch_pan_right() -> Style {
    Style::new(css!("touch-action: pan-right;")).unwrap()
}

/// Tailwind `touch-pan-y`; css `touch-action: pan-y;`
pub fn touch_pan_y() -> Style {
    Style::new(css!("touch-action: pan-y;")).unwrap()
}

/// Tailwind `touch-pan-up`; css `touch-action: pan-up;`
pub fn touch_pan_up() -> Style {
    Style::new(css!("touch-action: pan-up;")).unwrap()
}

/// Tailwind `touch-pan-down`; css `touch-action: pan-down;`
pub fn touch_pan_down() -> Style {
    Style::new(css!("touch-action: pan-down;")).unwrap()
}

/// Tailwind `touch-pinch-zoom`; css `touch-action: pinch-zoom;`
pub fn touch_pinch_zoom() -> Style {
    Style::new(css!("touch-action: pinch-zoom;")).unwrap()
}

/// Tailwind `touch-manipulation`; css `touch-action: manipulation;`
pub fn touch_manipulation() -> Style {
    Style::new(css!("touch-action: manipulation;")).unwrap()
}

