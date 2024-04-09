use stylist::{css, Style};

/// Tailwind `divide-solid `; css `border-style: solid;`
pub fn divide_solid () -> Style {
    Style::new(css!("border-style: solid;")).unwrap()
}

/// Tailwind `divide-dashed `; css `border-style: dashed;`
pub fn divide_dashed () -> Style {
    Style::new(css!("border-style: dashed;")).unwrap()
}

/// Tailwind `divide-dotted `; css `border-style: dotted;`
pub fn divide_dotted () -> Style {
    Style::new(css!("border-style: dotted;")).unwrap()
}

/// Tailwind `divide-double `; css `border-style: double;`
pub fn divide_double () -> Style {
    Style::new(css!("border-style: double;")).unwrap()
}

/// Tailwind `divide-none `; css `border-style: none;`
pub fn divide_none () -> Style {
    Style::new(css!("border-style: none;")).unwrap()
}

