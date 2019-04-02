use std::io;
use unic::segment::Graphemes;
use unic::segment::Words;
use unic::ucd::{name_aliases_of, GraphemeClusterBreak, Lowercase, Name, NameAliasType};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
struct CharInfo {
    char: char,
    display: String,
    name: String,
    is_lowercase: bool,
    grapheme_cluster_break: String,
}

impl CharInfo {
    fn from_char(c: char) -> Self {
        CharInfo {
            char: c,
            display: char_display(c),
            name: char_name(c),
            is_lowercase: Lowercase::of(c).as_bool(),
            grapheme_cluster_break: format!("{:?}", GraphemeClusterBreak::of(c)),
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    for word in Words::new(&buffer, |_| true) {
        println!("word");
        for grapheme_cluster in Graphemes::new(word) {
            println!("\tcluster");
            for char_info in grapheme_cluster.chars().map(CharInfo::from_char) {
                println!("\t\t{:?}", char_info);
            }
        }
    }

    Ok(())
}

fn char_display(c: char) -> String {
    if c.is_whitespace() {
        c.escape_default().collect::<String>()
    } else {
        c.to_string()
    }
}

fn char_name(c: char) -> String {
    Name::of(c)
        .map(|n| n.to_string())
        .or_else(|| char_name_abbreviations(c))
        .unwrap_or_else(|| "<none>".to_owned())
}

fn char_name_abbreviations(c: char) -> Option<String> {
    name_aliases_of(c, NameAliasType::NameAbbreviations).map(|abbrs| abbrs[0].to_owned())
}
