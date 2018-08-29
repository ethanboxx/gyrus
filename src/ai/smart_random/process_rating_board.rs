use ai::place_player;
extern crate rand;
use self::rand::Rng;
use ai::smart_random::RatingBoard;
use is_board_full;
use GameBoard;
use Players;
use IS_DEBUG;
pub fn process_rating_board(
    game_board: GameBoard,
    rating_board: RatingBoard,
    player_to_place: Players,
) -> GameBoard {
    loop {
        let random_tile = rand::thread_rng().gen_range(1, 10);
        if IS_DEBUG {
            println!("Trying2 {}", random_tile);
        };
        if highest_rating(rating_board.row_one[0], rating_board) && (random_tile == 1) {
            return GameBoard {
                row_one: [
                    place_player(game_board.row_one[0], player_to_place),
                    game_board.row_one[1],
                    game_board.row_one[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_one[1], rating_board) && (random_tile == 2) {
            return GameBoard {
                row_one: [
                    game_board.row_one[0],
                    place_player(game_board.row_one[1], player_to_place),
                    game_board.row_one[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_one[2], rating_board) && (random_tile == 3) {
            return GameBoard {
                row_one: [
                    game_board.row_one[0],
                    game_board.row_one[1],
                    place_player(game_board.row_one[2], player_to_place),
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_two[0], rating_board) && (random_tile == 4) {
            return GameBoard {
                row_two: [
                    place_player(game_board.row_two[0], player_to_place),
                    game_board.row_two[1],
                    game_board.row_two[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_two[1], rating_board) && (random_tile == 5) {
            return GameBoard {
                row_two: [
                    game_board.row_two[0],
                    place_player(game_board.row_two[1], player_to_place),
                    game_board.row_two[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_two[2], rating_board) && (random_tile == 6) {
            return GameBoard {
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1],
                    place_player(game_board.row_two[2], player_to_place),
                ],
                ..game_board
            };
        }

        if highest_rating(rating_board.row_three[0], rating_board) && (random_tile == 7) {
            return GameBoard {
                row_three: [
                    place_player(game_board.row_three[0], player_to_place),
                    game_board.row_three[1],
                    game_board.row_three[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_three[1], rating_board) && (random_tile == 8) {
            return GameBoard {
                row_three: [
                    game_board.row_three[0],
                    place_player(game_board.row_three[1], player_to_place),
                    game_board.row_three[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_three[2], rating_board) && (random_tile == 9) {
            return GameBoard {
                row_three: [
                    game_board.row_three[0],
                    game_board.row_three[1],
                    place_player(game_board.row_three[2], player_to_place),
                ],
                ..game_board
            };
        }
        if is_board_full(game_board) {
            println!("This should not be happening :(");
            panic!();
        } else {
            continue;
        };
    }
}

fn highest_rating(rating_being_tested: Option<f64>, rating_board: RatingBoard) -> bool {
    if IS_DEBUG {
        println!("rating{:#?}", rating_board);
    };
    if IS_DEBUG {
        println!("tested{:#?}", rating_being_tested);
    };

    match rating_being_tested {
        Option::None => false,
        Option::Some(rating_being_tested) => {
            (match rating_board.row_one[0] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_one[1] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_one[2] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_two[0] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_two[1] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_two[2] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_three[0] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_three[1] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_three[2] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            })
        }
    }
}
