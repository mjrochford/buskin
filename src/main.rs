pub mod ffz;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let emote_query = "pog";
    let emote_sort = "count";

    let url = format!(
        "https://api.frankerfacez.com/v1/emoticons?_sceheme=https&q={}&sort={}",
        emote_query,
        emote_sort
    );
    let emotes = reqwest::get(&url)
        .await?
        .json::<ffz::EmoticonsResponse>()
        .await?;

    for emote in emotes.emoticons {
        println!("{} {}", emote.name, emote.urls);
    }

    Ok(())
}
