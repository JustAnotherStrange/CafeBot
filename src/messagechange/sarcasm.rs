// sarcasm command for tExT lIkE tHiS. By g_w1
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::messagechange::modify;

#[command]
#[aliases("s", "/s")]
async fn sarcasm(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut sarcasted = sarcastify(&args.rest());
    sarcasted.insert_str(0, "@: ");
    sarcasted.insert_str(1, &msg.author.name);
    modify::modify(ctx, msg, &sarcasted).await?;
    Ok(())
}

fn sarcastify(to_sarc: &str) -> String {
    let mut sarcasted = String::new();
    let mut cap: bool = true;
    for cur in to_sarc.chars() {
        // Make it be alternating caps/lowercase
        cap = !cap;
        // if it can't be uppercase, just use the same char
        let to_push = if cap {
            cur.to_uppercase().nth(0).unwrap_or(cur)
        } else {
            cur.to_lowercase().nth(0).unwrap_or(cur)
        };
        sarcasted.push(to_push);
    }
    sarcasted
}
