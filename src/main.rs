use scraper::{element_ref::ElementRef, Html, Selector};
use std::io::prelude::*;
use std::{env, fs, fs::File};

fn main() {
    let args: Vec<String> = env::args().collect();

    let html = fs::read_to_string(args[1].clone()).unwrap();
    let mut file = File::create(format!("src/{}/{}.rs", args[2].clone(), args[3].clone())).unwrap();
    file.write_all(b"use stylist::{css, Style};\n\n").unwrap();

    let fragment = Html::parse_fragment(html.as_str());
    let selector = Selector::parse("tr").unwrap();

    for element in fragment.select(&selector) {
        let selector = Selector::parse("td").unwrap();
        let row: Vec<ElementRef> = element.select(&selector).collect();
        if row.len() == 2 {
            let doc_comment = format!(
                "/// Tailwind `{}`; css `{}`",
                row[0].inner_html(),
                row[1].inner_html().replace("\n", " ")
            );
            file.write_all(format!("{}\n", doc_comment).as_bytes())
                .unwrap();

            let function = format!(
                "pub fn {}() -> Style {{\n    Style::new(css!(\"{}\")).unwrap()\n}}\n",
                row[0].inner_html().replace("-", "_").replace("/", "d"),
                row[1]
                    .inner_html()
                    .replace("\n", " ")
                    .replace(r#"""#, r#"\""#),
            );
            file.write_all(format!("{}\n", function).as_bytes())
                .unwrap();
        }
    }
}
