use ai::process_ai;
use movement::process_movement;
mod ai;
mod draw;
mod movement;
pub mod tests;
use std::io;
const IS_DEBUG: bool = false;
#[derive(Copy, Clone, Debug)]
pub struct GameBoard {
    row_one: [TileStatus; 3],
    row_two: [TileStatus; 3],
    row_three: [TileStatus; 3],
}

#[derive(Copy, Clone, Debug)]
pub enum TileStatus {
    Nought(Cursor),
    Cross(Cursor),
    Cursor,
    None,
}

#[derive(Copy, Clone, Debug)]
pub enum Cursor {
    True,
    None,
}

#[derive(Copy, Clone)]
pub enum Players {
    Nought,
    Cross,
}

pub enum Winner {
    Nought,
    Cross,
    None,
}

pub enum GameMode {
    TwoPlayer,
    SinglePlayer,
    Spectate,
}

#[derive(Copy, Clone)]
pub enum AiMode {
    Random,
    SmartRandom,
    None,
}

pub struct MovementReturn {
    game_board: Option<GameBoard>,
    placed: bool,
}
fn main() {
    println!("Welcome to my noughts and crosses game made in rust.");
    let mut ai_mode = AiMode::None;
    let game_mode = game_mode_choice();
    match game_mode {
        GameMode::SinglePlayer | GameMode::Spectate => {
            ai_mode = ai_mode_choice();
        }
        _ => (),
    };
    print_instructions();
    let mut current_player = Players::Cross;
    let mut game_board = GameBoard {
        row_one: [TileStatus::Cursor, TileStatus::None, TileStatus::None],
        row_two: [TileStatus::None, TileStatus::None, TileStatus::None],
        row_three: [TileStatus::None, TileStatus::None, TileStatus::None],
    };

    println!("Crosses goes first.");
    println!("The board looks like this.");
    game_board.draw_game_board(&game_mode);
    loop {
        let movement_return = match game_mode {
            GameMode::Spectate => MovementReturn {
                game_board: Option::Some(game_board),
                placed: true,
            },
            _ => {
                let mut movement_return = process_movement(game_board, current_player);
                loop {
                    game_board = match movement_return.game_board {
                        Some(game_board) => game_board,
                        None => {
                            println!("That did not work");
                            movement_return = process_movement(game_board, current_player);
                            continue;
                        }
                    };
                    break;
                }
                movement_return
            }
        };

        if movement_return.placed {
            match game_board.has_someone_won() {
                Winner::Cross => {
                    match game_mode {
                        GameMode::SinglePlayer => {
                            println!("You won");
                        }
                        GameMode::TwoPlayer | GameMode::Spectate => {
                            println!("Crosses won");
                        }
                    }
                    game_board.draw_game_board(&game_mode);
                    break;
                }
                Winner::Nought => {
                    println!("Noughts won");
                    game_board.draw_game_board(&game_mode);
                    break;
                }
                Winner::None => {
                    if IS_DEBUG {
                        println!("No one has won");
                    };
                    if game_board.is_board_full() {
                        println!("It is a tie!");
                        game_board.draw_game_board(&game_mode);
                        break;
                    };
                }
            };

            match game_mode {
                GameMode::TwoPlayer => {
                    current_player = switch_player(current_player);
                }
                GameMode::SinglePlayer | GameMode::Spectate => {
                    current_player = match game_mode {
                        GameMode::SinglePlayer => switch_player(current_player),
                        _ => current_player,
                    };
                    game_board = process_ai(game_board, ai_mode, current_player);
                    current_player = switch_player(current_player);
                    match game_board.has_someone_won() {
                        Winner::Cross => {
                            println!("Crosses won");
                            game_board.draw_game_board(&game_mode);
                            break;
                        }
                        Winner::Nought => {
                            println!("Noughts won");
                            game_board.draw_game_board(&game_mode);
                            break;
                        }
                        Winner::None => {
                            if IS_DEBUG {
                                println!("No one has won");
                            };
                            if game_board.is_board_full() {
                                println!("It is a tie!");
                                game_board.draw_game_board(&game_mode);
                                break;
                            };
                        }
                    };
                }
            };
        };
        game_board.draw_game_board(&game_mode);
        continue;
    }
}

fn switch_player(current_player: Players) -> Players {
    match current_player {
        Players::Cross => {
            if IS_DEBUG {
                println!("Current player was switched to Nought");
            };
            Players::Nought
        }
        Players::Nought => {
            if IS_DEBUG {
                println!("Current player was switched to Cross");
            };
            Players::Cross
        }
    }
}

fn game_mode_choice() -> GameMode {
    println!("Input the number of players you want to play.");
    loop {
        let mut inputed_choice = String::new();
        io::stdin()
            .read_line(&mut inputed_choice)
            .expect("Failed to read line");
        let inputed_choice: u32 = match inputed_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("I did not understand that number. Plese Try again");
                continue;
            }
        };
        match inputed_choice {
            0 => {
                println!("Welcome to single spectate mode.");
                return GameMode::Spectate;
            }
            1 => {
                println!("Welcome to single player mode.");
                return GameMode::SinglePlayer;
            }
            2 => {
                println!("Welcome to two player mode.");
                return GameMode::TwoPlayer;
            }
            _ => {
                println!("This game only works with 1 or 2 players. Please try again.");
                continue;
            }
        }
    }
}

fn ai_mode_choice() -> AiMode {
    println!("Input the ai mode you want to play against. One or two?");
    loop {
        let mut inputed_choice = String::new();
        io::stdin()
            .read_line(&mut inputed_choice)
            .expect("Failed to read line");
        let inputed_choice: u32 = match inputed_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("I did not understand that number. Plese Try again");
                continue;
            }
        };
        match inputed_choice {
            1 => {
                println!("Welcome to Random mode.");
                return AiMode::Random;
            }
            2 => {
                println!("Welcome to SmartRandom mode.");
                return AiMode::SmartRandom;
            }
            _ => {
                println!("This game only works with ai 1 or 2");
                continue;
            }
        }
    }
}

fn print_instructions() {
    println!("To move the star left type 4 and hit enter");
    println!("To move the star right type 6 and hit enter");
    println!("To move the star up type 8 and hit enter");
    println!("To move the star down type 2 and hit enter");
    println!("To place your cross type 5 and hit enter");
}
