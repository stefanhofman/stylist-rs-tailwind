use stylist::{css, Style};

/// Tailwind `list-image-none`; css `list-style-image: none;`
pub fn list_image_none() -> Style {
    Style::new(css!("list-style-image: none;")).unwrap()
}

