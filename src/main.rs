use itertools::Itertools;
use std::env;
mod decklist;
mod tokens;

#[tokio::main]
async fn main() {
    let source: String = env::args().collect::<Vec<String>>().pop().unwrap();
    let card_names: Vec<String>;
    if source.ends_with(".txt") {
        card_names = decklist::read_from_file(source.as_str());
    } else if source.starts_with("http") {
        card_names = decklist::read_from_url(source.as_str());
    } else {
        panic!("Expected txt file or url containing decklist.")
    }
    let mut token_ids: Vec<String> = vec![];
    for name in card_names {
        let mut ids = tokens::get_related_token_ids(name.as_str())
            .await
            .unwrap_or_default();
        token_ids.append(&mut ids);
    }
    println!(
        "{:?}",
        token_ids.into_iter().unique().collect::<Vec<String>>()
    )
}
