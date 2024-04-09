use scraper::{element_ref::ElementRef, Html, Node, Selector};
use std::env;
use std::io::prelude::*;
use std::{fs, fs::File};
use toml::{Table, Value};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[0] == "generate" {
        fs::create_dir_all("src/components").unwrap();
        let mut lib_file = File::create("src/components/mod.rs").unwrap();
        let config = read_config();
        if let Value::Table(table) = config["components"].clone() {
            table.iter().for_each(|component| {
                lib_file
                    .write_all(format!("pub mod {};\n", component.0).as_bytes())
                    .unwrap();
                write_mod(component);
            });
        }
    }
}

fn read_config() -> Table {
    let toml = fs::read_to_string("config.toml").unwrap();
    toml.parse::<Table>().unwrap()
}

fn write_mod(component: (&String, &Value)) {
    fs::create_dir_all(format!("src/components/{}", component.0)).unwrap();
    let mut mod_file = File::create(format!("src/components/{}/mod.rs", component.0)).unwrap();
    if let Value::Table(table) = component.1 {
        table.iter().for_each(|sub_component| {
            if let Value::Table(sub_component_details) = sub_component.1 {
                if let Value::String(url) = sub_component_details["url"].clone() {
                    mod_file
                        .write_all(format!("/// [{}]({})\n", url, url).as_bytes())
                        .unwrap();
                    write_stylesheet(url, component.0, sub_component.0);
                }
            }
            mod_file
                .write_all(format!("pub mod {};\n", sub_component.0).as_bytes())
                .unwrap();
        });
    }
}

fn write_stylesheet(url: String, component: &String, sub_component: &String) {
    let html = reqwest::blocking::get(url).unwrap().text().unwrap();

    let mut file =
        File::create(format!("src/components/{}/{}.rs", component, sub_component)).unwrap();
    file.write_all(b"use stylist::{css, Style};\n\n").unwrap();

    let fragment = Html::parse_fragment(html.as_str());
    let table_selector = Selector::parse("table").unwrap();

    let table_fragment = fragment.select(&table_selector).nth(0).unwrap();
    let selector = Selector::parse("tr").unwrap();

    for element in table_fragment.select(&selector) {
        let selector = Selector::parse("td").unwrap();
        let rows: Vec<ElementRef> = element.select(&selector).collect();
        if rows.len() >= 2 {
            let row_0 = rows[0]
                .children()
                .filter_map(|node| match node.value() {
                    Node::Text(text) => Some(&text[..]),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("");
            let row_1 = rows[1]
                .children()
                .filter_map(|node| match node.value() {
                    Node::Text(text) => Some(&text[..]),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("")
                .replace("\n", " ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            let doc_comment = format!("/// Tailwind `{}`; css `{}`", row_0, row_1);
            file.write_all(format!("{}\n", doc_comment).as_bytes())
                .unwrap();

            let function = format!(
                "pub fn {}() -> Style {{\n    Style::new(css!(\"{}\")).unwrap()\n}}\n",
                row_0
                    .replace("-", "_")
                    .replace("/", "d")
                    .replace("%", "p")
                    .replace(".", "_")
                    .replace("static", "r#static"),
                row_1.replace(r#"""#, r#"\""#),
            );
            file.write_all(format!("{}\n", function).as_bytes())
                .unwrap();
        }
    }
}
