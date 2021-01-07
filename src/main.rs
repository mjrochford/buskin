pub mod ffz;
use ffz::{Emoticon, EmoticonsResponse};

use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_file_path = "pog.md";
    let emote_query = "kekw";
    let emote_sort = "count";

    let url = "https://api.frankerfacez.com/v1/emoticons?_sceheme=https";

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .query(&[
            ("q", emote_query),
            ("sort", emote_sort),
            ("per_page", "200"),
        ])
        .send()
        .await?;
    println!("{:?} remaining", response.headers().get("ratelimit-remaining").unwrap());

    let mut emotes = response.json::<EmoticonsResponse>().await?;

    remove_copy_emotes(&mut emotes.emoticons);

    let mut out_file = File::create(out_file_path)?;
    // emotes.emoticons.iter().for_each(|emote| println!("![{}]({} \"{}\")  ", emote.name, emote.urls, emote.name));
    for (i, emote) in emotes.emoticons.iter().enumerate() {
        writeln!(
            out_file,
            "<img src=\"{}\" title=\"{}\" width=24/> {}<br/>",
            emote.urls, emote.name, i
        )?;
    }

    Ok(())
}

fn remove_copy_emotes(emoticons: &mut Vec<Emoticon>) {
    let compare_emoticons = |one: &Emoticon, other: &Emoticon| -> std::cmp::Ordering {
        if one.name == other.name {
            return one.usage_count.cmp(&other.usage_count).reverse();
        }

        one.name.cmp(&other.name)
    };

    let remove_dups = |one: &mut Emoticon, other: &mut Emoticon| -> bool {
        if one.name != other.name {
            return false;
        }

        return one.usage_count < other.usage_count;
    };

    emoticons.sort_by(compare_emoticons);
    emoticons.dedup_by(remove_dups);
}
