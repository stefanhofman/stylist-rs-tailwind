use std::str::FromStr;
use stylist::{css, Style, StyleSource};

pub fn combine(style: &str, modifiers: Vec<String>) -> String {
    if modifiers.len() > 0 {
        let mods: String = modifiers.join(" ");
        return format!("{} {{ {} }}", mods, style);
    }
    String::from(style)
}

pub fn hover() -> String {
    String::from("&:hover")
}

/// Tailwind `bg-auto`; css `background-size: auto;`
pub fn bg_auto(modifiers: Vec<String>) -> Style {
    let css = combine("background-size: auto;", modifiers);
    Style::new(StyleSource::from_str(css.as_str()).unwrap()).unwrap()
}
#[macro_export]
macro_rules! bg_auto {
    ($($modifiers:expr)*) => {
        Style::new(
            StyleSource::from_str(combine("background-size: auto;", vec![$($modifiers)*]).as_str())
                .unwrap(),
        )
        .unwrap()
    };
}

/// Tailwind `bg-cover`; css `background-size: cover;`
pub fn bg_cover() -> Style {
    Style::new(css!("background-size: cover;")).unwrap()
}

/// Tailwind `bg-contain`; css `background-size: contain;`
pub fn bg_contain() -> Style {
    Style::new(css!("background-size: contain;")).unwrap()
}

pub fn a() -> Style {
    bg_auto!()
}
