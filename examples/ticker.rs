
use bevy::prelude::*;
use mirth_engine_testing_tools::{check, TestSet};
use bevy_time_structures::{Ticker, TickerStates, TimeStructures};

// Tests for Ticker
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TimeStructures {})

        // One and done tests that are NOT intended to panic.
        .add_systems(Startup, (
            test_default,
            test_new,
            test_new_with_duration,
            test_new_with_countdown,
            test_getters,
            test_set_current_value,
            test_set_start_value,
            test_add_to_start,
            test_add_to_current,
            test_reset,
            test_set_to_zero,
            test_set_current_to_min,
            test_set_current_to_max,
            test_comparisons,
            test_get_distance_from_start,
            test_get_countdown_value,
            test_pause_unpause,
            test_ticker_states,
            test_tick_loop,
        ).chain().in_set(TestSet::Set0))

        // One and done tests that are intended to panic.
        .add_systems(Startup, (
            test_new_panic_guard,
            test_countdown_panic_guard,
        ).chain().in_set(TestSet::Set1))

        .configure_sets(Startup, TestSet::Set0.before(TestSet::Set1))

        .run();
}

// ─── Safety Note ─────────────────────────────────────────────────────────────
//
// ticker_ticking runs every frame and queries for Ticker components attached to
// entities via Query<&mut Ticker>.  Every test below constructs Tickers as plain
// local stack values — they are never spawned into the ECS — so ticker_ticking
// will never see or mutate them.  Tests that need to verify tick() behaviour call
// it directly with a known std::time::Duration instead of relying on real elapsed
// time, keeping results fully deterministic regardless of frame timing.
//
// ─────────────────────────────────────────────────────────────────────────────

/// Verifies that Ticker::default() produces a zero-state Ticker.
///
/// All numeric fields should be 0 and the state should be Ticking, confirming
/// that the default constructor is a clean, ready-to-use baseline with no
/// residual values.
fn test_default() {
    let t = Ticker::default();
    check("default::current_value is 0",t.get_current_value() == 0,                     "expected 0");
    check("default::start_value is 0",  t.get_start_value()   == 0,                     "expected 0");
    check("default::digit is 0",        t.get_digit()         == 0,                     "expected 0");
    check("default::state is Ticking",  t.get_state()         == &TickerStates::Ticking,"expected Ticking");
}

/// Verifies that Ticker::new() correctly initializes all fields from a given starting value.
///
/// Covers a positive value (42), a negative value (-37), and zero to confirm that
/// current_value, start_value, digit (ones-place of abs(start)), and state are all
/// set correctly regardless of the sign of the input.
fn test_new() {
    let t = Ticker::new(42);
    check("new::current_value matches start",   t.get_current_value() == 42,                    "expected 42");
    check("new::start_value set correctly",     t.get_start_value()   == 42,                    "expected 42");
    check("new::digit is ones-place of 42",     t.get_digit()         == 2,                     "expected 2");
    check("new::state is Ticking",              t.get_state()         == &TickerStates::Ticking,"expected Ticking");

    let t_neg = Ticker::new(-37);
    check("new::negative start_value",          t_neg.get_current_value() == -37,                   "expected -37");
    check("new::digit of -37 is 7 (abs %10)",   t_neg.get_digit()         == 7,                     "expected 7");
    check("new::negative state is Ticking",     t_neg.get_state()         == &TickerStates::Ticking,"expected Ticking");

    let t_zero = Ticker::new(0);
    check("new::zero start_value",              t_zero.get_current_value() == 0, "expected 0");
}

/// Verifies that Ticker::new_with_duration() correctly initializes fields when a custom
/// timer duration is provided.
///
/// The internal timer duration cannot be directly inspected without ticking, so this
/// test confirms that start_value, current_value, digit, and state are all set as
/// expected, and that construction completes without a panic.
fn test_new_with_duration() {
    let t = Ticker::new_with_duration(10, 0.5);
    check("new_with_duration::current_value",   t.get_current_value() == 10,                    "expected 10");
    check("new_with_duration::start_value",     t.get_start_value()   == 10,                    "expected 10");
    check("new_with_duration::digit",           t.get_digit()         == 0,                     "expected 0");
    check("new_with_duration::state is Ticking",t.get_state()         == &TickerStates::Ticking, "expected Ticking");

    // Timer duration cannot be directly inspected without ticking, so we just
    // confirm construction succeeds and values are correct.
    pass("new_with_duration::constructed without panic");
}

/// Verifies that Ticker::new_with_countdown() derives the correct start_value from
/// the given duration using the formula: start_value = LOOP_POINT - duration.
///
/// A duration of 10 should produce start_value = 91 (101 - 10), and the minimum
/// valid duration of 1 should produce start_value = 100 (101 - 1).  Also confirms
/// that digit and state are initialized correctly.
fn test_new_with_countdown() {
    // LOOP_POINT = 101; duration = 10 => start_value = 91
    let t = Ticker::new_with_countdown(10);
    check("new_with_countdown::start_value is 101-10=91",   t.get_start_value()   == 91,                    "expected 91");
    check("new_with_countdown::current_value is 91",        t.get_current_value() == 91,                    "expected 91");
    check("new_with_countdown::digit is 1 (91 % 10)",       t.get_digit()         == 1,                     "expected 1");
    check("new_with_countdown::state is Ticking",           t.get_state()         == &TickerStates::Ticking, "expected Ticking");

    // countdown of 1 => start_value = 100
    let t2 = Ticker::new_with_countdown(1);
    check("new_with_countdown::min duration start_value=100", t2.get_start_value() == 100, "expected 100");
}

/// Verifies that all getter methods return the correct values for a freshly constructed Ticker.
///
/// Covers get_current_value(), get_start_value(), get_digit(), get_state(), and
/// get_timer().  Since get_timer() exposes an internal Bevy Timer with no straightforward
/// value to assert against at construction time, it is only checked to confirm that it
/// returns without panicking.
fn test_getters() {
    let t = Ticker::new(55);
    check("get_current_value",  t.get_current_value() == 55,                    "expected 55");
    check("get_start_value",    t.get_start_value()   == 55,                    "expected 55");
    check("get_digit",          t.get_digit()         == 5,                     "expected 5");
    check("get_state",          t.get_state()         == &TickerStates::Ticking, "expected Ticking");

    // get_timer: just confirm it returns without panic
    let _ = t.get_timer();
    pass("get_timer::returns reference without panic");
}

/// Verifies that set_current_value() correctly updates current_value across a range of inputs.
///
/// Tests a mid-range positive value, a negative value, and both boundary values
/// (TICKER_MAX_VALUE = 100, TICKER_MIN_VALUE = -100) to confirm the setter accepts
/// the full valid range without panicking.
fn test_set_current_value() {
    let mut t = Ticker::new(0);
    t.set_current_value(77);
    check("set_current_value::value updated", t.get_current_value() == 77, "expected 77");

    t.set_current_value(-50);
    check("set_current_value::negative value", t.get_current_value() == -50, "expected -50");

    t.set_current_value(100);
    check("set_current_value::max boundary", t.get_current_value() == 100, "expected 100");

    t.set_current_value(-100);
    check("set_current_value::min boundary", t.get_current_value() == -100, "expected -100");
}

/// Verifies that set_start_value() correctly updates start_value for both positive
/// and negative inputs.
///
/// Note that set_start_value() does not automatically update current_value or digit —
/// those only change when methods that explicitly operate on them are called.
fn test_set_start_value() {
    let mut t = Ticker::new(0);
    t.set_start_value(33);
    check("set_start_value::value updated", t.get_start_value() == 33, "expected 33");

    t.set_start_value(-33);
    check("set_start_value::negative value", t.get_start_value() == -33, "expected -33");
}

/// Verifies that add_to_start() correctly modifies start_value and clamps at both boundaries.
///
/// Confirms positive addition, negative addition (subtraction), and that results
/// exceeding TICKER_MAX_VALUE (100) or falling below TICKER_MIN_VALUE (-100) are
/// clamped rather than wrapping or panicking.
fn test_add_to_start() {
    let mut t = Ticker::new(50);
    t.add_to_start(10);
    check("add_to_start::positive addition", t.get_start_value() == 60, "expected 60");

    t.add_to_start(-20);
    check("add_to_start::negative addition (subtraction)", t.get_start_value() == 40, "expected 40");

    // Clamp at max (100)
    t.add_to_start(100);
    check("add_to_start::clamps at TICKER_MAX_VALUE (100)", t.get_start_value() == 100, "expected 100");

    // Clamp at min (-100)
    t.add_to_start(-127);
    t.add_to_start(-127);
    check("add_to_start::clamps at TICKER_MIN_VALUE (-100)", t.get_start_value() == -100, "expected -100");
}

/// Verifies that add_to_current() correctly modifies current_value and clamps at both boundaries.
///
/// Mirrors the structure of test_add_to_start: positive addition, negative addition,
/// and clamping behaviour at TICKER_MAX_VALUE and TICKER_MIN_VALUE.
fn test_add_to_current() {
    let mut t = Ticker::new(0);
    t.add_to_current(25);
    check("add_to_current::positive addition", t.get_current_value() == 25, "expected 25");

    t.add_to_current(-10);
    check("add_to_current::negative addition", t.get_current_value() == 15, "expected 15");

    // Clamp at max
    t.add_to_current(100);
    check("add_to_current::clamps at max (100)", t.get_current_value() == 100, "expected 100");

    // Clamp at min
    t.add_to_current(-127);
    t.add_to_current(-127);
    check("add_to_current::clamps at min (-100)", t.get_current_value() == -100, "expected -100");
}

/// Verifies that reset() restores current_value to start_value and updates digit accordingly.
///
/// Tests both a positive start_value (20) and a negative start_value (-37) to confirm
/// that digit correctly reflects the ones-place of the absolute value of start_value
/// after the reset.  Note that reset() does not affect state.
fn test_reset() {
    let mut t = Ticker::new(20);
    t.set_current_value(80);
    t.reset();
    check("reset::current_value returns to start_value", t.get_current_value() == 20, "expected 20");
    check("reset::digit reflects start_value ones-place",  t.get_digit()         == 0,  "expected 0");

    // Reset on negative start
    let mut t2 = Ticker::new(-37);
    t2.set_current_value(0);
    t2.reset();
    check("reset::negative start restored", t2.get_current_value() == -37, "expected -37");
    check("reset::digit of -37 is 7",       t2.get_digit()         == 7,   "expected 7");
}

/// Verifies that set_to_zero() sets current_value and digit to 0 without touching start_value.
///
/// set_to_zero() is the method tick() calls internally when the loop point is reached,
/// so confirming that only current_value and digit are affected — and start_value is
/// left alone — is important for understanding how looping behavior works.
fn test_set_to_zero() {
    let mut t = Ticker::new(99);
    t.set_to_zero();
    check("set_to_zero::current_value is 0", t.get_current_value() == 0, "expected 0");
    check("set_to_zero::digit is 0",         t.get_digit()         == 0, "expected 0");
    // start_value should be unaffected
    check("set_to_zero::start_value unchanged", t.get_start_value() == 99, "expected 99");
}

/// Verifies that set_current_to_min() sets current_value to TICKER_MIN_VALUE (-100) and
/// updates digit to reflect the ones-place of its absolute value (0, since 100 % 10 = 0).
fn test_set_current_to_min() {
    let mut t = Ticker::new(50);
    t.set_current_to_min();
    check("set_current_to_min::current_value is -100", t.get_current_value() == -100, "expected -100");
    check("set_current_to_min::digit is 0 (100 % 10)", t.get_digit()         == 0,    "expected 0");
}

/// Verifies that set_current_to_max() sets current_value to TICKER_MAX_VALUE (100) and
/// updates digit to reflect the ones-place of its absolute value (0, since 100 % 10 = 0).
fn test_set_current_to_max() {
    let mut t = Ticker::new(0);
    t.set_current_to_max();
    check("set_current_to_max::current_value is 100", t.get_current_value() == 100, "expected 100");
    check("set_current_to_max::digit is 0 (100 % 10)", t.get_digit()        == 0,   "expected 0");
}

/// Verifies the three comparison methods against all three possible relationships
/// between current_value and start_value.
///
/// Each of current_is_equal_to_start(), current_is_below_start(), and
/// current_is_above_start() is checked for correctness in the equal, below, and above
/// cases, ensuring that all three methods agree with one another and never return
/// conflicting results for the same state.
fn test_comparisons() {
    let mut t = Ticker::new(50);

    // current == start (both are 50)
    check("current_is_equal_to_start::true when equal",       t.current_is_equal_to_start(), "expected true");
    check("current_is_below_start::false when equal",         !t.current_is_below_start(),   "expected false");
    check("current_is_above_start::false when equal",         !t.current_is_above_start(),   "expected false");

    // current below start
    t.set_current_value(10);
    check("current_is_below_start::true when below",          t.current_is_below_start(),    "expected true");
    check("current_is_above_start::false when below",         !t.current_is_above_start(),   "expected false");
    check("current_is_equal_to_start::false when below",      !t.current_is_equal_to_start(),"expected false");

    // current above start
    t.set_current_value(75);
    check("current_is_above_start::true when above",          t.current_is_above_start(),    "expected true");
    check("current_is_below_start::false when above",         !t.current_is_below_start(),   "expected false");
    check("current_is_equal_to_start::false when above",      !t.current_is_equal_to_start(),"expected false");
}

/// Verifies that get_distance_from_start() returns the correct signed difference
/// between current_value and start_value (CURRENT - START).
///
/// Tests zero distance (equal values), positive distance (current above start),
/// negative distance (current below start), and a boundary case where the distance
/// is 200 — the maximum possible spread — to confirm the i16 return type handles it.
fn test_get_distance_from_start() {
    let mut t = Ticker::new(50);
    check("get_distance_from_start::zero when equal", t.get_distance_from_start() == 0, "expected 0");

    t.set_current_value(80);
    check("get_distance_from_start::positive when above start", t.get_distance_from_start() == 30, "expected 30");

    t.set_current_value(20);
    check("get_distance_from_start::negative when below start", t.get_distance_from_start() == -30, "expected -30");

    // Boundary: start=-100, current=100; distance=200 (fits in i16)
    let mut t2 = Ticker::new(-100);
    t2.set_current_value(100);
    check("get_distance_from_start::large positive distance", t2.get_distance_from_start() == 200, "expected 200");
}

/// Verifies that get_countdown_value() returns the correct number of seconds remaining
/// in a countdown, and returns 0 once the countdown is complete or current_value is
/// at or below zero.
///
/// Uses new_with_countdown(10) as the baseline (start_value = 91).  Tests the full
/// countdown remaining (10), a partial countdown (5 seconds left at current = 96),
/// completion at current = 0, and the edge case of a negative current_value, both of
/// which should return 0.
fn test_get_countdown_value() {
    // Made with new_with_countdown(10) => start=91, current=91
    let t = Ticker::new_with_countdown(10);
    check("get_countdown_value::full countdown remaining is 10", t.get_countdown_value() == 10, "expected 10");

    // Simulate partway through: current = 96 => 101-96 = 5 remaining
    let mut t2 = Ticker::new_with_countdown(10);
    t2.set_current_value(96);
    check("get_countdown_value::5 seconds remaining", t2.get_countdown_value() == 5, "expected 5");

    // Countdown complete: current >= LOOP_POINT is impossible, but current=0 or negative => returns 0
    let mut t3 = Ticker::new_with_countdown(10);
    t3.set_current_value(0);
    check("get_countdown_value::returns 0 when current_value is 0", t3.get_countdown_value() == 0, "expected 0");

    let mut t4 = Ticker::new_with_countdown(10);
    t4.set_current_value(-5);
    check("get_countdown_value::returns 0 when current_value is negative", t4.get_countdown_value() == 0, "expected 0");
}

/// Verifies that pause() and unpause() correctly update both the internal Bevy timer's
/// paused status and the Ticker's TickerStates field in lockstep.
///
/// Both fields must agree: a paused Ticker should have a paused timer AND a Paused
/// state, and an unpaused Ticker should have an unpaused timer AND a Ticking state.
/// Divergence between the two would indicate a bug in either pause() or unpause().
fn test_pause_unpause() {
    let mut t = Ticker::default();

    t.pause();
    check("pause::timer is paused after pause()",       t.get_timer().is_paused(),                    "expected paused == true");
    check("pause::state is Paused after pause()",       t.get_state() == &TickerStates::Paused,    "expected Paused");

    t.unpause();
    check("unpause::timer not paused after unpause()",  !t.get_timer().is_paused(),                   "expected paused == false");
    check("unpause::state is Ticking after unpause()",  t.get_state() == &TickerStates::Ticking,   "expected Ticking");
}

/// Verifies the core tick() behaviour: value advancement, digit tracking, loop-back
/// at LOOP_POINT, and that a paused Ticker does not advance.
///
/// Real elapsed time is not available in a Startup system, so tick() is driven directly
/// with std::time::Duration::from_secs(1) to guarantee deterministic results.  The loop
/// case (current = 100 ticking to LOOP_POINT = 101) confirms that zero_out() is called
/// correctly and that state is preserved through the tick in the paused case.
fn test_tick_loop() {
    // We can't rely on real elapsed time in a Startup system, so we test
    // tick() by feeding a duration that is guaranteed to fire the timer.
    // Default timer = 1.0s repeating; passing 1 second of duration fires it once.
    let one_second = std::time::Duration::from_secs(1);

    let mut t = Ticker::new(0);
    t.tick(one_second);
    check("tick::increments current_value by 1 after one second", t.get_current_value() == 1, "expected 1");
    check("tick::digit updated after tick",                       t.get_digit()         == 1, "expected 1");

    // Digit rollover: 9 -> 10, digit should become 0
    let mut t2 = Ticker::new(9);
    t2.tick(one_second);
    check("tick::digit rolls over 9->0 at tens boundary", t2.get_digit() == 0, "expected 0");

    // Loop: current = 100, one more tick should hit LOOP_POINT (101) and zero out
    let mut t3 = Ticker::new(100);
    t3.tick(one_second);
    check("tick::loops back to 0 at LOOP_POINT", t3.get_current_value() == 0, "expected 0");
    check("tick::digit is 0 after loop",         t3.get_digit()         == 0, "expected 0");

    // Paused ticker should NOT advance
    let mut t4 = Ticker::new(5);
    t4.pause();
    t4.tick(one_second);
    check("tick::paused ticker does not advance",       t4.get_current_value() == 5,                  "expected 5");
    check("tick::state remains Paused after tick",      t4.get_state() == &TickerStates::Paused,      "expected Paused");
}

/// Dedicated state transition tests enabled by TickerStates deriving PartialEq.
///
/// Verifies every path that changes state: pause(), unpause(), double-toggling
/// (idempotency), and that state remains unaffected by set_current_value() and
/// reset() in both the Ticking and Paused conditions.  This is the authoritative
/// test for TickerStates behaviour; test_pause_unpause covers the timer/state
/// agreement specifically, while this test covers state in isolation and breadth.
fn test_ticker_states() {

    // Fresh ticker is always Ticking
    let mut t = Ticker::new(0);
    check("ticker_states::initial state is Ticking",        t.get_state() == &TickerStates::Ticking,  "expected Ticking");

    // Single pause
    t.pause();
    check("ticker_states::Paused after pause()",            t.get_state() == &TickerStates::Paused,   "expected Paused");

    // Single unpause
    t.unpause();
    check("ticker_states::Ticking after unpause()",         t.get_state() == &TickerStates::Ticking,  "expected Ticking");

    // Repeated toggles stay consistent
    t.pause();
    t.pause();
    check("ticker_states::double-pause stays Paused",       t.get_state() == &TickerStates::Paused,   "expected Paused");

    t.unpause();
    t.unpause();
    check("ticker_states::double-unpause stays Ticking",    t.get_state() == &TickerStates::Ticking,  "expected Ticking");

    // State is independent of current_value changes
    t.set_current_value(50);
    check("ticker_states::state unaffected by set_current_value", t.get_state() == &TickerStates::Ticking, "expected Ticking");

    t.pause();
    t.set_current_value(75);
    check("ticker_states::state stays Paused through set_current_value", t.get_state() == &TickerStates::Paused, "expected Paused");

    // State is independent of reset()
    t.unpause();
    t.reset();
    check("ticker_states::reset does not alter state",      t.get_state() == &TickerStates::Ticking,  "expected Ticking");

    t.pause();
    t.reset();
    check("ticker_states::reset while Paused stays Paused", t.get_state() == &TickerStates::Paused,   "expected Paused");
}

/// Confirms new() panics on out-of-range values using std::panic::catch_unwind.
fn test_new_panic_guard() {
    println!("{}[INFO] The following 2 panics are intentional and expected.{}", TestColors::INFO, TestColors::RESET);
    let over  = std::panic::catch_unwind(|| Ticker::new(101));
    let under = std::panic::catch_unwind(|| Ticker::new(-101));
    check("new::panics on value > 100",  over.is_err(),  "expected panic");
    check("new::panics on value < -100", under.is_err(), "expected panic");
}

/// Confirms new_with_countdown() panics on out-of-range durations.
fn test_countdown_panic_guard() {
    println!("{}[INFO] The following 2 panics are intentional and expected.{}", TestColors::INFO, TestColors::RESET);
    let over  = std::panic::catch_unwind(|| Ticker::new_with_countdown(101));
    let under = std::panic::catch_unwind(|| Ticker::new_with_countdown(0));
    check("new_with_countdown::panics on duration > 100", over.is_err(),  "expected panic");
    check("new_with_countdown::panics on duration < 1",   under.is_err(), "expected panic");
}
