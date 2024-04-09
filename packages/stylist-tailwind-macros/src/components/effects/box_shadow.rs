use stylist::{css, Style};

/// Tailwind `shadow-sm`; css `box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);`
pub fn shadow_sm() -> Style {
    Style::new(css!("box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);")).unwrap()
}

/// Tailwind `shadow`; css `box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);`
pub fn shadow() -> Style {
    Style::new(css!("box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);")).unwrap()
}

/// Tailwind `shadow-md`; css `box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);`
pub fn shadow_md() -> Style {
    Style::new(css!("box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);")).unwrap()
}

/// Tailwind `shadow-lg`; css `box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);`
pub fn shadow_lg() -> Style {
    Style::new(css!("box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);")).unwrap()
}

/// Tailwind `shadow-xl`; css `box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);`
pub fn shadow_xl() -> Style {
    Style::new(css!("box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);")).unwrap()
}

/// Tailwind `shadow-2xl`; css `box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);`
pub fn shadow_2xl() -> Style {
    Style::new(css!("box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);")).unwrap()
}

/// Tailwind `shadow-inner`; css `box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);`
pub fn shadow_inner() -> Style {
    Style::new(css!("box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);")).unwrap()
}

/// Tailwind `shadow-none`; css `box-shadow: 0 0 #0000;`
pub fn shadow_none() -> Style {
    Style::new(css!("box-shadow: 0 0 #0000;")).unwrap()
}

