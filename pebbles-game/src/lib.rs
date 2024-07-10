use gstd::{msg, exec, Encode, Decode};
use pebbles_game_io::*;

fn init() {
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

fn handle() {
    let action: PebblesAction = msg::load().expect("Unable to load action message");
    let mut state = GameState::load();

    // Check for valid input data
    match action {
        PebblesAction::Turn(pebbles) => {
            assert!(pebbles > 0 && pebbles <= state.max_pebbles_per_turn, "Invalid number of pebbles to remove");
            assert!(state.pebbles_remaining >= pebbles, "Not enough pebbles to remove");
        },
        _ => {},
    }

    // Process the User's turn and check whether they win
    if let PebblesAction::Turn(pebbles) = action {
        state.pebbles_remaining -= pebbles;
        state.winner = check_winner(&state);

        if let Some(winner) = state.winner {
            msg::reply(PebblesEvent::Won(winner), 0).expect("Unable to send message");
            state.save();
            return;
        }

        // Process the Program turn and check whether it wins
        let pebbles_to_remove = match state.difficulty {
            DifficultyLevel::Easy => (get_random_u32() % (state.max_pebbles_per_turn + 1)) + 1,
            DifficultyLevel::Hard => find_best_move(state.max_pebbles_per_turn, state.pebbles_remaining),
        };
        state.pebbles_remaining -= pebbles_to_remove;
        state.winner = check_winner(&state);

        msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0).expect("Unable to send message");
    } else if let PebblesAction::GiveUp = action {
        // Handle GiveUp action
        state.winner = Some(Player::Program);
        msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to send message");
    } else if let PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } = action {
        // Handle Restart action
        state = GameState {
            difficulty,
            pebbles_count,
            max_pebbles_per_turn,
            pebbles_remaining: pebbles_count,
            first_player: if get_random_u32() % 2 == 0 { Player::User } else { Player::Program },
            winner: None,
        };
    }

    state.save();
}

fn state() {
    let state = GameState::load();
    msg::reply(state, 0).expect("Unable to send message");
}

fn check_winner(state: &GameState) -> Option<Player> {
    if state.pebbles_remaining == 0 {
        Some(match state.first_player {
            Player::User => Player

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
