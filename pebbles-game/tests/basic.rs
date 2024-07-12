use gtest::{Gtest, Message};
use pebbles_game_io::*;

#[test]
fn test_initialization() {
    let mut gtest = Gtest::new();
    let init = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };
    gtest.send_message(init);
    gtest.execute();
    gtest.assert_state(GameState {
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        pebbles_remaining: 10,
        difficulty: DifficultyLevel::Easy,
        first_player: Player::User,
        winner: None,
    });
}

#[test]
fn test_program_strategy_easy() {
    let mut gtest = Gtest::new();
    let init = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };
    gtest.send_message(init);
    gtest.execute();
    gtest.assert_state(GameState {
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        pebbles_remaining: 9,
        difficulty: DifficultyLevel::Easy,
        first_player: Player::Program,
        winner: None,
    });
}

#[test]
fn test_program_strategy_hard() {
    let mut gtest = Gtest::new();
    let init = PebblesInit {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };
    gtest.send_message(init);
    gtest.execute();
    gtest.assert_state(GameState {
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        pebbles_remaining: 8,
        difficulty: DifficultyLevel::Hard,
        first_player: Player::Program,
        winner: None,
    });
}