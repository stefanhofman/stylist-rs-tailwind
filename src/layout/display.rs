use stylist::{css, Style};

pub fn block() -> Style {
    Style::new(css!("display: block;")).unwrap()
}

pub fn inline_block() -> Style {
    Style::new(css!("display: inline-block;")).unwrap()
}

pub fn inline() -> Style {
    Style::new(css!("display: inline;")).unwrap()
}

pub fn flex() -> Style {
    Style::new(css!("display: flex;")).unwrap()
}

pub fn inline_flex() -> Style {
    Style::new(css!("display: inline-flex;")).unwrap()
}

pub fn table() -> Style {
    Style::new(css!("display: table;")).unwrap()
}

pub fn inline_table() -> Style {
    Style::new(css!("display: inline-table;")).unwrap()
}

pub fn table_caption() -> Style {
    Style::new(css!("display: table-caption;")).unwrap()
}

pub fn table_cell() -> Style {
    Style::new(css!("display: table-cell;")).unwrap()
}

pub fn table_column() -> Style {
    Style::new(css!("display: table-column;")).unwrap()
}

pub fn table_column_group() -> Style {
    Style::new(css!("display: table-column-group;")).unwrap()
}

pub fn table_footer_group() -> Style {
    Style::new(css!("display: table-footer-group;")).unwrap()
}

pub fn table_header_group() -> Style {
    Style::new(css!("display: table-header-group;")).unwrap()
}

pub fn table_row_group() -> Style {
    Style::new(css!("display: table-row-group;")).unwrap()
}
