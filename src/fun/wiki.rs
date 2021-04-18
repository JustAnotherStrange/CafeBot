use crate::rjq;
use serde_json::Value;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
#[command]
async fn wiki(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let choice = args.rest().to_string();
    let link = format!("https://en.wikipedia.org/w/api.php?action=query&origin=*&format=json&generator=search&gsrnamespace=0&gsrlimit=5&gsrsearch='{}'", choice);
    let json: Value = serde_json::from_str(&reqwest::get(&link).await?.text().await?).unwrap();
    let mut ids: Vec<&String> = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    for (k, v) in match json["query"]["pages"].as_object() {
        Some(x) => x,
        None => {
            let to_send = format!("No Wikipedia results for *{}*.", choice);
            msg.reply(&ctx.http, &to_send).await?;
            return Ok(());
        }
    }
    .iter()
    {
        // v is the json value, k is the id of the page
        ids.push(k);
        titles.push(rjq(v["title"].to_string()));
    }
    // print article names
    let mut descs = String::new();
    for i in 0..titles.len() {
        let link_url = format!("https://en.wikipedia.org/w/api.php?action=query&prop=info&format=json&pageids={}&inprop=url", ids[i]);
        let json_url: Value =
            serde_json::from_str(&reqwest::get(&link_url).await?.text().await?).unwrap();
        let article_url = json_url["query"]["pages"][ids[i]]["fullurl"]
            .as_str()
            .unwrap();
        let temp: String = format!("{}. **{}**: {}\n", i + 1, titles[i], article_url);
        descs.push_str(&temp);
    }
    let title = format!("**Wikipedia results for *{}:***", choice);
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content(&title);
            m.embed(|e| {
                // e.title(&title);
                e.description(&descs);
                e
            });
            m
        })
        .await?;
    Ok(())
}
