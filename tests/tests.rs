use gamesession_io::*;
use gtest::{Log, ProgramBuilder, System};

const GAME_SESSION_PROGRAM_ID: u64 = 1;
const WORDLE_PROGRAM_ID: u64 = 2;
// USER is my student number
const USER: u64 = 50;

#[test]
fn test_win() {
    let system = System::new();
    system.init_logger();

    let game_session_program =
        ProgramBuilder::from_file("./target/wasm32-unknown-unknown/game_session.opt.wasm")
            .with_id(GAME_SESSION_PROGRAM_ID)
            .build(&system);
    let wordle_program =
        ProgramBuilder::from_file("./target/wasm32-unknown-unknown/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    // Case 1: wordle_program init
    let res = wordle_program.send_bytes(USER, []);
    assert!(!res.main_failed());

    // Case 2: game_session_program init
    let res = game_session_program.send(
        USER,
        GameSessionInit {
            wordle_program_id: WORDLE_PROGRAM_ID.into(),
        },
    );
    assert!(!res.main_failed());

    // Case 3: CheckWord - failed: The user is not in the game
    let res = game_session_program.send(
        USER,
        GameSessionAction::CheckWord {
            word: "abcde".to_string(),
        },
    );
    assert!(res.main_failed());

    // Case 4: StartGame - success
    let res = game_session_program.send(USER, GameSessionAction::StartGame);
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(GameSessionEvent::StartSuccess);
    assert!(!res.main_failed() && res.contains(&log));

    // Case 5: StartGame failed: The user is aleady in the game
    let res = game_session_program.send(USER, GameSessionAction::StartGame);
    assert!(res.main_failed());

    // Case 6: CheckWord failed: Invalid word
    let res = game_session_program.send(
        USER,
        GameSessionAction::CheckWord {
            word: "qwert".to_string(),
        },
    );
    assert!(res.main_failed());

    // Case 7: CheckWord failed: Invalid word
    let res = game_session_program.send(
        USER,
        GameSessionAction::CheckWord {
            word: "shell".to_string(),
        },
    );
    assert!(res.main_failed());

    // Case 8: CheckWord success, but failed to guess
    let res = game_session_program.send(
        USER,
        GameSessionAction::CheckWord {
            word: "house".to_string(),
        },
    );
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(GameSessionEvent::CheckWordResult {
            correct_positions: vec![0, 1, 3, 4],
            contained_in_word: vec![],
        });
    assert!(!res.main_failed() && res.contains(&log));

    // Case 9: CheckWord success and has been guessed
    let res = game_session_program.send(
        USER,
        GameSessionAction::CheckWord {
            word: "human".to_string(),
        },
    );
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(GameSessionEvent::GameOver(GameStatus::Win));
    assert!(!res.main_failed() && res.contains(&log));

    // Case 10: CheckWord failed: The user is not in the game
    let res = game_session_program.send(
        51,
        GameSessionAction::CheckWord {
            word: "tests".to_string(),
        },
    );
    assert!(res.main_failed());

    let state: GameSessionState = game_session_program.read_state(b"").unwrap();
    println!("{:?}", state);
}

#[test]
fn test_tried_limit() {
    let system = System::new();
    system.init_logger();

    let game_session_program =
        ProgramBuilder::from_file("./target/wasm32-unknown-unknown/game_session.opt.wasm")
            .with_id(GAME_SESSION_PROGRAM_ID)
            .build(&system);
    let wordle_program =
        ProgramBuilder::from_file("./target/wasm32-unknown-unknown/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    // Case 1: wordle_program init
    let res = wordle_program.send_bytes(USER, []);
    assert!(!res.main_failed());

    // Case 2: game_session_program init
    let res = game_session_program.send(
        USER,
        GameSessionInit {
            wordle_program_id: WORDLE_PROGRAM_ID.into(),
        },
    );
    assert!(!res.main_failed());

    // Case 3: StartGame success
    let res = game_session_program.send(USER, GameSessionAction::StartGame);
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(GameSessionEvent::StartSuccess);
    assert!(!res.main_failed() && res.contains(&log));

    for i in 0..5 {
        // Case 4: CheckWord success, but not guessed
        let res = game_session_program.send(
            USER,
            GameSessionAction::CheckWord {
                word: "house".to_string(),
            },
        );
        if i == 4 {
            let log = Log::builder()
                .dest(USER)
                .source(GAME_SESSION_PROGRAM_ID)
                .payload(GameSessionEvent::GameOver(GameStatus::Lose));
            assert!(!res.main_failed() && res.contains(&log));
        } else {
            let log = Log::builder()
                .dest(USER)
                .source(GAME_SESSION_PROGRAM_ID)
                .payload(GameSessionEvent::CheckWordResult {
                    correct_positions: vec![0, 1, 3, 4],
                    contained_in_word: vec![],
                });
            assert!(!res.main_failed() && res.contains(&log));
        }
    }
    let state: GameSessionState = game_session_program.read_state(b"").unwrap();
    println!("{:?}", state);
}

#[test]
#[ignore]
fn test_dealyed_logic() {
    let system = System::new();
    system.init_logger();

    let game_session_program =
        ProgramBuilder::from_file("./target/wasm32-unknown-unknown/game_session.opt.wasm")
            .with_id(GAME_SESSION_PROGRAM_ID)
            .build(&system);
    let wordle_program =
        ProgramBuilder::from_file("./target/wasm32-unknown-unknown/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    // Case 1: wordle_program init
    let res = wordle_program.send_bytes(USER, []);
    assert!(!res.main_failed());

    // Case 2: game_session_program init
    let res = game_session_program.send(
        USER,
        GameSessionInit {
            wordle_program_id: WORDLE_PROGRAM_ID.into(),
        },
    );
    assert!(!res.main_failed());

    // Case 3: StartGame success
    let res = game_session_program.send(USER, GameSessionAction::StartGame);
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(GameSessionEvent::StartSuccess);
    assert!(!res.main_failed() && res.contains(&log));
    
    // Case 4: Delayed equal to 200 blocks (10 minutes) for the delayed message
    let result = system.spend_blocks(200);
    println!("{:?}", result);
    let log = Log::builder()
        .dest(USER)
        .source(GAME_SESSION_PROGRAM_ID)
        .payload(GameSessionEvent::GameOver(GameStatus::Lose));
    assert!(result[0].contains(&log));
    let state: GameSessionState = game_session_program.read_state(b"").unwrap();
    println!("{:?}", state);
}