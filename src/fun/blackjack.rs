use crate::database::database::{get_money, money_increment};
use rand::{seq::SliceRandom, thread_rng};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
    Error,
};
use std::{thread::sleep, time, time::Duration};

#[command]
#[only_in(guilds)]
async fn blackjack(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
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
    let response = format!("**{}** bet **{}** monies on blackjack.", &msg.author.name, bet);
    let mut message = msg.reply(&ctx.http, &response).await?;
    message.edit(&ctx, |m| {
        m.embed(|e| {
            e.title("Loading...");
            e
        })
    }).await?;
    blackjack_engine(ctx, &mut message, msg, bet).await?;
    Ok(())
}

async fn edit_embed(ctx: &Context, message: &mut Message, title: &str, description: &str) {
    message.edit(&ctx, |m| {
        m.embed(|e| {
            e.title(&title);
            e.description(&description);
            e
        })
    })
    .await
    .unwrap();
}

// game
async fn blackjack_engine(
    ctx: &Context,
    message: &mut Message,
    msg: &Message,
    bet: i32,
) -> Result<(), Error> {
    let quarter_second = time::Duration::from_millis(250);
    let mut deck = deckgen();
    // deal
    let mut hand1: Vec<usize> = Vec::new();
    let mut hand2: Vec<usize> = Vec::new();
    hand1.push(deck.pop().unwrap());
    hand1.push(deck.pop().unwrap());
    hand2.push(deck.pop().unwrap());
    hand2.push(deck.pop().unwrap());
    // let mut sum1: usize = hand1.iter().sum();
    let mut sum1: usize;
    let mut sum2: usize = hand2.iter().sum();
    let mut stay = false;
    let new_title = format_game_status(None, hand1.clone(), hand2.clone(), false);
    edit_embed(ctx, message, new_title.as_str(), "Hit or stay? (React)").await;
    // reactions
    let letters: Vec<char> = vec!['✋', '🛑'];
    for letter in letters.iter() {
        message.react(ctx, *letter).await?;
    }
    // println!("Your hand: {:?} (Total {}), One of the dealer's cards: {:?}", hand1, sum1, hand2);
    loop {
        // player turn
        while !stay {
            // edit_embed(ctx, message, new_title.as_str(), "Hit or stay?").await;
            if let Some(reaction) = message
                .await_reaction(&ctx)
                .timeout(Duration::from_secs(60))
                .await
            {
                // By default, the collector will collect only added reactions
                // We could also pattern-match the reaction in case we want
                // to handle added or removed reactions.
                // In this case we will just get the inner reaction.
                // let reacts: Vec<User> = message.reaction_users(&ctx.http, ReactionType::Unicode, Some(50), '🇦').await?;
                let emoji = &reaction.as_inner_ref().emoji;
                // println!("{:?}", reacts);
                let _ = match emoji.as_data().as_str() {
                    "✋" => {
                        hand1.push(deck.pop().unwrap());
                        sum1 = hand1.iter().sum();
                        if sum1 > 21 {
                            if hand1.contains(&11) {
                                for i in hand1.iter_mut() {
                                    if i == &11 {
                                        *i = 1;
                                        break;
                                    }
                                }
                                // sum1 = hand1.iter().sum();
                            } else {
                                break;
                            }
                        } else if sum1 == 21 {
                            break;
                        }
                        let new_title =
                            format_game_status(None, hand1.clone(), hand2.clone(), false);
                        edit_embed(ctx, message, new_title.as_str(), "Hit or stay? (React)").await;
                    }
                    "🛑" => {
                        // stay
                        stay = true; // this breaks the while loop
                    }
                    _ => {}
                };
            } else {
                edit_embed(
                    ctx,
                    message,
                    "Timed out.",
                    "One minute passed with no reactions, so the game ended with no results.",
                )
                .await;
                return Ok(());
            }
        }
        // outside of while loop, stayed
        // Checks
        sleep(quarter_second); // necessary?
        sum1 = hand1.iter().sum();
        if sum1 > 21 {
            if hand1.contains(&11) {
                for i in hand1.iter_mut() {
                    if i == &11 {
                        *i = 1;
                        break; // breaks big loop
                    }
                }
            } else {
                edit_embed(ctx, message, "Bust!", "Bust").await;
                sleep(Duration::from_millis(500));
                dealer_win(ctx, message, msg, hand1.clone(), hand2.clone(), bet).await;
                break;
            }
        } else if sum1 == 21 {
            edit_embed(ctx, message, "Blackjack!", "Blackjack!").await;
            sleep(Duration::from_millis(500));
            player_win(ctx, message, msg, hand1.clone(), hand2.clone(), bet * 2).await;
            break;
        }
        // dealer turn
        // sum1 = hand1.iter().sum();
        if sum2 < 17 {
            hand2.push(deck.pop().unwrap());
            // sum2 = hand2.iter().sum();
            let new_title = format_game_status(None, hand1.clone(), hand2.clone(), true);
            edit_embed(ctx, message, new_title.as_str(), "Dealer's turn.").await;
        } else if sum2 >= 17 {
            edit_embed(ctx, message, "The dealer stays.", "The dealer stays.").await;
            sleep(quarter_second);
            if stay {
                // edit_embed(ctx, message, "You stayed... testing.", "test").await;
                match testwin(hand1.clone(), hand2.clone()) {
                    "win" => player_win(ctx, message, msg, hand1.clone(), hand2.clone(), bet).await,
                    "lose" => {
                        dealer_win(ctx, message, msg, hand1.clone(), hand2.clone(), bet).await
                    }
                    "tie" => tie(ctx, message, hand1, hand2).await,
                    _ => {}
                }
                break;
            }
        }
        sleep(quarter_second); // necessary? i think so.

        // Final checks
        // sum1 = hand1.iter().sum();
        sum2 = hand2.iter().sum();
        if sum2 > 21 {
            edit_embed(ctx, message, "The dealer bust!", "The dealer bust!").await;
            sleep(quarter_second);
            player_win(ctx, message, msg, hand1.clone(), hand2.clone(), bet).await;
            break;
        } else if sum2 == 21 {
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
        sleep(quarter_second); // necessary?
    }
    Ok(())
}

// format the decks
fn format_game_status(
    starting_text: Option<&str>,
    hand1: Vec<usize>,
    hand2: Vec<usize>,
    all_of_dealer: bool,
) -> String {
    let sum1: usize = hand1.iter().sum();
    let sum2: usize = hand2.iter().sum();
    let mut output = String::new();
    match starting_text {
        Some(x) => output.push_str(x),
        None => {}
    }
    output.push_str("Your hand: \n");
    // deck 1:
    for card in hand1.iter() {
        let card_str = format!("{}  ", card);
        output.push_str(&*card_str);
    }
    let to_push = format!("\nTotal: {}.\n\n", sum1);
    output.push_str(&*to_push);
    if all_of_dealer {
        output.push_str("Dealer's hand: \n");
        for card in hand2.iter() {
            let card_str = format!("{}  ", card);
            output.push_str(&*card_str);
        }
        let to_push = format!("\nTotal: {}.", sum2);
        output.push_str(&*to_push);
    } else {
        output.push_str("One of the dealer's cards: ");
        let dealer_card_str = format!("{}", hand2[1]);
        output.push_str(&*dealer_card_str);
    }
    return output;
}

// ending functions
fn testwin(hand1: Vec<usize>, hand2: Vec<usize>) -> &'static str {
    // true if player wins, false if computer wins
    let sum1: usize = hand1.iter().sum();
    let sum2: usize = hand2.iter().sum();
    if sum1 > sum2 {
        return "win";
    } else if sum1 < sum2 {
        return "lose";
    } else if sum1 == sum2 {
        return "tie";
    }
    "tie"
}

async fn player_win(
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
        "You won!",
        &*format_game_status(None, hand1.clone(), hand2.clone(), true),
    )
    .await;
    money_increment(&msg.author, bet).unwrap();
    let response = format!("You won **{}** monies.", bet);
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
    money_increment(&msg.author, -bet).unwrap();
    let response = format!("You lost **{}** monies.", bet);
    msg.reply(&ctx.http, response).await.unwrap();
}
async fn tie(ctx: &Context, message: &mut Message, hand1: Vec<usize>, hand2: Vec<usize>) {
    edit_embed(
        ctx,
        message,
        "Tie.",
        &*format_game_status(Some("Tie. \n"), hand1.clone(), hand2.clone(), true),
    )
    .await;
}

// fn clear() {
//     // clears the terminal
//     print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
// }

fn deckgen() -> Vec<usize> {
    let mut rng = thread_rng();
    let mut deck: Vec<usize> = Vec::new();
    for _ in 0..4 {
        for i in 2..12 {
            deck.push(i);
        }
        for _ in 0..2 {
            deck.push(10);
        }
    }
    deck.shuffle(&mut rng);
    // println!("{:?}", deck);
    return deck;
}
