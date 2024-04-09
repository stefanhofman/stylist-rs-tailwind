use stylist::{css, Style};

/// Tailwind `backdrop-blur-none`; css `backdrop-filter: blur(0);`
pub fn backdrop_blur_none() -> Style {
    Style::new(css!("backdrop-filter: blur(0);")).unwrap()
}

/// Tailwind `backdrop-blur-sm`; css `backdrop-filter: blur(4px);`
pub fn backdrop_blur_sm() -> Style {
    Style::new(css!("backdrop-filter: blur(4px);")).unwrap()
}

/// Tailwind `backdrop-blur`; css `backdrop-filter: blur(8px);`
pub fn backdrop_blur() -> Style {
    Style::new(css!("backdrop-filter: blur(8px);")).unwrap()
}

/// Tailwind `backdrop-blur-md`; css `backdrop-filter: blur(12px);`
pub fn backdrop_blur_md() -> Style {
    Style::new(css!("backdrop-filter: blur(12px);")).unwrap()
}

/// Tailwind `backdrop-blur-lg`; css `backdrop-filter: blur(16px);`
pub fn backdrop_blur_lg() -> Style {
    Style::new(css!("backdrop-filter: blur(16px);")).unwrap()
}

/// Tailwind `backdrop-blur-xl`; css `backdrop-filter: blur(24px);`
pub fn backdrop_blur_xl() -> Style {
    Style::new(css!("backdrop-filter: blur(24px);")).unwrap()
}

/// Tailwind `backdrop-blur-2xl`; css `backdrop-filter: blur(40px);`
pub fn backdrop_blur_2xl() -> Style {
    Style::new(css!("backdrop-filter: blur(40px);")).unwrap()
}

/// Tailwind `backdrop-blur-3xl`; css `backdrop-filter: blur(64px);`
pub fn backdrop_blur_3xl() -> Style {
    Style::new(css!("backdrop-filter: blur(64px);")).unwrap()
}

