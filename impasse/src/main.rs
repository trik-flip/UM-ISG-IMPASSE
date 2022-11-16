// Made by Philip Bollen
extern crate rand;
mod games;
use game_engine::enhancements::{base::nega_with_table, transposition_table::TranspositionTable};
use game_engine::move_finders::{find_best_move_t_tt_id, human_agent};
use game_engine::traits::{ChildStates, ScoreOfState, StateHash, TerminalState};
use games::impasse::game::Impasse;

use std::env;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::time::Duration;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let set_time;
    let save_to_file;
    let starting;
    // !====================== Arg Parsing =====================================
    match args.len() {
        1 => {
            help();
            exit(1);
        }
        2 => {
            starting = match args[1].parse() {
                Ok(x) => x,
                Err(_) => true,
            };
            set_time = 200;
            save_to_file = true;
        }
        3 => {
            starting = match args[1].parse() {
                Ok(x) => x,
                Err(_) => true,
            };
            set_time = match args[2].parse() {
                Ok(x) => x,
                Err(_) => 200,
            };
            save_to_file = true;
        }
        4 => {
            starting = match args[1].parse() {
                Ok(x) => x,
                Err(_) => true,
            };
            set_time = match args[2].parse() {
                Ok(x) => x,
                Err(_) => 200,
            };
            save_to_file = match args[3].parse() {
                Ok(x) => x,
                Err(_) => true,
            };
        }
        _ => {
            starting = true;
            set_time = 200;
            save_to_file = true;
        }
    }
    // println!("Procces ID: {}\nStarting: ", id());

    let binding = Impasse::gen_hash_field(420);
    let game = Impasse::new(&binding);

    let mut current_move = game;
    let mut color = true;
    let mut time_of_color = 0;
    let mut time_of_not_color = 0;
    let mut number_of_moves: u16 = 0;
    let mut table = TranspositionTable::default();
    let mut game_history = Vec::new();
    // let mut seed = StdRng::seed_from_u64(42);
    let now = Instant::now();
    while !current_move.is_terminal() {
        {
            match color {
                true => println!("O is thinking"),
                false => println!("X is thinking"),
            }
            number_of_moves += 1;

            println!("{}", current_move);
        }
        let now = Instant::now();
        let new_move = match color {
            true => match starting {
                true => computer_agent(&current_move, set_time, color, &mut table),
                false => human_agent(&current_move, color),
            },
            false => match starting {
                true => human_agent(&current_move, color),
                false => computer_agent(&current_move, set_time, color, &mut table),
            },
        };
        let stop = now.elapsed();
        {
            game_history.push((new_move, color));
            current_move = current_move + new_move;
            println!("{}", new_move);
        }
        {
            if color {
                time_of_color += stop.as_millis()
            } else {
                time_of_not_color += stop.as_millis()
            }
        }
        color = !color;
        {
            if stop.as_millis() > 2000 {
                println!("time: {}s", stop.as_secs());
            } else if stop.as_micros() > 2000 {
                println!("time: {}ms", stop.as_millis());
            } else {
                println!("time: {}us", stop.as_micros());
            }
            println!();
        }
    }
    let stop = now.elapsed();

    match color {
        true => println!("X wins"),
        false => println!("O wins"),
    }
    {
        println!("stopped with {} turns", number_of_moves);
        println!("total time played: {}s", stop.as_secs());
        println!("'O' time: {}s", time_of_color / 1000);
        println!("'X' time: {}s", time_of_not_color / 1000);
    }
    if save_to_file {
        let mut file = File::create("Game.txt")?;
        for single_move in game_history {
            if single_move.1 {
                writeln!(file, "O: {}\n", single_move.0).unwrap();
            } else {
                writeln!(file, "X: {}\n", single_move.0).unwrap();
            }
        }
    }
    Ok(())
}

fn help() {
    println!("This is the help menu of the game");
    println!("=================================");
    println!("The arguments are structured as following");
    println!("impasse.exe <starting:bool><set_time:uint><save_game:bool>");
    println!("");
    println!("Types:");
    println!("\tbool:");
    println!("\t\ttrue \t-> passing a true value");
    println!("\t\tfalse \t-> passing a false value");
    println!("\tuint:");
    println!(
        "\t\tprovide a number between 1 and 2^64 \n\t\tThis is used for time in ms so be nice to yourself😉\n\t\tJust so you know, 100ms is about 7ply deep in mid game, 200ms is the default"
    );
    println!("");
    println!("Arguments:");
    println!("\tstarting -> determines if the computer is to start or not");
    println!("\t\ttrue -> computer starts");
    println!("\t\tfalse -> player starts");
    println!("");
    println!("\tset_time -> the time (in milli seconds) the computer is given to search for the best move");
    println!("");
    println!(
        "\tsave_game -> The program provides an option to save the game to a \"game.txt\" file"
    );
    println!("");
    println!("Default Values:");
    println!("\tstarting = true");
    println!("\tset_time = 200 milli seconds");
    println!("\tsave_game = true");
    println!("");
    println!("Examples:");
    println!("\t>impasse.exe lets play; # Use the default values");
    println!("\t>impasse.exe y; # use the default values");
    println!("\t>impasse.exe true; # the computer will start");
    println!(
        "\t>impasse.exe false 400; # the player will start, computer can think 400ms per turn"
    );
    println!("\t>impasse.exe true 1000 false; # the computer will start, computer can think 1s per turn, and the game will not be saved to a file.");
    println!("");
    println!("During the game:");
    println!("\tThe player will be provided with all possible moves ranging from 1 to x. Where x is the last move");
    println!(
        "\tIt's possible to select the move which you like to do by simply typing the index (which is provided along side the move)"
    );
    println!(
        "\tand hitting [Enter], this will execute the move and directly pass the turn to the computer."
    );
    println!("\n\n Happy playing 😊");
}

fn computer_agent<
    T: ScoreOfState + TerminalState + ChildStates<M> + Sync + Eq + Copy + StateHash,
    M: Copy + Send + Default + Ord,
>(
    current_move: &T,
    set_time: u64,
    color: bool,
    table: &mut TranspositionTable<M>,
) -> M {
    find_best_move_t_tt_id(
        current_move,
        Duration::from_millis(set_time),
        color,
        table,
        |state, depth, color, table| {
            -nega_with_table(state, depth, color, table, isize::MIN + 1, isize::MAX)
        },
    )
}
