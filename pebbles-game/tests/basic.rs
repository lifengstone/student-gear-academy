//! Check whether the game initialized correctly.
//! Check all program strategies (you may split the get_random_u32() function into two separated implementations for #[cfg(test)] and #[cfg(not(test))] environments).
//! Check negative scenarios and invalid input data processing.

// use pebbles_game_io::*;
// use gtest::{Log, Program, System};

// const PROGRAM_ID: u64 = 1;

// #[test]
// fn test_initialization() {
//     let system = System::new();
//     system.init_logger();

//     let program = Program::current(&system);

//     // Correct initialization
//     let init_msg = PebblesInit {
//         difficulty: DifficultyLevel::Easy,
//         pebbles_count: 10,
//         max_pebbles_per_turn: 3,
//     };

//     let init_result = program.send(PROGRAM_ID, init_msg);
//     assert!(!init_result.main_failed());

//     // Check if the state is initialized correctly
//     let state: GameState = program.read_state(()).expect("Failed to read state");
//     assert_eq!(state.pebbles_count, 10);
//     assert_eq!(state.max_pebbles_per_turn, 3);
//     assert_eq!(state.pebbles_remaining, 10);
//     assert_eq!(state.difficulty, DifficultyLevel::Easy);
//     assert!(state.winner.is_none());

//     // Invalid initialization: pebbles_count <= 0
//     let init_msg = PebblesInit {
//         difficulty: DifficultyLevel::Easy,
//         pebbles_count: 0,
//         max_pebbles_per_turn: 3,
//     };
//     let init_result = program.send(PROGRAM_ID, init_msg);
//     assert!(init_result.main_failed());

//     // Invalid initialization: max_pebbles_per_turn <= 0
//     let init_msg = PebblesInit {
//         difficulty: DifficultyLevel::Easy,
//         pebbles_count: 10,
//         max_pebbles_per_turn: 0,
//     };
//     let init_result = program.send(PROGRAM_ID, init_msg);
//     assert!(init_result.main_failed());

//     // Invalid initialization: max_pebbles_per_turn > pebbles_count
//     let init_msg = PebblesInit {
//         difficulty: DifficultyLevel::Easy,
//         pebbles_count: 10,
//         max_pebbles_per_turn: 11,
//     };
//     let init_result = program.send(PROGRAM_ID, init_msg);
//     assert!(init_result.main_failed());
// }

// #[test]
// fn test_program_strategies() {
//     let system = System::new();
//     system.init_logger();

//     let program = Program::current(&system);

//     let init_msg = PebblesInit {
//         difficulty: DifficultyLevel::Easy,
//         pebbles_count: 10,
//         max_pebbles_per_turn: 3,
//     };
//     program.send(PROGRAM_ID, init_msg);

//     // Check if the program makes a valid move on its turn
//     let action = PebblesAction::Turn(2);
//     let handle_result = program.send(PROGRAM_ID, action);
//     let log = handle_result.log();
//     assert!(log.contains(&Log::event(PebblesEvent::CounterTurn(_))));
// }

// #[test]
// fn test_negative_scenarios() {
//     let system = System::new();
//     system.init_logger();

//     let program = Program::current(&system);

//     let init_msg = PebblesInit {
//         difficulty: DifficultyLevel::Easy,
//         pebbles_count: 10,
//         max_pebbles_per_turn: 3,
//     };
//     program.send(PROGRAM_ID, init_msg);

//     // Negative scenario: User takes more pebbles than allowed
//     let action = PebblesAction::Turn(4);
//     let handle_result = program.send(PROGRAM_ID, action);
//     let log = handle_result.log();
//     assert!(log.contains(&Log::event(PebblesEvent::InvalidMove)));

//     // Negative scenario: User takes more pebbles than remaining
//     let action = PebblesAction::Turn(11);
//     let handle_result = program.send(PROGRAM_ID, action);
//     let log = handle_result.log();
//     assert!(log.contains(&Log::event(PebblesEvent::InvalidMove)));

//     // Negative scenario: User gives up
//     let action = PebblesAction::GiveUp;
//     let handle_result = program.send(PROGRAM_ID, action);
//     let log = handle_result.log();
//     assert!(log.contains(&Log::event(PebblesEvent::Won(Player::Program))));

//     // Negative scenario: User tries to play after giving up
//     let action = PebblesAction::Turn(1);
//     let handle_result = program.send(PROGRAM_ID, action);
//     let log = handle_result.log();
//     // Since the game is already over after giving up, any further moves should be considered invalid.
//     assert!(log.contains(&Log::event(PebblesEvent::InvalidMove)));
// }