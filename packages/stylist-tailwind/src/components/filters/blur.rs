use stylist::{css, Style};

/// Tailwind `blur-none`; css `filter: blur(0);`
pub fn blur_none() -> Style {
    Style::new(css!("filter: blur(0);")).unwrap()
}

/// Tailwind `blur-sm`; css `filter: blur(4px);`
pub fn blur_sm() -> Style {
    Style::new(css!("filter: blur(4px);")).unwrap()
}

/// Tailwind `blur`; css `filter: blur(8px);`
pub fn blur() -> Style {
    Style::new(css!("filter: blur(8px);")).unwrap()
}

/// Tailwind `blur-md`; css `filter: blur(12px);`
pub fn blur_md() -> Style {
    Style::new(css!("filter: blur(12px);")).unwrap()
}

/// Tailwind `blur-lg`; css `filter: blur(16px);`
pub fn blur_lg() -> Style {
    Style::new(css!("filter: blur(16px);")).unwrap()
}

/// Tailwind `blur-xl`; css `filter: blur(24px);`
pub fn blur_xl() -> Style {
    Style::new(css!("filter: blur(24px);")).unwrap()
}

/// Tailwind `blur-2xl`; css `filter: blur(40px);`
pub fn blur_2xl() -> Style {
    Style::new(css!("filter: blur(40px);")).unwrap()
}

/// Tailwind `blur-3xl`; css `filter: blur(64px);`
pub fn blur_3xl() -> Style {
    Style::new(css!("filter: blur(64px);")).unwrap()
}

