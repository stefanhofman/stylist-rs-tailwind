use stylist::{css, Style};

/// Tailwind `bg-none`; css `background-image: none;`
pub fn bg_none() -> Style {
    Style::new(css!("background-image: none;")).unwrap()
}

/// Tailwind `bg-gradient-to-t`; css `background-image: linear-gradient(to top, var(--tw-gradient-stops));`
pub fn bg_gradient_to_t() -> Style {
    Style::new(css!("background-image: linear-gradient(to top, var(--tw-gradient-stops));")).unwrap()
}

/// Tailwind `bg-gradient-to-tr`; css `background-image: linear-gradient(to top right, var(--tw-gradient-stops));`
pub fn bg_gradient_to_tr() -> Style {
    Style::new(css!("background-image: linear-gradient(to top right, var(--tw-gradient-stops));")).unwrap()
}

/// Tailwind `bg-gradient-to-r`; css `background-image: linear-gradient(to right, var(--tw-gradient-stops));`
pub fn bg_gradient_to_r() -> Style {
    Style::new(css!("background-image: linear-gradient(to right, var(--tw-gradient-stops));")).unwrap()
}

/// Tailwind `bg-gradient-to-br`; css `background-image: linear-gradient(to bottom right, var(--tw-gradient-stops));`
pub fn bg_gradient_to_br() -> Style {
    Style::new(css!("background-image: linear-gradient(to bottom right, var(--tw-gradient-stops));")).unwrap()
}

/// Tailwind `bg-gradient-to-b`; css `background-image: linear-gradient(to bottom, var(--tw-gradient-stops));`
pub fn bg_gradient_to_b() -> Style {
    Style::new(css!("background-image: linear-gradient(to bottom, var(--tw-gradient-stops));")).unwrap()
}

/// Tailwind `bg-gradient-to-bl`; css `background-image: linear-gradient(to bottom left, var(--tw-gradient-stops));`
pub fn bg_gradient_to_bl() -> Style {
    Style::new(css!("background-image: linear-gradient(to bottom left, var(--tw-gradient-stops));")).unwrap()
}

/// Tailwind `bg-gradient-to-l`; css `background-image: linear-gradient(to left, var(--tw-gradient-stops));`
pub fn bg_gradient_to_l() -> Style {
    Style::new(css!("background-image: linear-gradient(to left, var(--tw-gradient-stops));")).unwrap()
}

/// Tailwind `bg-gradient-to-tl`; css `background-image: linear-gradient(to top left, var(--tw-gradient-stops));`
pub fn bg_gradient_to_tl() -> Style {
    Style::new(css!("background-image: linear-gradient(to top left, var(--tw-gradient-stops));")).unwrap()
}

