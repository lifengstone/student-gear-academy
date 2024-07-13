//! Check whether the game initialized correctly.
//! Check all program strategies (you may split the get_random_u32() function into two separated implementations for #[cfg(test)] and #[cfg(not(test))] environments).
//! Check negative scenarios and invalid input data processing.

use gtest::{Log, Program, System};
use pebbles_game_io::*;

/// 测试游戏初始化是否正确
#[test]
fn test_game_initialization() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };

    let encoded_init_msg = init_msg.encode();

    let init_result = program.send_bytes(42, encoded_init_msg);
    assert!(!init_result.main_failed(), "初始化失败");

    let state = program.state::<GameState>();
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 3);
    assert!(state.pebbles_remaining <= 10);
    assert!(matches!(state.first_player, Player::User | Player::Program));
    assert!(state.winner.is_none());
}

/// 测试所有程序策略
#[test]
fn test_program_strategies() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };

    let encoded_init_msg = init_msg.encode();
    program.send_bytes(42, encoded_init_msg);

    let mut state = program.state::<GameState>();

    // 易难度下，测试程序随机移除石子
    if state.first_player == Player::Program {
        assert!(state.pebbles_remaining < 10);
    }

    // 重置状态并测试困难模式下的最优策略
    program.send_bytes(42, encoded_init_msg);
    state = program.state::<GameState>();
    state.difficulty = DifficultyLevel::Hard;
    program.send_bytes(42, state.encode());

    state = program.state::<GameState>();
    if state.first_player == Player::Program {
        let best_move = find_best_move(state.max_pebbles_per_turn, state.pebbles_remaining);
        assert_eq!(state.pebbles_remaining, 10 - best_move);
    }
}

/// 测试负场景和无效输入数据处理
#[test]
fn test_negative_scenarios_and_invalid_data() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    let invalid_init_msgs = vec![
        PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 0,
            max_pebbles_per_turn: 3,
        },
        PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 10,
            max_pebbles_per_turn: 0,
        },
        PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 5,
            max_pebbles_per_turn: 6,
        },
    ];

    for invalid_init in invalid_init_msgs {
        let encoded_init = invalid_init.encode();
        let result = program.send_bytes(42, encoded_init);
        assert!(result.main_failed(), "错误的初始化参数应导致失败");
    }
}