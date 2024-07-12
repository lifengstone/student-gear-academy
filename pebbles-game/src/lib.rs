//! The is the program to implement the Pebbles Game. The game's rules are the following:
//! - There are two players: User and Program. The first player is chosen randomly.
//! - The game starts with 𝑁 pebbles (e.g., 𝑁=15).
//! - On the player's turn they must remove from 1 to 𝐾 pebbles (e.g., if 𝐾=2, then the player removes 1 or 2 pebbles per turn).
//! - The player who takes last pebble(s) is the winner.

#![no_std]

use gstd::{exec, msg, prelude::*};
use pebbles_game_io::*;

static mut GAME_STATE: Option<GameState> = None;

/// Receives PebblesInit using the msg::load function;
/// Checks input data for validness;
/// Chooses the first player using the exec::random function;
/// Processes the first turn if the first player is Program.
/// Fills the GameState structure.
#[no_mangle]
extern fn init() {
    let init: PebblesInit = msg::load().expect("Unable to load init message");

    // Check for valid input data
    if init.pebbles_count <= 0 || init.max_pebbles_per_turn <= 0 || init.max_pebbles_per_turn > init.pebbles_count {
        panic!("Invalid init parameters");
    }

    // Choose the first player
    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    // Initialize the game state
    let mut state = GameState {
        pebbles_count: init.pebbles_count,
        max_pebbles_per_turn: init.max_pebbles_per_turn,
        pebbles_remaining: init.pebbles_count,
        difficulty: init.difficulty,
        first_player: first_player.clone(),
        winner: None,
    };

    // Process the first turn if the first player is Program
    if first_player == Player::Program {
        let pebbles_to_remove = match state.difficulty {
            DifficultyLevel::Easy => (get_random_u32() % (state.max_pebbles_per_turn + 1)) + 1,
            DifficultyLevel::Hard => find_best_move(state.max_pebbles_per_turn, state.pebbles_remaining),
        };
        state.pebbles_remaining -= pebbles_to_remove;
        state.winner = check_winner(&state);
    }

    // Save the state
    unsafe { GAME_STATE = Some(state) };
}

/// Receives PebblesAction using msg::load function;
/// Checks input data for validness;
/// Processes the User's turn and check whether they win;
/// Processes the Program turn and check whether it wins;
/// Send a message to the user with the correspondent PebblesEvent;
#[no_mangle]
extern fn handle() {
    let action: PebblesAction = msg::load().expect("Unable to load action message");

    unsafe {
        let mut state = GAME_STATE.take().expect("GameState isn't initialized");

        // The game is over, no further actions can be processed
        if state.winner.is_some() {
            GAME_STATE = Some(state);
            return;
        }

        match action {
            PebblesAction::Turn(pebbles) => {
                // Check for valid input data
                if pebbles <= 0 || pebbles > state.max_pebbles_per_turn || pebbles > state.pebbles_remaining {
                    msg::reply(PebblesEvent::InvalidMove, 0).expect("Unable to send message");
                    GAME_STATE = Some(state);
                    return;
                }

                // Process the User's turn
                state.pebbles_remaining -= pebbles;
                state.winner = check_winner(&state);

                // If the game is over, notify the winner
                if let Some(ref winner) = state.winner {
                    msg::reply(PebblesEvent::Won(winner.clone()), 0).expect("Unable to send message");
                    GAME_STATE = Some(state);
                    return;
                }

                // Process the Program's turn
                let pebbles_to_remove = match state.difficulty {
                    DifficultyLevel::Easy => (get_random_u32() % (state.max_pebbles_per_turn + 1)) + 1,
                    DifficultyLevel::Hard => find_best_move(state.max_pebbles_per_turn, state.pebbles_remaining),
                };
                state.pebbles_remaining -= pebbles_to_remove;
                state.winner = check_winner(&state);

                // Notify the user of the Program's turn
                msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0).expect("Unable to send message");
            },
            PebblesAction::GiveUp => {
                // Handle GiveUp action
                state.winner = Some(Player::Program);
                msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to send message");
            },
            PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
                // Handle Restart action
                state = GameState {
                    difficulty,
                    pebbles_count,
                    max_pebbles_per_turn,
                    pebbles_remaining: pebbles_count,
                    first_player: if get_random_u32() % 2 == 0 { Player::User } else { Player::Program },
                    winner: None,
                };
            },
        }

        GAME_STATE = Some(state);
    }
}

/// Returns the GameState structure using the msg::reply function
#[no_mangle]
extern fn state() {
    // let state = unsafe { GAME_STATE.take().expect("GameState isn't initialized") };
    msg::reply(unsafe { GAME_STATE.clone() }, 0).expect("Failed to share state");
}

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

fn check_winner(state: &GameState) -> Option<Player> {
    if state.pebbles_remaining == 0 {
        Some(match state.first_player {
            Player::User => Player::Program,
            Player::Program => Player::User,
        })
    } else {
        None
    }
}

fn find_best_move(max_pebbles_per_turn: u32, pebbles_remaining: u32) -> u32 {
    // For the "Hard" difficulty, the program should make the optimal move.
    // If the remaining pebbles divided by (max_pebbles_per_turn + 1) have a remainder,
    // remove enough pebbles to make the remainder 0. Otherwise, remove the maximum allowed.
    if pebbles_remaining % (max_pebbles_per_turn + 1) > 0 {
        pebbles_remaining % (max_pebbles_per_turn + 1)
    } else {
        max_pebbles_per_turn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gtest::{Log, Program, System};

    const PROGRAM_ID: u64 = 1;

    #[test]
    fn test_initialization() {
        // create test environment
        let system = System::new();
        system.init_logger();

        // create program instance
        let program = Program::current(&system);
        assert_eq!(program.id(), 1.into());

        // initialize program
        let init_msg = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
        };
        let encoded_init_msg = init_msg.encode();

        // let init_result = program.send_bytes(PROGRAM_ID, encoded_init_msg);
        // assert!(!init_result.main_failed());
    }
}