
// Imports
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use mirth_engine_tickers::*;
#[cfg(feature = "ticker_testing")]
use mirth_engine_testing_tools::{check_condition, TestColors, TestSet};

fn main() {
    App::new()

        .add_plugins(Tickers{})

        // Spawning tickers to mess with.
        .add_systems(Startup, setup_update_test_ticker)

        // Tests that occur on every frame.
        .add_systems(Update, (
            // test_update_ticker_advances_forward,
            // test_update_ticker_stays_within_range,
            // test_update_paused_ticker_never_advances,
            // test_update_looping_ticker_never_exceeds_end,
            // test_update_countdown_ticker_never_increases,
            // test_update_accrual_ticker_has_not_fired_early,
            test_update_print_info,
        ).chain())

        .run();
}



// ###################################### UPDATE TESTS ######################################## //
//
// Unlike the Startup tests above, these tests spawn a real Ticker<i32, f32> entity into
// the ECS so that ticker_ticking (registered by TimeStructures) actually queries and
// advances it every frame using real elapsed time.  This is the only way to verify that
// ticker_ticking itself is wired correctly — Startup tests call .tick() directly and
// never touch the ECS, so they can't catch a broken system registration or query.
//
// These tests run every frame indefinitely, re-checking the same invariants each time.
// This will produce a continuous stream of [PASS]/[FAIL] messages while the app runs —
// that's expected and is the point: a single [FAIL] appearing at any point means
// ticker_ticking violated an invariant on that frame.

/// Marker component distinguishing each Update-set test's dedicated ticker entity,
/// since multiple Ticker<i32, f32> entities now coexist in the world and each test
/// system needs to query only its own.
#[derive(Component)]
struct ForwardTestMarker;

#[derive(Component)]
struct PausedTestMarker;

#[derive(Component)]
struct LoopingTestMarker;

#[derive(Component)]
struct CountdownTestMarker;

#[derive(Component)]
struct AccrualTestMarker;

/// Spawns a single looping Ticker<i32, f32> entity for the per-frame tests to observe.
fn setup_update_test_ticker(mut commands: Commands) {

    let ticker: Ticker<i8, f32> = Ticker::new(
        0,
        0,
        100,
        0.1,
        false,
        false,
        true,
        false
    );
    commands.spawn((ticker, ForwardTestMarker));
}

/// Spawns a paused Ticker entity to confirm ticker_ticking leaves paused tickers untouched
/// across real frames, not just within a single .tick() call as the Startup test checks.
fn setup_update_test_paused_ticker(mut commands: Commands) {

    let ticker: Ticker<i32, f32> = Ticker::new(
        0,
        42,
        100,
        1.0,
        true,
        true,
        true,
        true
    );
    commands.spawn((ticker, PausedTestMarker));
}

/// Spawns a small-range looping Ticker entity so looping resets are observable
/// frequently in real time rather than requiring a very long-running app.
fn setup_update_test_looping_ticker(mut commands: Commands) {

    let ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        3,
        0.1,
        false,
        true,
        true,
        true
    );
    commands.spawn((ticker, LoopingTestMarker));
}

/// Spawns a ticking-down Ticker entity to confirm ticker_ticking respects tick
/// direction over real frames, not just a single hand-fed .tick() call.
fn setup_update_test_countdown_ticker(mut commands: Commands) {

    let ticker: Ticker<i32, f32> = Ticker::new(
        0,
        1_000_000,
        1_000_000,
        1.0,
        false,
        false,
        false,
        true
    );
    commands.spawn((ticker, CountdownTestMarker));
}

/// Spawns a Ticker with a very long interval to confirm accrued_delta accumulates
/// correctly across many frames without firing prematurely.
fn setup_update_test_accrual_ticker(mut commands: Commands) {

    let ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        100,
        9999.0,
        false,
        true,
        true,
        true
    );
    commands.spawn((ticker, AccrualTestMarker));
}

/// Verifies that the spawned Ticker's current_value always stays within its
/// start_value/end_value range, confirming ticker_ticking's clamping logic holds
/// up under real, continuously-accumulating frame deltas rather than the single
/// hand-fed deltas used in the Startup tests.
fn test_update_ticker_stays_within_range(
    tickers: Query<&Ticker<i32, f32>>,
) {
    for ticker in &tickers {
        let current = ticker.current_value();
        let start = ticker.start_value();
        let end = ticker.end_value();

        let min = start.min(end);
        let max = start.max(end);

        check_condition(
            current >= min && current <= max,
            "update::ticker_ticking keeps current_value within start_value/end_value range",
            "expected current_value to remain within range"
        );
    }
}

/// Verifies that a spawned, unpaused, ticking-up Ticker's current_value never decreases
/// from one frame to the next, confirming ticker_ticking is advancing it forward over time.
///
/// Tracks the previous frame's value in a Local<Option<i32>> since this needs to compare
/// across frames rather than within a single one.
fn test_update_ticker_advances_forward(
    tickers: Query<&Ticker<i32, f32>, With<ForwardTestMarker>>,
    mut previous_value: Local<Option<i32>>,
) {
    for ticker in &tickers {
        let current = ticker.current_value();

        if let Some(previous) = *previous_value {
            check_condition(
                current >= previous,
                "update::ticker_ticking never decreases current_value while ticking up",
                "expected current_value to be >= previous frame's value"
            );
        }

        *previous_value = Some(current);
    }
}

/// Verifies that a paused Ticker's current_value never changes across any frame,
/// confirming ticker_ticking's pause check holds under real, repeated frame advancement.
fn test_update_paused_ticker_never_advances(
    tickers: Query<&Ticker<i32, f32>, With<PausedTestMarker>>,
) {
    for ticker in &tickers {
        check_condition(
            ticker.current_value() == 42,
            "update::paused ticker never advances across real frames",
            "expected current_value to remain 42"
        );
    }
}

/// Verifies that a looping Ticker's current_value never exceeds end_value, confirming
/// that repeated real-frame ticks correctly trigger the loop-back-to-start_value reset
/// rather than current_value creeping past the boundary on some frame.
fn test_update_looping_ticker_never_exceeds_end(
    tickers: Query<&Ticker<i32, f32>, With<LoopingTestMarker>>,
) {
    for ticker in &tickers {
        check_condition(
            ticker.current_value() <= ticker.end_value(),
            "update::looping ticker never exceeds end_value across real frames",
            "expected current_value <= end_value"
        );

        check_condition(
            ticker.current_value() >= ticker.start_value(),
            "update::looping ticker never drops below start_value across real frames",
            "expected current_value >= start_value"
        );
    }
}

/// Verifies that a ticking-down Ticker's current_value never increases from one frame
/// to the next, mirroring test_update_ticker_advances_forward but for the opposite
/// tick direction.
fn test_update_countdown_ticker_never_increases(
    tickers: Query<&Ticker<i32, f32>, With<CountdownTestMarker>>,
    mut previous_value: Local<Option<i32>>,
) {
    for ticker in &tickers {
        let current = ticker.current_value();

        if let Some(previous) = *previous_value {
            check_condition(
                current <= previous,
                "update::ticking-down ticker never increases current_value across real frames",
                "expected current_value to be <= previous frame's value"
            );
        }

        *previous_value = Some(current);
    }
}

/// Verifies that a Ticker with an extremely long interval has not yet advanced its
/// current_value away from its starting point, confirming accrued_delta is being
/// tracked correctly rather than firing ticks prematurely due to a unit or
/// precision mistake somewhere in the accumulation logic.
fn test_update_accrual_ticker_has_not_fired_early(
    tickers: Query<&Ticker<i32, f32>, With<AccrualTestMarker>>,
) {
    for ticker in &tickers {
        check_condition(
            ticker.current_value() == 0,
            "update::ticker with a very long interval has not fired a tick yet",
            "expected current_value to remain 0 until the interval elapses"
        );
    }
}

/// Prints the dedicated forward-test Ticker's current_value every frame.
/// Purely observational — no assertions, just visibility into what the value
/// is actually doing frame to frame while debugging.
fn test_update_print_info(
    tickers: Query<&Ticker<i8, f32>, With<ForwardTestMarker>>,
) {
    for ticker in &tickers {

        println!("START_VALUE: {}", ticker.start_value());
        println!("CURRENT_VALUE: {}", ticker.current_value());
        println!("END_VALUE: {}", ticker.end_value());
        println!("INTERVAL: {}", ticker.interval());
        println!("ACCRUED_DELTA: {}\n", ticker.accrued_delta());
    }
}

// ############################################################################################## //
