pub mod ffz;
use ffz::{Emoticon, EmoticonsResponse};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let emote_query = "Pog";
    let emote_sort = "count";

    let url = "https://api.frankerfacez.com/v1/emoticons?_sceheme=https";

    let client = reqwest::Client::new();

    let mut emotes = client.get(url)
        .query(&[("q", emote_query), ("sort", emote_sort)])
        .send()
        .await?
        .json::<EmoticonsResponse>()
        .await?;

    remove_copy_emotes(&mut emotes.emoticons);

    // emotes.emoticons.iter().for_each(|emote| println!("![{}]({} \"{}\")  ", emote.name, emote.urls, emote.name));
    emotes.emoticons.iter().for_each(|emote| println!("<img src=\"{}\" alt=\"{}\" width=24/> {}<br/>", emote.urls, emote.name, emote.name));

    Ok(())
}

fn remove_copy_emotes(emoticons: &mut Vec<Emoticon>) {
    let compare_emoticons = |one: &Emoticon, other: &Emoticon| -> std::cmp::Ordering { 
        if one.name == other.name  {
            return one.usage_count.cmp(&other.usage_count).reverse();
        }

        one.name.cmp(&other.name)
    };

    let remove_dups = |one: &mut Emoticon, other: &mut Emoticon| -> bool { 
        if one.name != other.name {
            return false;
        }

        return one.usage_count < other.usage_count
    };


    emoticons.sort_by(compare_emoticons);
    emoticons.dedup_by(remove_dups);

}
