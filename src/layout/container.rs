use stylist::{css, Style};

pub fn container() -> Style {
    Style::new(css!("width: 100%;")).expect("Malformed style: container")
}
