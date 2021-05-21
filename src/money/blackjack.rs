// play blackjack for money
// All throughout: `msg` variable is the message the user sent, and `message` is the one the bot sent.
use crate::database::database::{lost_increment, money_increment_without_lost};
use crate::{
    database::database::{gen_connection, get_money, money_increment_with_lost},
    tools::stats::init_stats_if_necessary,
};
use rand::{seq::SliceRandom, thread_rng};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
    Error,
};
use std::{thread::sleep, time, time::Duration};
use serenity::model::user::User;

#[command]
#[only_in(guilds)]
#[aliases("bj")]
async fn blackjack(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // increase game counter
    let conn = gen_connection();
    init_stats_if_necessary(&conn);
    conn.execute("update stats set blackjacks = blackjacks + 1", [])?;
    // parse bet amount
    let bet: i32 = match args.rest().trim().parse() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(&ctx.http, "Please enter an amount to bet as an argument.")
                .await?;
            return Ok(());
        }
    };
    if bet > get_money(&msg.author)? || bet < 0 {
        msg.reply(&ctx.http, "You can't bet more money than you have.")
            .await?;
        return Ok(());
    }

    // subtract money from the beginning so no negative money
    money_increment_without_lost(&msg.author, -bet)?;

    // reply a message first and edit it with the embed, as a workaround to make the embed message be a reply
    let response = format!(
        "**{}** bet **{}** monies on blackjack.",
        &msg.author.name, bet
    );
    let mut message = msg.reply(&ctx.http, &response).await?;

    // initial embed
    // call the blackjack engine
    blackjack_engine(ctx, &mut message, msg, bet).await?;
    Ok(())
}

// Edits the embed with a new title and a new description.
pub async fn edit_embed(ctx: &Context, message: &mut Message, title: &str, description: &str) {
    message
        .edit(&ctx, |m| {
            m.embed(|e| {
                e.title(&title);
                e.description(&description);
                e
            })
        })
        .await
        .unwrap();
}

// Here is the game itself, which is actually a modified version of a previous blackjack game I wrote for the CLI.
async fn blackjack_engine(
    ctx: &Context,
    message: &mut Message,
    msg: &Message,
    bet: i32,
) -> Result<(), Error> {
    let quarter_second = time::Duration::from_millis(250); // for sleeps later on
    let mut deck = deck_gen(); //

    // deal the deck
    let mut hand1: Vec<usize> = Vec::new();
    let mut hand2: Vec<usize> = Vec::new();
    hand1.push(deck.pop().unwrap());
    hand1.push(deck.pop().unwrap());
    hand2.push(deck.pop().unwrap());
    hand2.push(deck.pop().unwrap());
    // fixes the infamous 22 bug
    if hand1[0] == 11 && hand1[1] == 11 {
        'deck1: for i in hand1.iter_mut() {
            if i == &11 {
                *i = 1;
                break 'deck1;
            }
        }
    }
    if hand2[0] == 11 && hand2[1] == 11 {
        'deck2: for i in hand2.iter_mut() {
            if i == &11 {
                *i = 1;
                break 'deck2;
            }
        }
    }
    let mut sum1: usize;
    let mut sum2: usize;

    // Bot's reactions, so the user knows what to do.
    message
        .edit(&ctx, |m| {
            m.embed(|e| {
                e.title("Loading...");
                e
            })
        })
        .await?;

    let letters: Vec<char> = vec!['✋', '🛑'];
    for letter in letters.iter() {
        message.react(ctx, *letter).await?;
    }
    let mut stay = false;

    // set the title of the embed to be a nicely formatted display of the game status.
    let new_title = format_game_status(None, hand1.clone(), hand2.clone(), false);
    edit_embed(ctx, message, new_title.as_str(), "Hit or stay? (React)").await;
    // large loop, where the gameplay happens
    'main: loop {
        // player turn
        'not_stay: while !stay {
            // await reactions and then match on them
            if let Some(reaction) = message
                .await_reaction(&ctx)
                .timeout(Duration::from_secs(60)) // after 60 seconds without reactions, it will go to the "else" statement.
                .await
            {
                let emoji = &reaction.as_inner_ref().emoji;
                let reacted = &*reaction.as_inner_ref().clone();
                if reacted.user(&ctx).await? != msg.author {
                    continue 'not_stay;
                }
                // match on the reacted emoji
                let _ = match emoji.as_data().as_str() {
                    // hit!
                    "✋" => {
                        hand1.push(deck.pop().unwrap()); // take a card from the deck and put it into the player's hand.
                        sum1 = hand1.iter().sum(); // generate sum.

                        // if player's sum is greater than 21
                        if sum1 > 21 {
                            // if there is an ace (11), change it to a 1.
                            if hand1.contains(&11) {
                                for i in hand1.iter_mut() {
                                    if i == &11 {
                                        *i = 1;
                                        break; // according to logic, this should break 'not_stay. Not sure tho.
                                    }
                                }
                            // if there is not an ace, break.
                            } else {
                                break; // same as above.
                            }
                        } else if sum1 == 21 {
                            break; // should i maybe specify which loop these are breaking for clarity? problem is, i don't know which one they are breaking ;-;
                        }

                        // If it gets here, the sum is less than 21.
                        // update embed with new game status
                        let new_title =
                            format_game_status(None, hand1.clone(), hand2.clone(), false);
                        edit_embed(ctx, message, new_title.as_str(), "Hit or stay? (React)").await;
                    }
                    "🛑" => {
                        // stay
                        stay = true; // this breaks the while loop. everywhere else, I just do `break.` hm...
                    }
                    _ => {} // if the reaction is neither a hand nor a stop sign, then do nothing.
                };
            } else {
                // gets here if there were no reactions for 60 seconds.
                let new_description = format!("One minute passed with no reactions, so the game ended with no results. As a result, you lost your bet, which was **{}** monies.", bet);

                lost_increment(*msg.guild_id.unwrap().as_u64(), bet, &gen_connection()).unwrap();
                edit_embed(ctx, message, "Timed out.", &*new_description).await;
                return Ok(()); // end the game
            }
        }

        // outside of 'not_stay while loop, so the player is stayed, busted, or blackjack.
        // Checks
        sleep(quarter_second); // necessary? yeah, i think so.
        sum1 = hand1.iter().sum(); // update player sum
        sum2 = hand2.iter().sum();

        // i think this big chunk of checks the same as is in the 'not_stay while loop. hm.
        if sum1 > 21 {
            // if there is an ace as an 11, change it to a 1.
            if hand1.contains(&11) {
                for i in hand1.iter_mut() {
                    if i == &11 {
                        *i = 1;
                        break; // breaks big loop, because outside of while loop.
                    }
                }
            } else {
                // the sum is greater than 21, and there are no aces as 11s. so, they player has bust.
                edit_embed(ctx, message, "Bust!", "Bust").await;
                sleep(Duration::from_millis(500));
                dealer_win(ctx, message, msg, hand1.clone(), hand2.clone(), bet).await;
                break;
            }
        } else if sum1 == 21 {
            if sum2 == 21 {
                tie(ctx, message, hand1, hand2, &msg.author, bet).await;
                return Ok(());
            }
            edit_embed(ctx, message, "Blackjack!", "Blackjack!").await;
            sleep(Duration::from_millis(500));
            player_win(ctx, message, msg, hand1.clone(), hand2.clone(), bet * 3).await;
            break;
        }

        // dealer turn
        if sum2 < 17 {
            hand2.push(deck.pop().unwrap()); // deal a card from deck
                                             // update status
            let new_title = format_game_status(None, hand1.clone(), hand2.clone(), true);
            edit_embed(ctx, message, new_title.as_str(), "Dealer's turn.").await;
        } else if sum2 >= 17 {
            if sum2 > 21 {
                // if there is an ace as an 11, change it to a 1.
                if hand1.contains(&11) {
                    for i in hand1.iter_mut() {
                        if i == &11 {
                            *i = 1;
                            break; // breaks big loop, because outside of while loop.
                        }
                    }
                }
            }
            // Dealer has to stay at greater or equal to 17.
            edit_embed(ctx, message, "The dealer stays.", "The dealer stays.").await;
            sleep(quarter_second);
            if stay {
                // this can probably be simplified.
                match test_win(hand1.clone(), hand2.clone()).as_str() {
                    "win" => {
                        player_win(ctx, message, msg, hand1.clone(), hand2.clone(), bet * 2).await
                    }
                    "lose" => {
                        dealer_win(ctx, message, msg, hand1.clone(), hand2.clone(), bet).await
                    }
                    "tie" => tie(ctx, message, hand1, hand2, &msg.author, bet).await,
                    _ => {}
                }
                break;
            }
        }
        sleep(quarter_second); // necessary? i think so.

        // Final checks
        // Same checks again!?!? Me one month ago writing the engine for this was being pretty bald ngl.
        sum1 = hand1.iter().sum();
        sum2 = hand2.iter().sum();
        if sum2 > 21 {
            // 11 checking.
            if hand2.contains(&11) {
                for i in hand2.iter_mut() {
                    if i == &11 {
                        *i = 1;
                        continue 'main; // breaks big loop, because outside of while loop.
                    }
                }
            }
            edit_embed(ctx, message, "The dealer bust!", "The dealer bust!").await;
            sleep(quarter_second);
            player_win(ctx, message, msg, hand1.clone(), hand2.clone(), bet * 2).await;
            break;
        } else if sum2 == 21 {
            if sum1 == 21 {
                tie(ctx, message, hand1, hand2, &msg.author, bet).await;
                return Ok(());
            }
            edit_embed(
                ctx,
                message,
                "The dealer got a blackjack!",
                "The dealer got a blackjack!",
            )
            .await;
            sleep(quarter_second);
            dealer_win(ctx, message, msg, hand1.clone(), hand2.clone(), bet).await;
            break;
        }
        sleep(quarter_second);
        // clear();
        if !stay {
            // most things get here, i think.
            let new_title = format_game_status(None, hand1.clone(), hand2.clone(), false);
            edit_embed(ctx, message, new_title.as_str(), "description").await;
        }
        sleep(quarter_second); // necessary? sure.
    }
    Ok(())
}

// The game engine is done, now for the several, extremely overcomplicated functions that it calls.

// Format game status for display in embed.
fn format_game_status(
    starting_text: Option<&str>, // such as "You win!" "You lose."
    hand1: Vec<usize>,
    hand2: Vec<usize>,
    all_of_dealer: bool, // if it displays the dealer's entire hand or not.
) -> String {
    // generate sums
    let sum1: usize = hand1.iter().sum();
    let sum2: usize = hand2.iter().sum();

    // make mutable string, which gets pushed to throughout this labyrinth of control statements
    let mut output = String::new();

    // if there is starting text, then push it first. If not, then do nothing.
    match starting_text {
        Some(x) => output.push_str(x),
        None => {}
    }

    // Player's hand
    output.push_str("Your hand: \n");

    // push each card individually
    for card in hand1.iter() {
        let card_str = format!("{}  ", card);
        output.push_str(&*card_str);
    }

    // player's total, and two newlines to keep it separated
    let to_push = format!("\nTotal: {}.\n\n", sum1);
    output.push_str(&*to_push);

    // Dealer's hand
    if all_of_dealer {
        // if the player is allowed to see the entire hand of the dealer, format it in the same way that the player's hand was.
        output.push_str("Dealer's hand: \n");
        for card in hand2.iter() {
            let card_str = format!("{}  ", card);
            output.push_str(&*card_str);
        }
        let to_push = format!("\nTotal: {}.", sum2);
        output.push_str(&*to_push);
    } else {
        // If you are only allowed to see one of the dealer's cards, just format the first one.
        output.push_str("One of the dealer's cards: ");
        let dealer_card_str = format!("{}", hand2[1]);
        output.push_str(&*dealer_card_str);
    }

    // finally, return the jumbled mass of labels, numbers, and newlines as one big string.
    return output;
}

// Ending functions

// This function may not be necessary. It is only called once.
fn test_win(hand1: Vec<usize>, hand2: Vec<usize>) -> String {
    // true if player wins, false if computer wins
    let sum1: usize = hand1.iter().sum();
    let sum2: usize = hand2.iter().sum();
    if sum1 > sum2 {
        return "win".to_string();
    } else if sum1 < sum2 {
        return "lose".to_string();
    } else if sum1 == sum2 {
        return "tie".to_string();
    }
    "tie".to_string()
}

// Winning, losing, and tie functions
async fn player_win(
    ctx: &Context,
    message: &mut Message,
    msg: &Message,
    hand1: Vec<usize>,
    hand2: Vec<usize>,
    betx2: i32,
) {
    edit_embed(
        ctx,
        message,
        "You won!",
        &*format_game_status(None, hand1.clone(), hand2.clone(), true),
    )
    .await;
    money_increment_with_lost(&msg.author, msg.guild_id.unwrap().as_u64().clone(), betx2).unwrap();
    let response = format!("You won **{}** monies.", betx2 / 2);
    msg.reply(&ctx.http, response).await.unwrap();
}

async fn dealer_win(
    ctx: &Context,
    message: &mut Message,
    msg: &Message,
    hand1: Vec<usize>,
    hand2: Vec<usize>,
    bet: i32,
) {
    edit_embed(
        ctx,
        message,
        "You lose.",
        &*format_game_status(None, hand1.clone(), hand2.clone(), true),
    )
    .await;
    // money is already lost, so increment pool
    lost_increment(*msg.guild_id.unwrap().as_u64(), bet, &gen_connection()).unwrap();
    let response = format!("You lost **{}** monies.", bet);
    msg.reply(&ctx.http, response).await.unwrap();
}
async fn tie(ctx: &Context, message: &mut Message, hand1: Vec<usize>, hand2: Vec<usize>, user: &User, bet: i32) {
    money_increment_without_lost(&user, bet).unwrap();
    edit_embed(
        ctx,
        message,
        "Tie.",
        &*format_game_status(Some("Tie. \n"), hand1.clone(), hand2.clone(), true),
    )
    .await;
}

// generate deck
fn deck_gen() -> Vec<usize> {
    let mut rng = thread_rng();
    let mut deck: Vec<usize> = Vec::new();

    // four suits
    for _ in 0..4 {
        // 2, 3, 4, ... 11 per suit
        for i in 2..12 {
            deck.push(i);
        }

        // because face cards count as 10, push three more tens per suit.
        for _ in 0..2 {
            deck.push(10);
        }
    }

    deck.shuffle(&mut rng); // shuffle using rand crate
    return deck; // return the deck, which is of type Vec<usize>.
}
