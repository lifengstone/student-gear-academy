#![no_std]

use gstd::{msg, exec};
use pebbles_game_io::*;

const STATE: &[u8] = b"state";

#[no_mangle]
extern fn init() {
    let init: PebblesInit = msg::load().expect("Unable to load init message");

    // Check for valid input data
    assert!(init.pebbles_count > 0, "Pebbles count must be greater than 0");
    assert!(init.max_pebbles_per_turn > 0, "Max pebbles per turn must be greater than 0");
    assert!(init.max_pebbles_per_turn <= init.pebbles_count, "Max pebbles per turn must not exceed total pebbles count");

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
    state.save();
}

#[no_mangle]
extern fn handle() {
    let action: PebblesAction = msg::load().expect("Unable to load action message");
    let mut state = GameState::load();

    // The game is over, no further actions can be processed
    if state.winner.is_some() {
        return;
    }

    match action {
        PebblesAction::Turn(pebbles) => {
            // Check for valid input data
            assert!(pebbles > 0 && pebbles <= state.max_pebbles_per_turn, "Invalid number of pebbles to remove");
            assert!(state.pebbles_remaining >= pebbles, "Not enough pebbles to remove");

            // Process the User's turn
            state.pebbles_remaining -= pebbles;
            state.winner = check_winner(&state);

            // If the game is over, notify the winner
            if let Some(winner) = state.winner {
                msg::reply(PebblesEvent::Won(winner), 0).expect("Unable to send message");
                state.save();
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

    state.save();
}

#[no_mangle]
extern fn state() {
    let state = unsafe { STATE.take().expect("State isn't initialized") };
    msg::reply(GameState::from_iter(state), 0).expect("Failed to reply from `state()`");
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

