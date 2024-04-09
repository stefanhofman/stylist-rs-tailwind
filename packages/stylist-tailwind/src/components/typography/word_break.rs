use stylist::{css, Style};

/// Tailwind `break-normal`; css `overflow-wrap: normal; word-break: normal;`
pub fn break_normal() -> Style {
    Style::new(css!("overflow-wrap: normal; word-break: normal;")).unwrap()
}

/// Tailwind `break-words`; css `overflow-wrap: break-word;`
pub fn break_words() -> Style {
    Style::new(css!("overflow-wrap: break-word;")).unwrap()
}

/// Tailwind `break-all`; css `word-break: break-all;`
pub fn break_all() -> Style {
    Style::new(css!("word-break: break-all;")).unwrap()
}

/// Tailwind `break-keep`; css `word-break: keep-all;`
pub fn break_keep() -> Style {
    Style::new(css!("word-break: keep-all;")).unwrap()
}

