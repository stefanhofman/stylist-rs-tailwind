use stylist::{css, Style};

/// Tailwind `drop-shadow-sm`; css `filter: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05));`
pub fn drop_shadow_sm() -> Style {
    Style::new(css!("filter: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05));")).unwrap()
}

/// Tailwind `drop-shadow`; css `filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06));`
pub fn drop_shadow() -> Style {
    Style::new(css!("filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06));")).unwrap()
}

/// Tailwind `drop-shadow-md`; css `filter: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));`
pub fn drop_shadow_md() -> Style {
    Style::new(css!("filter: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));")).unwrap()
}

/// Tailwind `drop-shadow-lg`; css `filter: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1));`
pub fn drop_shadow_lg() -> Style {
    Style::new(css!("filter: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1));")).unwrap()
}

/// Tailwind `drop-shadow-xl`; css `filter: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08));`
pub fn drop_shadow_xl() -> Style {
    Style::new(css!("filter: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08));")).unwrap()
}

/// Tailwind `drop-shadow-2xl`; css `filter: drop-shadow(0 25px 25px rgb(0 0 0 / 0.15));`
pub fn drop_shadow_2xl() -> Style {
    Style::new(css!("filter: drop-shadow(0 25px 25px rgb(0 0 0 / 0.15));")).unwrap()
}

/// Tailwind `drop-shadow-none`; css `filter: drop-shadow(0 0 #0000);`
pub fn drop_shadow_none() -> Style {
    Style::new(css!("filter: drop-shadow(0 0 #0000);")).unwrap()
}

