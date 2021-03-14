use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
        Args,
    },
    http::AttachmentType,
};
use serde_json::Value;
use rand::{thread_rng, Rng};

#[command]
async fn xkcd(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let num = args.single::<u32>().unwrap_or(456789); // take the first argument and try to convert to u32. if fail, set to 456789 (for later)
                                                      // make https request with reqwest to find the number of most recent comic
    let resp = reqwest::get("https://xkcd.com/info.0.json")
        .await?
        .text()
        .await?;
    let json: Value = serde_json::from_str(&resp)?; // json deserialize
    let max_num: u32 = format!("{}", json["num"]).trim().parse().unwrap(); // format the max num into u32
    args.rewind();
    if num > max_num || num == 0 {
        // if the number is too high. this will also trigger when it becomes 456789 after failing to parse into u32
        if args.rest() == "" {
            // if no arguments, send latest comic
            print_xkcd(max_num, msg, ctx).await?;
        } else if args.single::<String>().unwrap() == "random" {
            // if argument is "random", send a random comic
            let rand_num = thread_rng().gen_range(0..max_num);
            print_xkcd(rand_num, msg, ctx).await?;
        } else {
            // finally, if the arguments were neither nothing nor random, this means that they
            // entered a number too large or less than zero (due to the nature of hte u32 type)
            let response = format!(
                "Please enter no arguments, 'random', or a number between 1 and {}.",
                max_num
            );
            msg.reply(&ctx.http, &response).await?;
        }
    } else {
        // if number in between 1 and max_num, send its corresponding comic.
        print_xkcd(num, msg, ctx).await?;
        return Ok(());
    }
    Ok(())
}

// send xkcd comics by passing a u32 for the comics number
async fn print_xkcd(num: u32, msg: &Message, ctx: &Context) -> CommandResult {
    let link = format!("https://xkcd.com/{}/info.0.json", num); // insert number into link for metadata request
    let comic = reqwest::get(link).await?.text().await?; // make https request
    let json: Value = serde_json::from_str(&comic)?; // json parse metadata
                                                     // set vars from metadata and format titles, dates, etc
    let title = format!(
        "**xkcd {}: {}**",
        json["num"].to_string(),
        rjq(json["safe_title"].to_string())
    );
    let date = format!(
        "{}-{}-{}",
        rjq(json["month"].to_string()),
        rjq(json["day"].to_string()),
        rjq(json["year"].to_string())
    );
    let image_link = rjq(json["img"].to_string());
    let desc = rjq(json["alt"].to_string());
    // send message with cool embed stuff and image link as an attachment
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(&title);
            m.embed(|e| {
                // e.title(&title);
                e.description(&date);
                e.image("attachment://&image_link");
                e.footer(|f| {
                    f.text(&desc);
                    f
                });
                e
            });
            m.add_file(AttachmentType::Image(&image_link));
            m
        })
        .await;
    Ok(())
}

#[inline]
// remove json quotes
fn rjq(s: String) -> String {
    let mut st = String::from(&s); // because mutable String passing weird
    st.truncate(st.len() - 1); // remove ending quote
    return st[1..].to_string(); // remove beginning quote
}
