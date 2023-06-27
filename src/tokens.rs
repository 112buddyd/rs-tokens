use scryfall::card::Card;
use scryfall::error::Error;

pub async fn get_related_token_ids(name: &str) -> Result<Vec<String>, Error> {
    let card = Card::named(name).await?;
    if card.all_parts.is_none() {
        return Ok(vec![]);
    }
    Ok(card
        .all_parts
        .unwrap()
        .iter()
        .filter(|c| c.type_line.contains("Token"))
        .map(|c| c.id.to_string())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::get_related_token_ids;

    #[tokio::test]
    async fn error_if_bad_name() {
        assert!(get_related_token_ids("FakeCard9000").await.is_err());
    }

    #[tokio::test]
    async fn has_no_tokens() {
        assert_eq!(
            get_related_token_ids("Wrath of God").await.unwrap().len(),
            0
        );
    }

    #[tokio::test]
    async fn has_one_token() {
        assert_eq!(
            get_related_token_ids("Raise the Alarm")
                .await
                .unwrap()
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn has_two_tokens() {
        assert_eq!(
            get_related_token_ids("Rin and Seri, Inseparable")
                .await
                .unwrap()
                .len(),
            2
        );
    }
}
