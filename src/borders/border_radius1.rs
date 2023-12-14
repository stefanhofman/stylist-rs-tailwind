use stylist::{css, Style};

/// Tailwind `rounded-none`; css `border-radius: 0px`
pub fn rounded_none() -> Style {
    Style::new(css!("border-radius: 0px;")).unwrap()
}

/// Tailwind `rounded-sm`; css `border-radius: 0.125rem`
pub fn rounded_sm() -> Style {
    Style::new(css!("border-radius: 0.125rem;")).unwrap()
}

/// Tailwind `rounded`; css `border-radius: 0.25rem`
pub fn rounded() -> Style {
    Style::new(css!("border-radius: 0.25rem;")).unwrap()
}

/// Tailwind `rounded-md`; css `border-radius: 0.375rem`
pub fn rounded_md() -> Style {
    Style::new(css!("border-radius: 0.375rem;")).unwrap()
}

/// Tailwind `rounded-lg`; css `border-radius: 0.5rem`
pub fn rounded_lg() -> Style {
    Style::new(css!("border-radius: 0.5rem;")).unwrap()
}

/// Tailwind `rounded-xl`; css `border-radius: 0.75rem`
pub fn rounded_xl() -> Style {
    Style::new(css!("border-radius: 0.75rem;")).unwrap()
}

/// Tailwind `rounded-2xl`; css `border-radius: 1rem`
pub fn rounded_2xl() -> Style {
    Style::new(css!("border-radius: 1rem;")).unwrap()
}

/// Tailwind `rounded-3xl`; css `border-radius: 1.5rem`
pub fn rounded_3xl() -> Style {
    Style::new(css!("border-radius: 1.5rem;")).unwrap()
}

/// Tailwind `rounded-full`; css `border-radius: 9999px`
pub fn rounded_full() -> Style {
    Style::new(css!("border-radius: 9999px;")).unwrap()
}

////////////////////////////////////////////////////////////////

/// Tailwind `rounded-t-none`; css `border-top-left-radius: 0px; border-top-right-radius: 0px`
pub fn rounded_t_none() -> Style {
    Style::new(css!(
        r#"
            border-top-left-radius: 0px;
            border-top-right-radius: 0px;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-t-sm`; css `border-top-left-radius: 0.125rem; border-top-right-radius:
/// 0.125rem`
pub fn rounded_t_sm() -> Style {
    Style::new(css!(
        r#"
            border-top-left-radius: 0.125rem;
            border-top-right-radius: 0.125rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-t`; css `border-top-left-radius: 0.25rem; border-top-right-radius:
/// 0.25rem`
pub fn rounded_t() -> Style {
    Style::new(css!(
        r#"
            border-top-left-radius: 0.25rem;
            border-top-right-radius: 0.25rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-t-md`; css `border-top-left-radius: 0.375rem; border-top-right-radius:
/// 0.375rem`
pub fn rounded_t_md() -> Style {
    Style::new(css!(
        r#"
            border-top-left-radius: 0.375rem;
            border-top-right-radius: 0.375rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-t-lg`; css `border-top-left-radius: 0.5rem; border-top-right-radius:
/// 0.5rem`
pub fn rounded_t_lg() -> Style {
    Style::new(css!(
        r#"
            border-top-left-radius: 0.5rem;
            border-top-right-radius: 0.5rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-t-xl`; css `border-top-left-radius: 0.75rem; border-top-right-radius:
/// 0.75rem`
pub fn rounded_t_xl() -> Style {
    Style::new(css!(
        r#"
            border-top-left-radius: 0.75rem;
            border-top-right-radius: 0.75rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-t-2xl`; css `border-top-left-radius: 1rem; border-top-right-radius:
/// 1rem`
pub fn rounded_t_2xl() -> Style {
    Style::new(css!(
        r#"
            border-top-left-radius: 1rem;
            border-top-right-radius: 1rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-t-3xl`; css `border-top-left-radius: 1.5rem; border-top-right-radius:
/// 1.5rem`
pub fn rounded_t_3xl() -> Style {
    Style::new(css!(
        r#"
            border-top-left-radius: 1.5rem;
            border-top-right-radius: 1.5rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-t-full`; css `border-top-left-radius: 9999px; border-top-right-radius:
/// 9999px`
pub fn rounded_t_full() -> Style {
    Style::new(css!(
        r#"
            border-top-left-radius: 9999px;
            border-top-right-radius: 9999px;
        "#
    ))
    .unwrap()
}

////////////////////////////////////////////////////////////////

/// Tailwind `rounded-r-none`; css `border-top-right-radius: 0px; border-bottom-right-radius: 0px`
pub fn rounded_r_none() -> Style {
    Style::new(css!(
        r#"
            border-top-right-radius: 0px;
            border-bottom-right-radius: 0px;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-r-sm`; css `border-top-right-radius: 0.125rem; border-bottom-right-radius:
/// 0.125rem`
pub fn rounded_r_sm() -> Style {
    Style::new(css!(
        r#"
            border-top-right-radius: 0.125rem;
            border-bottom-right-radius: 0.125rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-t`; css `border-top-right-radius: 0.25rem; border-bottom-right-radius:
/// 0.25rem`
pub fn rounded_r() -> Style {
    Style::new(css!(
        r#"
            border-top-right-radius: 0.25rem;
            border-bottom-right-radius: 0.25rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-r-md`; css `border-top-right-radius: 0.375rem; border-bottom-right-radius:
/// 0.375rem`
pub fn rounded_r_md() -> Style {
    Style::new(css!(
        r#"
            border-top-right-radius: 0.375rem;
            border-bottom-right-radius: 0.375rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-r-lg`; css `border-top-right-radius: 0.5rem; border-bottom-right-radius:
/// 0.5rem`
pub fn rounded_r_lg() -> Style {
    Style::new(css!(
        r#"
            border-top-right-radius: 0.5rem;
            border-bottom-right-radius: 0.5rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-r-xl`; css `border-top-right-radius: 0.75rem; border-bottom-right-radius:
/// 0.75rem`
pub fn rounded_r_xl() -> Style {
    Style::new(css!(
        r#"
            border-top-right-radius: 0.75rem;
            border-bottom-right-radius: 0.75rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-r-2xl`; css `border-top-right-radius: 1rem; border-bottom-right-radius:
/// 1rem`
pub fn rounded_r_2xl() -> Style {
    Style::new(css!(
        r#"
            border-top-right-radius: 1rem;
            border-bottom-right-radius: 1rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-r-3xl`; css `border-top-right-radius: 1.5rem; border-bottom-right-radius:
/// 1.5rem`
pub fn rounded_r_3xl() -> Style {
    Style::new(css!(
        r#"
            border-top-right-radius: 1.5rem;
            border-bottom-right-radius: 1.5rem;
        "#
    ))
    .unwrap()
}

/// Tailwind `rounded-r-full`; css `border-top-right-radius: 9999px; border-bottom-right-radius:
/// 9999px`
pub fn rounded_r_full() -> Style {
    Style::new(css!(
        r#"
            border-top-right-radius: 9999px;
            border-bottom-right-radius: 9999px;
        "#
    ))
    .unwrap()
}
