use stylist::{css, Style};

/// Tailwind `whitespace-normal`; css `white-space: normal;`
pub fn whitespace_normal() -> Style {
    Style::new(css!("white-space: normal;")).unwrap()
}

/// Tailwind `whitespace-nowrap`; css `white-space: nowrap;`
pub fn whitespace_nowrap() -> Style {
    Style::new(css!("white-space: nowrap;")).unwrap()
}

/// Tailwind `whitespace-pre`; css `white-space: pre;`
pub fn whitespace_pre() -> Style {
    Style::new(css!("white-space: pre;")).unwrap()
}

/// Tailwind `whitespace-pre-line`; css `white-space: pre-line;`
pub fn whitespace_pre_line() -> Style {
    Style::new(css!("white-space: pre-line;")).unwrap()
}

/// Tailwind `whitespace-pre-wrap`; css `white-space: pre-wrap;`
pub fn whitespace_pre_wrap() -> Style {
    Style::new(css!("white-space: pre-wrap;")).unwrap()
}

/// Tailwind `whitespace-break-spaces`; css `white-space: break-spaces;`
pub fn whitespace_break_spaces() -> Style {
    Style::new(css!("white-space: break-spaces;")).unwrap()
}

