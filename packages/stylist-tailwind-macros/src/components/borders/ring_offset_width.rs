use stylist::{css, Style};

/// Tailwind `ring-offset-0`; css `--tw-ring-offset-width: 0px; box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);`
pub fn ring_offset_0() -> Style {
    Style::new(css!("--tw-ring-offset-width: 0px; box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);")).unwrap()
}

/// Tailwind `ring-offset-1`; css `--tw-ring-offset-width: 1px; box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);`
pub fn ring_offset_1() -> Style {
    Style::new(css!("--tw-ring-offset-width: 1px; box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);")).unwrap()
}

/// Tailwind `ring-offset-2`; css `--tw-ring-offset-width: 2px; box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);`
pub fn ring_offset_2() -> Style {
    Style::new(css!("--tw-ring-offset-width: 2px; box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);")).unwrap()
}

/// Tailwind `ring-offset-4`; css `--tw-ring-offset-width: 4px; box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);`
pub fn ring_offset_4() -> Style {
    Style::new(css!("--tw-ring-offset-width: 4px; box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);")).unwrap()
}

/// Tailwind `ring-offset-8`; css `--tw-ring-offset-width: 8px; box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);`
pub fn ring_offset_8() -> Style {
    Style::new(css!("--tw-ring-offset-width: 8px; box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);")).unwrap()
}

