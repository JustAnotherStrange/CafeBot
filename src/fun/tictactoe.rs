// TODO: TESTING!
use serenity::{
    framework::standard::{macros::command, CommandResult, Args},
    model::prelude::*,
    prelude::*,
};
// for random
use rand::{thread_rng, Rng};
// for time stuff
use std::{thread, time};
// for usize to i32 convert
use crate::fun::blackjack::edit_embed;
use crate::money::shop::get_amount_of_tickets;
use std::time::Duration;

// define tile (E for empty)
#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    X,
    O,
    E,
}

#[command]
async fn tictactoe(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let bet = match args.single::<u32>() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(&ctx.http, "Please [correct arguments]").await?;
            return Ok(());
        }
    };
    // TODO: negative/betting too much prevention
    let diff = match args.single::<u32>() {
        Ok(x) => x,
        Err(_) => {
            msg.reply(&ctx.http, "Please [correct arguments]").await?;
            return Ok(());
        }
    };
    let mut message = msg.reply(&ctx.http, "tic tac toe").await?;
    // initial embed
    message
        .edit(&ctx, |m| {
            m.embed(|e| {
                e.title("Loading...");
                e
            })
        })
        .await?;

    tictactoe_engine(&ctx, &mut message, &msg, bet, diff).await.unwrap();
    Ok(())
}

// tictactoe engine, which is actually just a modified version of https://github.com/justanotherstrange/tictactoe.rs
async fn tictactoe_engine(ctx: &Context, message: &mut Message, msg: &Message, bet: u32, diff: u32) -> Result<(), ()> {
    // let quarter_second = time::Duration::from_millis(250); // for sleeps later on
    let now = time::Instant::now();
    // gen board
    let mut board: [Tile; 9] = [Tile::E; 9];
    let difficulty_int: usize = diff as usize;

    // show board key for prompting for first move
    let board_key = format!("1 | 2 | 3\n----------\n4 | 5 | 6\n----------\n7 | 8 | 9");
    edit_embed(&ctx, message, board_key.as_str(), "What is your first move?").await;
    loop {
        let mut input_int: usize;
        'main: loop {
            // remove newline character and turn into an integer from string
            // match thing
            if let Some(reaction) = message
                .await_reaction(&ctx)
                .timeout(Duration::from_secs(60)) // after 60 seconds without reactions, it will go to the "else" statement.
                .await
            {
                let emoji = &reaction.as_inner_ref().emoji;
                let reacted = &*reaction.as_inner_ref().clone();
                if reacted.user(&ctx).await.unwrap() != msg.author {
                    continue 'main;
                }
                // match on the reacted emoji
                input_int = match emoji.as_data().as_str() {
                    "🎫" => 0,
                    "📈" => 1,
                    "🛑" => 2,
                    _ => continue 'main, // if the reaction is none of the above, then do nothing.
                };
            } else {
                // gets here if there were no reactions for 60 seconds.
                let new_description = "Two minutes passed with no reactions, so the shop closed.";
                edit_embed(ctx, message, "Timed out.", new_description).await;
                // todo: lose money on timeout like in blackjack
                return Ok(()); // close the game
            }
            if board[input_int] == Tile::E {
                break;
            } else {
                println!("someone has already gone there!");
                thread::sleep(time::Duration::from_secs(1));
                format_board(&mut board);
            }
        }
        board[input_int] = Tile::X; // todo: make this not use a function, and therefore, work/compile. it will have to take/await reactions. maybe keep it in a function?
        format_board(&mut board);
        let win = win_check(&mut board);
        if win == 1 {
            let response = format!("Difficulty was: {}, so you won WIP monies.\nTime: {} seconds", difficulty_int, now.elapsed().as_secs());
            edit_embed(&ctx, message, "You win!", response.as_str()).await;
            break;
        } else if win == 2 {
            let response = format!("Difficulty was: {}, but you didn't win any monies.\nTime: {} seconds", difficulty_int, now.elapsed().as_secs());
            edit_embed(&ctx, message, "Tie.", response.as_str()).await;
            break;
        }
        // computer turn
        // todo: thinking message
        // println!("thinking...");
        // wait one second
        thread::sleep(time::Duration::from_secs(1)); // todo: make this sleep less?
        let comp_turn = computer_turn(&mut board, difficulty_int as i32);
        if comp_turn.1 == true {
            board[comp_turn.0] = Tile::O;
        }
        let response = format_board(&mut board);
        edit_embed(&ctx, message, "playing", response.as_str()).await; // todo: better board printing. see how this looks on discord.
        let win = win_check(&mut board);
        if win == 1 {
            let response = format!("Difficulty was: {}, so you lost WIP monies.\nTime: {} seconds", difficulty_int, now.elapsed().as_secs());
            edit_embed(&ctx, message, "You lose!", response.as_str()).await;
            break;
        } else if win == 2 {
            let response = format!("Difficulty was: {}, but you didn't win any monies.\nTime: {} seconds", difficulty_int, now.elapsed().as_secs());
            edit_embed(&ctx, message, "Tie.", response.as_str()).await;
            break;
        }
    }
    Ok(())
}

fn print_tile(t: Tile) -> char {
    // formats the tile enum into a char
    return match t {
        Tile::X => 'X',
        Tile::O => 'O',
        _ => '-',
    };
}

fn format_board(board: &mut [Tile; 9]) -> String {
    // formats board with nice formatting and a key off to the side
    let mut to_return = String::new();
    let str = format!(
        "```{} | {} | {}\n",
        print_tile(board[0]),
        print_tile(board[1]),
        print_tile(board[2])
    );
    to_return.push_str(str.as_str());
    let str = format!(
        "{} | {} | {}\n",
        print_tile(board[3]),
        print_tile(board[4]),
        print_tile(board[5])
    );
    to_return.push_str(str.as_str());
    let str = format!(
        "{} | {} | {}```\n",
        print_tile(board[6]),
        print_tile(board[7]),
        print_tile(board[8])
    );
    to_return.push_str(str.as_str());
    to_return.push_str("\n`Key:\n1 | 2 | 3\n4 | 5 | 6\n7 | 8 | 9`");
    return to_return;
}

fn win_check(board: &mut [Tile; 9]) -> i32 {
    // hardcoded all possible winning positions
    // returns 0 if nothing, 1 if someone won, and 2 if its a tie
    if (board[0] == Tile::X && board[1] == Tile::X && board[2] == Tile::X)
        || (board[3] == Tile::X && board[4] == Tile::X && board[5] == Tile::X)
        || (board[6] == Tile::X && board[7] == Tile::X && board[8] == Tile::X)
        || (board[0] == Tile::X && board[3] == Tile::X && board[6] == Tile::X)
        || (board[1] == Tile::X && board[4] == Tile::X && board[7] == Tile::X)
        || (board[2] == Tile::X && board[5] == Tile::X && board[8] == Tile::X)
        || (board[0] == Tile::X && board[4] == Tile::X && board[8] == Tile::X)
        || (board[2] == Tile::X && board[4] == Tile::X && board[6] == Tile::X)
    {
        return 1;
    }

    if board[0] == Tile::O && board[1] == Tile::O && board[2] == Tile::O
        || (board[3] == Tile::O && board[4] == Tile::O && board[5] == Tile::O)
        || (board[6] == Tile::O && board[7] == Tile::O && board[8] == Tile::O)
        || (board[0] == Tile::O && board[3] == Tile::O && board[6] == Tile::O)
        || (board[1] == Tile::O && board[4] == Tile::O && board[7] == Tile::O)
        || (board[2] == Tile::O && board[5] == Tile::O && board[8] == Tile::O)
        || (board[0] == Tile::O && board[4] == Tile::O && board[8] == Tile::O)
        || (board[2] == Tile::O && board[4] == Tile::O && board[6] == Tile::O)
    {
        return 1;
    }

    // checks 0 thru 8 board pieces, and if one of them is empty, it breaks
    // therefore, if all of them are full, it'll return 2 for tie.
    for i in 0..9 {
        if board[i] == Tile::E {
            return 0;
        }
    }
    return 2;
}


fn go_two_os(board: &mut [Tile; 9]) -> (usize, bool) {
    if (board[1] == Tile::O && board[2] == Tile::O && board[0] == Tile::E)
        || (board[3] == Tile::O && board[6] == Tile::O && board[0] == Tile::E)
        || (board[4] == Tile::O && board[8] == Tile::O && board[0] == Tile::E)
    {
        return (0, true);
    }
    if (board[0] == Tile::O && board[2] == Tile::O && board[1] == Tile::E)
        || (board[7] == Tile::O && board[4] == Tile::O && board[1] == Tile::E)
    {
        return (1, true);
    }
    if (board[0] == Tile::O && board[1] == Tile::O && board[2] == Tile::E)
        || (board[8] == Tile::O && board[5] == Tile::O && board[2] == Tile::E)
        || (board[4] == Tile::O && board[6] == Tile::O && board[2] == Tile::E)
    {
        return (2, true);
    }
    if (board[0] == Tile::O && board[6] == Tile::O && board[3] == Tile::E)
        || (board[4] == Tile::O && board[5] == Tile::O && board[3] == Tile::E)
    {
        return (3, true);
    }
    if (board[0] == Tile::O && board[8] == Tile::O && board[4] == Tile::E)
        || (board[1] == Tile::O && board[7] == Tile::O && board[4] == Tile::E)
        || (board[2] == Tile::O && board[6] == Tile::O && board[4] == Tile::E)
        || (board[3] == Tile::O && board[5] == Tile::O && board[4] == Tile::E)
    {
        return (4, true);
    }
    if (board[3] == Tile::O && board[4] == Tile::O && board[5] == Tile::E)
        || (board[2] == Tile::O && board[8] == Tile::O && board[5] == Tile::E)
    {
        return (5, true);
    }
    if (board[7] == Tile::O && board[8] == Tile::O && board[6] == Tile::E)
        || (board[0] == Tile::O && board[3] == Tile::O && board[6] == Tile::E)
        || (board[2] == Tile::O && board[4] == Tile::O && board[6] == Tile::E)
    {
        return (6, true);
    }
    if (board[6] == Tile::O && board[8] == Tile::O && board[7] == Tile::E)
        || (board[1] == Tile::O && board[4] == Tile::O && board[7] == Tile::E)
    {
        return (7, true);
    }
    if (board[6] == Tile::O && board[7] == Tile::O && board[8] == Tile::E)
        || (board[2] == Tile::O && board[5] == Tile::O && board[8] == Tile::E)
        || (board[0] == Tile::O && board[4] == Tile::O && board[8] == Tile::E)
    {
        return (8, true);
    }
    return (0, false);
}

fn go_two_xs(board: &mut [Tile; 9]) -> (usize, bool) {
    if (board[1] == Tile::X && board[2] == Tile::X && board[0] == Tile::E)
        || (board[4] == Tile::X && board[8] == Tile::X && board[0] == Tile::E)
        || (board[3] == Tile::X && board[6] == Tile::X && board[0] == Tile::E)
    {
        return (0, true);
    }
    if (board[0] == Tile::X && board[2] == Tile::X && board[1] == Tile::E)
        || (board[7] == Tile::X && board[4] == Tile::X && board[1] == Tile::E)
    {
        return (1, true);
    }
    if (board[0] == Tile::X && board[1] == Tile::X && board[2] == Tile::E)
        || (board[8] == Tile::X && board[5] == Tile::X && board[2] == Tile::E)
        || (board[4] == Tile::X && board[6] == Tile::X && board[2] == Tile::E)
    {
        return (2, true);
    }
    if (board[4] == Tile::X && board[5] == Tile::X && board[3] == Tile::E)
        || (board[0] == Tile::X && board[6] == Tile::X && board[3] == Tile::E)
    {
        return (3, true);
    }
    if (board[3] == Tile::X && board[5] == Tile::X && board[4] == Tile::E)
        || (board[2] == Tile::X && board[6] == Tile::X && board[4] == Tile::E)
        || (board[0] == Tile::X && board[8] == Tile::X && board[4] == Tile::E)
        || (board[1] == Tile::X && board[7] == Tile::X && board[4] == Tile::E)
    {
        return (4, true);
    }
    if (board[2] == Tile::X && board[8] == Tile::X && board[5] == Tile::E)
        || (board[3] == Tile::X && board[4] == Tile::X && board[5] == Tile::E)
    {
        return (5, true);
    }
    if (board[7] == Tile::X && board[8] == Tile::X && board[6] == Tile::E)
        || (board[2] == Tile::X && board[4] == Tile::X && board[6] == Tile::E)
        || (board[0] == Tile::X && board[3] == Tile::X && board[6] == Tile::E)
    {
        return (6, true);
    }
    if (board[6] == Tile::X && board[8] == Tile::X && board[7] == Tile::E)
        || (board[1] == Tile::X && board[4] == Tile::X && board[7] == Tile::E)
    {
        return (7, true);
    }
    if (board[6] == Tile::X && board[7] == Tile::X && board[8] == Tile::E)
        || (board[2] == Tile::X && board[5] == Tile::X && board[8] == Tile::E)
        || (board[0] == Tile::X && board[4] == Tile::X && board[8] == Tile::E)
    {
        return (8, true);
    }
    return (0, false);
}

fn go_rand_norm_edge(board: &mut [Tile; 9]) -> (usize, bool) {
    let rand = thread_rng().gen_range(1..6);
    match rand {
        1 => {
            if board[1] == Tile::E {
                return (1, true);
            }
        }
        2 => {
            if board[3] == Tile::E {
                return (3, true);
            }
        }
        3 => {
            if board[4] == Tile::E {
                return (4, true);
            }
        }
        4 => {
            if board[5] == Tile::E {
                return (5, true);
            }
        }
        5 => {
            if board[7] == Tile::E {
                return (7, true);
            }
        }
        _ => unreachable!(),
    }
    return (0, false);
}

fn go_two_xs_nonconsecutive(board: &mut [Tile; 9]) -> (usize, bool) {
    for _ in 1..101 {
        if (board[0] == Tile::X && board[8] == Tile::X && board[4] == Tile::O)
            || (board[2] == Tile::X && board[6] == Tile::X && board[4] == Tile::O)
            || (board[0] == Tile::X && board[6] == Tile::X && board[4] == Tile::O)
            || (board[8] == Tile::X && board[7] == Tile::X && board[4] == Tile::O)
            || (board[2] == Tile::X && board[0] == Tile::X && board[4] == Tile::O)
            || (board[2] == Tile::X && board[8] == Tile::X && board[4] == Tile::O)
        {
            let norm_edge = go_rand_norm_edge(board);
            if norm_edge.1 == true {
                return (norm_edge.0, true);
            }
        }
    }
    return (0, false);
}

fn go_middle(board: &mut [Tile; 9]) -> (usize, bool) {
    if board[4] == Tile::E {
        return (4, true);
    }
    return (0, false);
}

fn go_randcorner_w_bordering_edge(board: &mut [Tile; 9]) -> (usize, bool) {
    for _ in 1..101 {
        let rand = thread_rng().gen_range(1..5);
        match rand {
            1 => {
                if board[0] == Tile::E && board[3] == Tile::X && board[1] == Tile::X {
                    return (0, true);
                }
            }
            2 => {
                if board[2] == Tile::E && board[1] == Tile::X && board[5] == Tile::X {
                    return (2, true);
                }
            }
            3 => {
                if board[6] == Tile::E && board[3] == Tile::X && board[7] == Tile::X {
                    return (6, true);
                }
            }
            4 => {
                if board[8] == Tile::E && board[5] == Tile::X && board[7] == Tile::X {
                    return (9, true);
                }
            }
            _ => unreachable!(),
        }
    }
    return (0, false);
}

fn go_rand_norm_corner(board: &mut [Tile; 9]) -> (usize, bool) {
    for _ in 1..101 {
        let rand = thread_rng().gen_range(1..5);
        match rand {
            1 => {
                if board[0] == Tile::E {
                    return (0, true);
                }
            }
            2 => {
                if board[2] == Tile::E {
                    return (2, true);
                }
            }
            3 => {
                if board[6] == Tile::E {
                    return (6, true);
                }
            }
            4 => {
                if board[8] == Tile::E {
                    return (8, true);
                }
            }
            _ => unreachable!(),
        }
    }
    return (0, false);
}

fn go_rand_corner_w_bordering_corner(board: &mut [Tile; 9]) -> (usize, bool) {
    for _ in 1..101 {
        let rand = thread_rng().gen_range(1..5);
        match rand {
            1 => {
                if (board[2] == Tile::X && board[0] == Tile::E)
                    || (board[6] == Tile::X && board[0] == Tile::E)
                {
                    return (0, true);
                }
            }
            2 => {
                if (board[0] == Tile::X && board[2] == Tile::E)
                    || (board[8] == Tile::X && board[2] == Tile::E)
                {
                    return (2, true);
                }
            }
            3 => {
                if (board[0] == Tile::X && board[6] == Tile::E)
                    || (board[8] == Tile::X && board[6] == Tile::E)
                {
                    return (6, true);
                }
            }
            4 => {
                if (board[2] == Tile::X && board[8] == Tile::E)
                    || (board[6] == Tile::X && board[8] == Tile::E)
                {
                    return (8, true);
                }
            }
            _ => println!("uh oh, numbers broke."),
        }
    }
    return (0, false);
}

fn go_complete_random(board: &mut [Tile; 9]) -> (usize, bool) {
    for _ in 1..101 {
        let rand = thread_rng().gen_range(0..9);
        match rand {
            0 => {
                if board[0] == Tile::E {
                    return (0, true);
                }
            }
            1 => {
                if board[1] == Tile::E {
                    return (1, true);
                }
            }
            2 => {
                if board[2] == Tile::E {
                    return (2, true);
                }
            }
            3 => {
                if board[3] == Tile::E {
                    return (3, true);
                }
            }
            4 => {
                if board[4] == Tile::E {
                    return (4, true);
                }
            }
            5 => {
                if board[5] == Tile::E {
                    return (5, true);
                }
            }
            6 => {
                if board[6] == Tile::E {
                    return (6, true);
                }
            }
            7 => {
                if board[7] == Tile::E {
                    return (7, true);
                }
            }
            8 => {
                if board[8] == Tile::E {
                    return (8, true);
                }
            }
            _ => unreachable!(),
        }
    }
    return (0, false);
}

fn computer_diff_gen(diffgen: i32) -> bool {
    return if diffgen == 100 {
        true
    } else {
        let rand = thread_rng().gen_range(0..101);
        if rand >= diffgen {
            false
        } else {
            true
        }
    }
}

fn computer_turn(board: &mut [Tile; 9], diffcomp: i32) -> (usize, bool) {
    // make it actually do like go_complete_random or the following using the pattern etc
    let compdiffgen = computer_diff_gen(diffcomp);
    if compdiffgen == true {
        let two_os = go_two_os(board);
        if two_os.1 == true {
            return (two_os.0, true);
        }
        let two_xs = go_two_xs(board);
        if two_xs.1 == true {
            return (two_xs.0, true);
        }
        let two_xs_nonconsecutive = go_two_xs_nonconsecutive(board);
        if two_xs_nonconsecutive.1 == true {
            return (two_xs_nonconsecutive.0, true);
        }
        let middle = go_middle(board);
        if middle.1 == true {
            return (middle.0, true);
        }
        let randcorner_w_bordering_edge = go_randcorner_w_bordering_edge(board);
        if randcorner_w_bordering_edge.1 == true {
            return (randcorner_w_bordering_edge.0, true);
        }
        let rand_norm_corner = go_rand_norm_corner(board);
        if rand_norm_corner.1 == true {
            return (rand_norm_corner.0, true);
        }
        let rand_norm_edge = go_rand_norm_edge(board);
        if rand_norm_edge.1 == true {
            return (rand_norm_edge.0, true);
        }
        let rand_corner_w_bordering_corner = go_rand_corner_w_bordering_corner(board);
        if rand_corner_w_bordering_corner.1 == true {
            return (rand_corner_w_bordering_corner.0, true);
        }
        let complete_random = go_complete_random(board);
        if complete_random.1 == true {
            return (complete_random.0, true);
        }
    } else {
        let complete_random = go_complete_random(board);
        if complete_random.1 == true {
            return (complete_random.0, true);
        }
    }
    return (0, false);
}