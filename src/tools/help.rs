// Simple help command.
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*
};

#[command]
async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // build the message
    let footer = "CafeBot v0.3.5: https://github.com/JustAnotherStrange/CafeBot";
    match args.rest() {
        "" => {
            let mut response = String::new();
            response.push_str("__Commands:__\n");
            response.push_str("`^help [page]` - show help pages. Specify no page for the general help or use one of the following categories: admin\n");
            response.push_str("`^ping` - pong\n");
            response.push_str("`^say` - repeat anything that comes after this command\n");
            response.push_str("`^count` - count as high as you can\n");
            response.push_str("`^hair [^bald]` - see how bald you are.\n");
            response.push_str("`^zote` - find precepts of zote. ^zote [number] for a specific precept, ^zote random for a random one, and ^zote bald for our own precept.\n");
            response.push_str("`^bruh` - get a random bruh emote\n");
            response.push_str("`^latency` - see latency to bot host.\n");
            response.push_str("`^sarcasm [^s, ^/s]` - modify your message to bE lIkE tHiS.\n");
            response.push_str("`^owo` - modify your message to be owoified.\n");
            response.push_str("`^daily` - run this daily to maintain a streak. forgetting one day will result in a reset.\n");
            response.push_str("`^xkcd` - get xkcd comics. run with no arguments for the latest, 'random' for a random comic, or a number of the specific one you want.\n");
            response.push_str("`^rockpaperscissors [^rps]` - play rock paper scissors! Example: ^rps rock\n");
            response.push_str("`^wiki [search term]` - search wikipedia\n");
            response.push_str("`^dice [^d]` - roll dice using the standard dnd syntax; supports bonuses.\n");
            response.push_str("`^custom` - create a custom command. e.g. '^custom bruh bruh moment'.\n");
            response.push_str("`^run [^r]` - run a custom command. e.g. '^r bruh'. Do '^r [command name] delete' to delete one (admin only) or '^r list' to list available commands. ");
            response.push_str("as well, you can pipe the output into 'programs' using the `|` symbol.\n");
            // send commands as an embed
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    // m.content("");
                    m.embed(|e| {
                        e.title("**Welcome to CafeBot!**");
                        e.description(&response);
                        e.footer(|f| {
                            f.text(&footer);
                            f
                        });
                        e
                    });
                    m
                })
                .await?;
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
                        let mut response = String::new();
                        response.push_str("__Commands:__\n");
                        response.push_str("`^admin_test` - test if you are an admin\n");
                        response.push_str("`^status [string]` - change the bot's status (will display as 'Playing [entered status]')\n");
                        response.push_str("`^slow_mode [seconds]` - set the slow mode in that channel to a certain amount of seconds. set to 0 to turnoff slow mode.\n");
                        // send commands as an embed
                        msg.channel_id
                            .send_message(&ctx.http, |m| {
                                // m.content("");
                                m.embed(|e| {
                                    e.title("**CafeBot Admin**");
                                    e.description(&response);
                                    e.footer(|f| {
                                        f.text(&footer);
                                        f
                                    });
                                    e
                                });
                                m
                            })
                            .await?;
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
