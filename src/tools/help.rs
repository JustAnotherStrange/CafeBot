// Help command that uses embeds + reaction navigation
use crate::admin::admin_test::is_admin;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::time::Duration;
pub struct EditContent {
    pub title: String,
    pub description: String,
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    // needed in order for embed to be a reply
    let mut message = msg.reply(&ctx.http, "**Welcome to CafeBot!**").await?;
    // initial reactions
    let emojis: Vec<char> = vec!['🔧', '✂', '💲', '🌐', '🤴', '🛑'];
    for emoji in emojis.iter() {
        message.react(&ctx, *emoji).await?;
    }
    help_edit_embed(
        &ctx,
        &mut message,
        "__CafeBot Help__",
        "Please pick a page.",
    )
    .await;
    '_main: loop {
        if let Some(reaction) = message
            .await_reaction(&ctx)
            .timeout(Duration::from_secs(120)) // after 120 seconds without reactions, it will go to the "else" statement.
            .await
        {
            let emoji = &reaction.as_inner_ref().emoji;
            let mut response = EditContent {
                title: "React with one of the emojis to get to a help page.".to_string(),
                description: "".to_string(),
            };
            // match on the reacted emoji
            match emoji.as_data().as_str() {
                "🔧" => {
                    // TOOLS
                    response = EditContent {
                        title: "__Tools:__".to_string(),
                        description: TOOLS.to_string(),
                    };
                }
                "✂" => {
                    response = EditContent {
                        title: "__Message Modification:__".to_string(),
                        description: MESSAGE_MODIFICATION.to_string(),
                    };
                }
                "💲" => {
                    // MONEY
                    response = EditContent {
                        title: "__Money:__".to_string(),
                        description: MONEY.to_string(),
                    };
                }
                "🌐" => {
                    // MISC
                    response = EditContent {
                        title: "__Misc:__".to_string(),
                        description: MISC.to_string(),
                    }
                }
                "🤴" => {
                    // ADMIN
                    // Check if user is an admin and only give them the commands if they are.
                    response = if is_admin(ctx, msg).await {
                        EditContent {
                            title: "__CafeBot Admin:__".to_string(),
                            description: ADMIN.to_string(),
                        }
                    } else {
                        EditContent {
                            title: "__You can't access that.__".to_string(),
                            description: "Please pick a different page.".to_string(),
                        }
                    }
                }
                "🛑" => {
                    response = EditContent {
                        title: "Goodbye!".to_string(),
                        description: "This message is inactive.".to_string(),
                    };
                    help_edit_embed(
                        &ctx,
                        &mut message,
                        response.title.as_str(),
                        response.description.as_str(),
                    )
                    .await;
                    return Ok(()); // end
                }
                _ => {} // if the reaction is none of the above, then do nothing.
            }
            help_edit_embed(
                &ctx,
                &mut message,
                response.title.as_str(),
                response.description.as_str(),
            )
            .await;
        } else {
            // gets here if there were no reactions for 120 seconds.
            help_edit_embed(ctx, &mut message, "Goodbye!", "This message is inactive.").await;
            return Ok(()); // end
        }
    }
}

async fn help_edit_embed(ctx: &Context, message: &mut Message, title: &str, description: &str) {
    message
        .edit(&ctx, |m| {
            m.embed(|e| {
                e.title(&title);
                e.description(&description);
                e.footer(|f| f.text(FOOTER));
                e
            })
        })
        .await
        .unwrap();
}

// Help pages
const TOOLS: &str = "\
`^help` - show help pages. navigate through reactions
`^ping` - pong
`^profile` - show information about your user in the database
`^latency` - see latency to bot host.
`^custom` - create a custom command. e.g. '^custom bruh bruh moment'.
`^run [^r]` - run a custom command. e.g. '^r bruh'. Do '^r [command name] delete' to delete one (admin only) or '^r list' to list available commands.
as well, you can pipe the output into 'programs' using the `|` symbol.
`^wiki [search term]` - search wikipedia
`^xkcd` - get xkcd comics. run with no arguments for the latest, 'random' for a random comic, or a number of the specific one you want.";

const MESSAGE_MODIFICATION: &str = "\
`^say` - repeat anything that comes after this command
`^sarcasm [^s, ^/s]` - modify your message to bE lIkE tHiS.
`^owo` - modify your message to be owoified.";

const MONEY: &str = "\
`^wallet` - see how much money you have.
`^daily` - run this daily to maintain a streak. forgetting one day will result in a reset.
`^coin_flip [bet]` - flip a coin for money.
`^blackjack [^bj] [bet]` - play blackjack for money.
`^give_money [^give] [amount] [recipient]` - give money to someone.
`^rockpaperscissors [^rps] [move] [bet]` - play rock paper scissors! example: ^rps rock 50
`^leaderboard [choice]` - get a leaderboard. available options are currently 'money' and 'daily'.
`^pool` - see the pool for your server. its usage will soon be changed, right now it is 1/4th of all lost money";

const MISC: &str = "\
`^count` - count as high as you can
`^hair [^bald]` - see how bald you are.
`^balder [person1] [person2]` - compare 2 people to see how bald they are
`^zote` - find precepts of zote. ^zote [number] for a specific precept, ^zote random for a random one, and ^zote bald for our own precept.
`^bruh` - get a random bruh emote
`^dice [^d]` - roll dice using the standard dnd syntax; supports bonuses.
`^tictactoe [^ttt] [difficulty]` - play tic tac toe! difficulty is 1 through 100.";

const ADMIN: &str = "\
`^admin_test` - test if you are an admin
`^status [string]` - change the bot's status (will display as 'Playing [entered status]')
`^slow_mode [seconds]` - set the slow mode in that channel to a certain amount of seconds. set to 0 to turnoff slow mode.";

const FOOTER: &str = "\
Key: 🔧 - tools, ✂ - message modification, 💲 - money, 🌐 - miscellaneous, 🤴 - admin.
CafeBot v0.8.3: https://github.com/JustAnotherStrange/CafeBot";
