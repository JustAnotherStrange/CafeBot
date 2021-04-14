use rand::{thread_rng, Rng};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
async fn game(ctx: &Context, msg: &Message) -> CommandResult {
    let turn: u32 = thread_rng().gen_range(1..12);
    let response = match turn {
        1 => "You have brought something terrible onto this planet. What you have done is unforgivable. But I, a measly bot, must also comply with the rules. I HAVE LOST THE GAME.",
        2 => "fuck you, i lost the game",
        3 => "what have u brought upon this cursed land\ni lost the game",
        4 => "ew\n\ni lost the game",
        5 => "WHY ME? WHY MEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE i lost the game",
        6 => "u fool\nur bad\nur trash\ni lost the game",
        7 => "heyyyyyyyy\ni lost the game",
        8 => "I 🤦‍♂️🙎‍♂️🧔👶🧑LOST 😤😋😎😕🤣😛😏🏛🗻🚂🗼🛥🚉🚕🎗🎧THE 🈚️✴️📵🔅⛎🖤⚜️🚾🆒#️⃣GAME🏑🎽🥌🏓⚽️🎫🎖🏆SEND THIS TO 50 FRIENDS📲📲📲5️⃣0️⃣TO UNDO THE CURSE🧙‍♀️🧙‍♂️🎢🎩🗾🏨🏘⏲📹☎️",
        9 => "I.\nLost.\nThe.\nGame.",
        10 => "gg i guess we're both noobs at this game. wanna play minecraft instead?",
        11 => "sorry for your loss https://cdn.discordapp.com/attachments/646474285116030992/679674483472334858/3pnaex_1.jpg",
        _ => unreachable!(),
    };
    msg.reply(&ctx.http, &response).await?;
    Ok(())
}
