use stylist::{css, Style};

/// Tailwind `ring-0`; css `box-shadow: var(--tw-ring-inset) 0 0 0 calc(0px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
pub fn ring_0() -> Style {
    Style::new(css!("box-shadow: var(--tw-ring-inset) 0 0 0 calc(0px + var(--tw-ring-offset-width)) var(--tw-ring-color);")).unwrap()
}

/// Tailwind `ring-1`; css `box-shadow: var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
pub fn ring_1() -> Style {
    Style::new(css!("box-shadow: var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color);")).unwrap()
}

/// Tailwind `ring-2`; css `box-shadow: var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
pub fn ring_2() -> Style {
    Style::new(css!("box-shadow: var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);")).unwrap()
}

/// Tailwind `ring`; css `box-shadow: var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
pub fn ring() -> Style {
    Style::new(css!("box-shadow: var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color);")).unwrap()
}

/// Tailwind `ring-4`; css `box-shadow: var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
pub fn ring_4() -> Style {
    Style::new(css!("box-shadow: var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color);")).unwrap()
}

/// Tailwind `ring-8`; css `box-shadow: var(--tw-ring-inset) 0 0 0 calc(8px + var(--tw-ring-offset-width)) var(--tw-ring-color);`
pub fn ring_8() -> Style {
    Style::new(css!("box-shadow: var(--tw-ring-inset) 0 0 0 calc(8px + var(--tw-ring-offset-width)) var(--tw-ring-color);")).unwrap()
}

/// Tailwind `ring-inset`; css `--tw-ring-inset: inset;`
pub fn ring_inset() -> Style {
    Style::new(css!("--tw-ring-inset: inset;")).unwrap()
}

