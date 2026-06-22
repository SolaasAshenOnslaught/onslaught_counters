
// Imports
use bevy::prelude::*;
use mirth_engine_testing_tools::{check_condition, TestColors, TestSet};
use bevy_time_structures::{Ticker, Tickers};

fn main() {
    App::new()

        .add_plugins(DefaultPlugins)
        .add_plugins(Tickers{})

        // One and done tests that are NOT intended to panic.
        .add_systems(Startup, (
            setup_update_test_ticker,
            setup_update_test_paused_ticker,
            setup_update_test_looping_ticker,
            setup_update_test_countdown_ticker,
            setup_update_test_accrual_ticker,
            test_default,
            test_new,
            test_getters,
            test_set_current_value,
            test_set_start_value,
            test_set_end_value,
            test_add_to_start_value,
            test_add_to_current_value,
            test_add_to_end_value,
            test_add_to_interval,
            test_reset,
            test_set_current_to_min,
            test_set_current_to_max,
        ).chain().in_set(TestSet::Set0))

        // One and done tests that are NOT intended to panic.
        .add_systems(Startup, (
            test_equal_methods,
            test_difference_methods,
            test_digit_methods,
            test_digit_with_dda_methods,
            test_pause_unpause,
            test_looping_toggle,
            test_tick_direction_toggle,
            test_frame_spike_toggle,
            test_percentage_from_start,
            test_percentage_from_end,
            test_tick_counts_up,
            test_tick_counts_down,
            test_tick_loops_at_boundary,
            test_tick_paused_does_not_advance,
            test_tick_frame_spike_handling,
        ).chain().in_set(TestSet::Set1))

        // One and done tests that are intended to panic.
        .add_systems(Startup, (
            test_new_panic_guard,
        ).chain().in_set(TestSet::Set2))

        // Tests that occur on every frame.
        .add_systems(Update, (
            // test_update_ticker_advances_forward,
            // test_update_ticker_stays_within_range,
            // test_update_paused_ticker_never_advances,
            // test_update_looping_ticker_never_exceeds_end,
            // test_update_countdown_ticker_never_increases,
            // test_update_accrual_ticker_has_not_fired_early,
            test_update_print_info,
        ).chain().in_set(TestSet::Set3))

        .configure_sets(Startup, TestSet::Set0.before(TestSet::Set1))
        .configure_sets(Startup, TestSet::Set1.before(TestSet::Set2))
        .configure_sets(Startup, TestSet::Set1.before(TestSet::Set3))

        .run();
}

// ################################### NON-PANIC STARTUP TESTS ################################## //
/// Verifies that Ticker::default() produces the documented baseline state:
/// start_value = 0, current_value = 0, end_value = 100, interval = 1.0,
/// not paused, looping, ticking up, and handling frame spikes.
fn test_default() {

    let ticker: Ticker<i32, f32> = Ticker::default();
    check_condition(
        ticker.start_value() == 0,
        "default::start_value is 0",
        "expected 0"
    );


    check_condition(
        ticker.current_value() == 0,
        "default::current_value is 0",
        "expected 0"
    );


    check_condition(
        ticker.end_value() == 100,
        "default::end_value is 100",
        "expected 100"
    );


    check_condition(
        ticker.interval() == 1.0,
        "default::interval is 1.0",
        "expected 1.0"
    );


    check_condition(
        !ticker.is_paused(),
        "default::is not paused",
        "expected unpaused"
    );


    check_condition(
        ticker.is_looping(),
        "default::is looping",
        "expected looping"
    );


    check_condition(
        ticker.is_ticking_up(),
        "default::is ticking up",
        "expected ticking up"
    );


    check_condition(
        ticker.is_handling_frame_spikes(),
        "default::is handling frame spikes",
        "expected frame spike handling on"
    );
}

/// Verifies that Ticker::new() correctly initializes all fields from the given values,
/// covering a positive start_value, a negative one, and a current_value that differs
/// from start_value but still lies within range.
fn test_new() {

    let ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker.start_value() == 0,
        "new::start_value set correctly",
        "expected 0"
    );


    check_condition(
        ticker.current_value() == 0,
        "new::current_value set correctly",
        "expected 0"
    );


    check_condition(
        ticker.end_value() == 100,
        "new::end_value set correctly",
        "expected 100"
    );


    let ticker_negative: Ticker<i32, f32> = Ticker::new(
        -37,
        -37,
        50,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker_negative.start_value() == -37,
        "new::negative start_value",
        "expected -37"
    );


    check_condition(
        ticker_negative.current_value() == -37,
        "new::negative current_value",
        "expected -37"
    );


    let ticker_mid: Ticker<i32, f32> = Ticker::new(
        0,
        40,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker_mid.current_value() == 40,
        "new::current_value differs from start_value",
        "expected 40"
    );
}

/// Verifies that all getter methods return the values set at construction.
fn test_getters() {

    let ticker: Ticker<i32, f32> = Ticker::new(
        10,
        25,
        55,
        2.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker.start_value() == 10,
        "get_start_value",
        "expected 10"
    );


    check_condition(
        ticker.current_value() == 25,
        "get_current_value",
        "expected 25"
    );


    check_condition(
        ticker.end_value() == 55,
        "get_end_value",
        "expected 55"
    );


    check_condition(
        ticker.interval() == 2.0,
        "get_interval",
        "expected 2.0"
    );


    check_condition(
        !ticker.is_paused(),
        "get_is_paused",
        "expected false"
    );


    check_condition(
        ticker.is_looping(),
        "get_is_looping",
        "expected true"
    );
}

/// Verifies that set_current_value() updates current_value, including clamping
/// to the start_value/end_value range rather than panicking on out-of-range input.
fn test_set_current_value() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        100,
        1.0,
        false,
        true,
        true,
        true
    );

    ticker.set_current_value(77);
    check_condition(
        ticker.current_value() == 77,
        "set_current_value::value updated",
        "expected 77"
    );


    ticker.set_current_value(150);
    check_condition(
        ticker.current_value() == 100,
        "set_current_value::clamps at end_value",
        "expected 100"
    );


    ticker.set_current_value(-50);
    check_condition(
        ticker.current_value() == 0,
        "set_current_value::clamps at start_value",
        "expected 0"
    );
}

/// Verifies that set_start_value() updates start_value and clamps it to V::MIN/V::MAX.
fn test_set_start_value() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        100,
        1.0,
        false,
        true,
        true,
        true
    );

    ticker.set_start_value(33);
    check_condition(
        ticker.start_value() == 33,
        "set_start_value::value updated",
        "expected 33"
    );


    ticker.set_start_value(-33);
    check_condition(
        ticker.start_value() == -33,
        "set_start_value::negative value",
        "expected -33"
    );
}

/// Verifies that set_end_value() updates end_value and clamps it to V::MIN/V::MAX.
fn test_set_end_value() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        100,
        1.0,
        false,
        true,
        true,
        true
    );

    ticker.set_end_value(200);
    check_condition(
        ticker.end_value() == 200,
        "set_end_value::value updated",
        "expected 200"
    );
}

/// Verifies that add_to_start_value() modifies start_value and clamps at V::MIN/V::MAX.
fn test_add_to_start_value() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        50,
        50,
        1000,
        1.0,
        false,
        true,
        true,
        true
    );

    ticker.add_to_start_value(10);
    check_condition(
        ticker.start_value() == 60,
        "add_to_start_value::positive addition",
        "expected 60"
    );


    ticker.add_to_start_value(-20);
    check_condition(
        ticker.start_value() == 40,
        "add_to_start_value::negative addition (subtraction)",
        "expected 40"
    );
}

/// Verifies that add_to_current_value() modifies current_value and clamps within
/// the start_value/end_value range.
fn test_add_to_current_value() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        100,
        1.0,
        false,
        true,
        true,
        true
    );

    ticker.add_to_current_value(25);
    check_condition(
        ticker.current_value() == 25,
        "add_to_current_value::positive addition",
        "expected 25"
    );


    ticker.add_to_current_value(-10);
    check_condition(
        ticker.current_value() == 15,
        "add_to_current_value::negative addition",
        "expected 15"
    );


    ticker.add_to_current_value(1000);
    check_condition(
        ticker.current_value() == 100,
        "add_to_current_value::clamps at end_value",
        "expected 100"
    );
}

/// Verifies that add_to_end_value() modifies end_value and clamps at V::MIN/V::MAX.
fn test_add_to_end_value() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        100,
        1.0,
        false,
        true,
        true,
        true
    );

    ticker.add_to_end_value(50);
    check_condition(
        ticker.end_value() == 150,
        "add_to_end_value::positive addition",
        "expected 150"
    );
}

/// Verifies that add_to_interval() modifies interval and clamps it to a positive value,
/// never allowing it to reach zero or go negative.
fn test_add_to_interval() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        100,
        1.0,
        false,
        true,
        true,
        true
    );

    ticker.add_to_interval(0.5);
    check_condition(
        ticker.interval() == 1.5,
        "add_to_interval::positive addition",
        "expected 1.5"
    );


    ticker.add_to_interval(-100.0);
    check_condition(
        ticker.interval() > 0.0,
        "add_to_interval::clamps above zero rather than going negative",
        "expected interval > 0.0"
    );
}

/// Verifies that reset() restores current_value to start_value, for both a positive
/// and a negative start_value.
fn test_reset() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        20,
        20,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    ticker.set_current_value(80);
    ticker.reset();
    check_condition(
        ticker.current_value() == 20,
        "reset::current_value returns to start_value",
        "expected 20"
    );


    let mut ticker_negative: Ticker<i32, f32> = Ticker::new(
        -37,
        -37,
        50,
        1.0,
        false,
        true,
        true,
        true
    );
    ticker_negative.set_current_value(0);
    ticker_negative.reset();
    check_condition(
        ticker_negative.current_value() == -37,
        "reset::negative start_value restored",
        "expected -37"
    );
}

/// Verifies that setting current_value to the start/end range minimum behaves as expected,
/// replacing the old set_current_to_min() helper with set_current_value(start_value).
fn test_set_current_to_min() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        -100,
        50,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    ticker.set_current_value(ticker.start_value());
    check_condition(
        ticker.current_value() == -100,
        "set_current_to_min::current_value matches start_value",
        "expected -100"
    );
}

/// Verifies that setting current_value to the start/end range maximum behaves as expected,
/// replacing the old set_current_to_max() helper with set_current_value(end_value).
fn test_set_current_to_max() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    ticker.set_current_value(ticker.end_value());
    check_condition(
        ticker.current_value() == 100,
        "set_current_to_max::current_value matches end_value",
        "expected 100"
    );
}

/// Verifies the three equality-comparison methods: is_current_equal_to_start(),
/// is_current_equal_to_end(), and is_start_equal_to_end().
fn test_equal_methods() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        50,
        50,
        50,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker.is_current_equal_to_start(),
        "is_current_equal_to_start::true when equal",
        "expected true"
    );


    check_condition(
        ticker.is_current_equal_to_end(),
        "is_current_equal_to_end::true when equal",
        "expected true"
    );


    check_condition(
        ticker.is_start_equal_to_end(),
        "is_start_equal_to_end::true when equal",
        "expected true"
    );


    ticker.set_current_value(10);
    let ticker_below: Ticker<i32, f32> = Ticker::new(
        0,
        10,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        !ticker_below.is_current_equal_to_start(),
        "is_current_equal_to_start::false when below start",
        "expected false"
    );


    check_condition(
        !ticker_below.is_current_equal_to_end(),
        "is_current_equal_to_end::false when below end",
        "expected false"
    );
}

/// Verifies that difference_from_start(), difference_from_end(), and
/// difference_from_start_to_end() all return correct, always-positive distances.
fn test_difference_methods() {

    let ticker: Ticker<i32, f32> = Ticker::new(
        -100,
        50,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker.difference_from_start() == 150,
        "difference_from_start::correct distance",
        "expected 150"
    );


    check_condition(
        ticker.difference_from_end() == 50,
        "difference_from_end::correct distance",
        "expected 50"
    );


    check_condition(
        ticker.difference_from_start_to_end() == 200,
        "difference_from_start_to_end::correct distance",
        "expected 200"
    );


    let ticker_equal: Ticker<i32, f32> = Ticker::new(
        50,
        50,
        50,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker_equal.difference_from_start() == 0,
        "difference_from_start::zero when equal",
        "expected 0"
    );
}

/// Verifies digit_1(), digit_2(), and digit_3() correctly extract the ones, tens,
/// and hundreds digits of current_value, regardless of sign.
fn test_digit_methods() {

    let ticker: Ticker<i32, f32> = Ticker::new(
        0,
        123,
        1000,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker.digit_1() == 3,
        "digit_1::ones place of 123",
        "expected 3"
    );


    check_condition(
        ticker.digit_2() == 2,
        "digit_2::tens place of 123",
        "expected 2"
    );


    check_condition(
        ticker.digit_3() == 1,
        "digit_3::hundreds place of 123",
        "expected 1"
    );


    let ticker_negative: Ticker<i32, f32> = Ticker::new(
        -1000,
        -45,
        0,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker_negative.digit_1() == 5,
        "digit_1::ones place of -45 uses absolute value",
        "expected 5"
    );


    check_condition(
        ticker_negative.digit_2() == 4,
        "digit_2::tens place of -45 uses absolute value",
        "expected 4"
    );


    let ticker_small: Ticker<i32, f32> = Ticker::new(
        0,
        7,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker_small.digit_2() == 0,
        "digit_2::returns 0 when tens place doesn't exist",
        "expected 0"
    );


    check_condition(
        ticker_small.digit_3() == 0,
        "digit_3::returns 0 when hundreds place doesn't exist",
        "expected 0"
    );
}

/// Verifies digit_2_with_dda() and digit_3_with_dda() correctly distinguish between
/// a digit that is absent (-1) and a digit that is present but happens to be 0.
fn test_digit_with_dda_methods() {

    let ticker_absent: Ticker<i32, f32> = Ticker::new(
        0,
        6,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker_absent.digit_2_with_dda() == -1,
        "digit_2_with_dda::-1 when tens place absent",
        "expected -1"
    );


    let ticker_present: Ticker<i32, f32> = Ticker::new(
        0,
        63,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker_present.digit_2_with_dda() == 6,
        "digit_2_with_dda::6 when tens place present",
        "expected 6"
    );


    let ticker_zero_digit: Ticker<i32, f32> = Ticker::new(
        0,
        1003,
        2000,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker_zero_digit.digit_3_with_dda() == 0,
        "digit_3_with_dda::0 when hundreds place present but is zero",
        "expected 0"
    );
}

/// Verifies that pause() and unpause() correctly update is_paused().
fn test_pause_unpause() {

    let mut ticker: Ticker<i32, f32> = Ticker::default();

    ticker.pause();
    check_condition(
        ticker.is_paused(),
        "pause::is_paused true after pause()",
        "expected true"
    );


    ticker.unpause();
    check_condition(
        !ticker.is_paused(),
        "unpause::is_paused false after unpause()",
        "expected false"
    );
}

/// Verifies that start_looping() and stop_looping() correctly update is_looping().
fn test_looping_toggle() {

    let mut ticker: Ticker<i32, f32> = Ticker::default();

    ticker.stop_looping();
    check_condition(
        !ticker.is_looping(),
        "stop_looping::is_looping false",
        "expected false"
    );


    ticker.start_looping();
    check_condition(
        ticker.is_looping(),
        "start_looping::is_looping true",
        "expected true"
    );
}

/// Verifies that tick_up() and tick_down() correctly update is_ticking_up().
fn test_tick_direction_toggle() {

    let mut ticker: Ticker<i32, f32> = Ticker::default();

    ticker.tick_down();
    check_condition(
        !ticker.is_ticking_up(),
        "tick_down::is_ticking_up false",
        "expected false"
    );


    ticker.tick_up();
    check_condition(
        ticker.is_ticking_up(),
        "tick_up::is_ticking_up true",
        "expected true"
    );
}

/// Verifies that start_handling_frame_spikes() and stop_handling_frame_spikes()
/// correctly update is_handling_frame_spikes().
fn test_frame_spike_toggle() {

    let mut ticker: Ticker<i32, f32> = Ticker::default();

    ticker.stop_handling_frame_spikes();
    check_condition(
        !ticker.is_handling_frame_spikes(),
        "stop_handling_frame_spikes::false",
        "expected false"
    );


    ticker.start_handling_frame_spikes();
    check_condition(
        ticker.is_handling_frame_spikes(),
        "start_handling_frame_spikes::true",
        "expected true"
    );
}

/// Verifies that percentage_from_start() returns the correct fraction of current_value's
/// distance from start_value, and returns -1.0 when start_value equals end_value.
fn test_percentage_from_start() {

    let ticker: Ticker<i32, f32> = Ticker::new(
        0,
        40,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        (ticker.percentage_from_start() - 0.4).abs() < 0.0001,
        "percentage_from_start::0.4 for 40 in 0..100",
        "expected ~0.4"
    );


    let ticker_equal: Ticker<i32, f32> = Ticker::new(
        100,
        100,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker_equal.percentage_from_start() == -1.0,
        "percentage_from_start::-1.0 when start equals end",
        "expected -1.0"
    );
}

/// Verifies that percentage_from_end() returns the correct fraction of current_value's
/// distance from end_value, and returns -1.0 when start_value equals end_value.
fn test_percentage_from_end() {

    let ticker: Ticker<i32, f32> = Ticker::new(
        0,
        60,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        (ticker.percentage_from_end() - 0.4).abs() < 0.0001,
        "percentage_from_end::0.4 for 60 in 0..100",
        "expected ~0.4"
    );


    let ticker_equal: Ticker<i32, f32> = Ticker::new(
        100,
        100,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    check_condition(
        ticker_equal.percentage_from_end() == -1.0,
        "percentage_from_end::-1.0 when start equals end",
        "expected -1.0"
    );
}

/// Verifies that tick() increments current_value when ticking up and enough delta
/// time has accrued to cross the interval threshold.
fn test_tick_counts_up() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    ticker.tick(1.0);
    check_condition(
        ticker.current_value() == 1,
        "tick::increments current_value by 1 after one full interval",
        "expected 1"
    );
}

/// Verifies that tick() decrements current_value when ticking down and enough delta
/// time has accrued to cross the interval threshold.
fn test_tick_counts_down() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        50,
        100,
        1.0,
        false,
        true,
        false,
        true
    );
    ticker.tick(1.0);
    check_condition(
        ticker.current_value() == 49,
        "tick::decrements current_value by 1 when ticking down",
        "expected 49"
    );
}

/// Verifies that a looping Ticker resets current_value to start_value upon reaching
/// either boundary, rather than exceeding the start_value/end_value range.
fn test_tick_loops_at_boundary() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        99,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    ticker.tick(1.0);
    check_condition(
        ticker.current_value() == 0,
        "tick::loops back to start_value at end_value boundary",
        "expected 0"
    );
}

/// Verifies that a paused Ticker does not advance current_value regardless of delta passed in.
fn test_tick_paused_does_not_advance() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        5,
        100,
        1.0,
        true,
        true,
        true,
        true
    );
    ticker.tick(1.0);
    check_condition(
        ticker.current_value() == 5,
        "tick::paused ticker does not advance",
        "expected 5"
    );
}

/// Verifies that a Ticker with frame spike handling enabled correctly fires multiple
/// ticks at once when a single large delta is passed in, rather than firing only once.
fn test_tick_frame_spike_handling() {

    let mut ticker: Ticker<i32, f32> = Ticker::new(
        0,
        0,
        100,
        1.0,
        false,
        true,
        true,
        true
    );
    ticker.tick(3.5);
    check_condition(
        ticker.current_value() == 3,
        "tick::frame spike handling fires multiple ticks for a large delta",
        "expected 3"
    );
}
// ############################################################################################## //



// ###################################### PANIC STARTUP TESTS ################################### //
/// Confirms new() panics on a current_value outside the start_value/end_value range,
/// using std::panic::catch_unwind to verify the panic without crashing the test suite.
fn test_new_panic_guard() {

    println!("{}[INFO]{} The following panic is intentional and expected.", TestColors::INFO, TestColors::DEFAULT);
    let result = std::panic::catch_unwind(|| {
        let _ticker: Ticker<i32, f32> = Ticker::new(
            0,
            150,
            100,
            1.0,
            false,
            true,
            true,
            true
        );
    });
    check_condition(
        result.is_err(),
        "new::panics when current_value is outside start_value/end_value range",
        "expected panic"
    );
}
// ############################################################################################## //



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
