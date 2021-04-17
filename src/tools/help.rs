// Simple help command.
// I tried to make it use embeds but it was a hassle and didn't work after a lot of debugging.
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};

#[command]
async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // build the message
    match args.rest() {
        "" => {
            let response = MessageBuilder::new()
                .push_bold_safe("Welcome to CafeBot!\n \n")
                .push("Commands:\n")
                .push("^help [page] - show help pages. Specify no page for the general help or use one of the following categories: admin\n")
                .push("^ping - pong\n")
                .push("^say - repeat anything that comes after this command\n")
                .push("^count - count as high as you can\n")
                .push("^hair [^bald] - see how bald you are.\n")
                .push("^zote - find precepts of zote. ^zote [number] for a specific precept, ^zote random for a random one, and ^zote bald for our own precept.\n")
                .push("^bruh - get a random bruh emote\n")
                .push("^latency - see latency to bot host.\n")
                .push("^sarcasm [^s, ^/s] - modify your message to bE lIkE tHiS.\n")
                .push("^owo - modify your message to be owoified.\n")
                .push("^daily - run this daily to maintain a streak. forgetting one day will result in a reset.\n")
                .push("^xkcd - get xkcd comics. run with no arguments for the latest, 'random' for a random comic, or a number of the specific one you want.\n")
                .push("^rockpaperscissors [^rps] - play rock paper scissors! Example: ^rps rock\n")
                .push("^wiki [search term] - search wikipedia\n")
                .build();
            msg.reply(&ctx.http, &response).await?;
        }
        "admin" => {
            if let Some(member) = &msg.member {
                // only let admins ask for admin help
                for role in &member.roles {
                    if role
                        .to_role_cached(&ctx.cache)
                        .await
                        .map_or(false, |r| r.has_permission(Permissions::ADMINISTRATOR))
                    {
                        let response = MessageBuilder::new()
                            .push_bold_safe("CafeBot admin\n \n")
                            .push("^admin_test - test if you are an admin\n")
                            .push("^status [string] - change the bot's status (will display as 'Playing [entered status]')\n")
                            .push("^slow_mode [seconds] - set the slow mode in that channel to a certain amount of seconds. Set to 0 to turnoff slow mode.\n")
                            .build();
                        msg.reply(&ctx.http, &response).await?;
                        break;
                    }
                }
            } else {
                msg.reply(&ctx.http, "You can't access this help page")
                    .await?;
            }
        }
        _ => {
            msg.reply(&ctx.http, "Please enter either no category for general help or one of these categories: admin.").await?;
        }
    }
    Ok(())
}
