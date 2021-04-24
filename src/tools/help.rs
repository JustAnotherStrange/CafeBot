// Simple help command.
use crate::admin::admin_test::is_admin;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // build the message
    let footer = "CafeBot v0.5.0: https://github.com/JustAnotherStrange/CafeBot";
    match args.rest() {
        "" => {
            let response = "__Commands:__
            `^help [page]` - show help pages. Specify no page for the general help or use one of the following categories: admin
            `^ping` - pong
            `^say` - repeat anything that comes after this command
            `^count` - count as high as you can
            `^hair [^bald]` - see how bald you are.
            `^zote` - find precepts of zote. ^zote [number] for a specific precept, ^zote random for a random one, and ^zote bald for our own precept.
            `^bruh` - get a random bruh emote
            `^latency` - see latency to bot host.
            `^sarcasm [^s, ^/s]` - modify your message to bE lIkE tHiS.
            `^owo` - modify your message to be owoified.
            `^daily` - run this daily to maintain a streak. forgetting one day will result in a reset.
            `^xkcd` - get xkcd comics. run with no arguments for the latest, 'random' for a random comic, or a number of the specific one you want.
            `^rockpaperscissors [^rps] [move] [bet]` - play rock paper scissors! Example: ^rps rock 50
            `^wiki [search term]` - search wikipedia
            `^dice [^d]` - roll dice using the standard dnd syntax; supports bonuses.
            `^custom` - create a custom command. e.g. '^custom bruh bruh moment'.
            `^run [^r]` - run a custom command. e.g. '^r bruh'. Do '^r [command name] delete' to delete one (admin only) or '^r list' to list available commands.
            as well, you can pipe the output into 'programs' using the `|` symbol.
            `^wallet` - see how much money you have.
            `^coin_flip [bet]` - flip a coin for money.
            `^give_money [^give] [amount] [recipient]` - give money to someone.
            `^blackjack` - play blackjack!";
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
            if is_admin(ctx, msg).await {
                let response = "__Commands:__
                `^admin_test` - test if you are an admin
                `^status [string]` - change the bot's status (will display as 'Playing [entered status]')
                `^slow_mode [seconds]` - set the slow mode in that channel to a certain amount of seconds. set to 0 to turnoff slow mode.";
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
