
// Imports
use bevy_app::prelude::*;
use bevy_app::ScheduleRunnerPlugin;
use bevy_ecs::prelude::*;
use bevy_time::TimePlugin;
use half::f16;
use std::time::Duration;
use mirth_engine_tickers::*;
#[cfg(feature = "testing_tools")]
use mirth_engine_testing_tools::*;

fn main() {
    App::new()

        // 1.0 / 60.0 == Loop 60 Times Per 1 Second
        // Change the calculation to w/e you want for testing.
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0)))
        .add_plugins(TimePlugin)
        .add_plugins(Tickers{})

        // Comment this add_system line to prevent the master ticker from spawning.
        .add_systems(Startup, spawn_custom_ticker)

        // Comment this add_system line to prevent the variant tickers from spawning.
        .add_systems(Startup, spawn_ticker_variants)

        // Don't touch this unless you want the tickers to NOT print their information.
        .add_systems(Update, print_ticker_information)
        .run();
}

fn spawn_custom_ticker(mut commands: Commands) {

    // Change the fields to whatever you want to test any kind of ticker.
    commands.spawn(Ticker::<i32, f32>::new(
        0,
        5,
        25,
        1.0,
        true,
        true,
        true,
        TickerBehaviors::MutLooper,
    ));
}

fn spawn_ticker_variants(mut commands: Commands) {

    // LOOPER VARIATIONS
    commands.spawn(Ticker::<i8, f16>::new_looper(
        0,
        100,
        1.0,
        true
    ));
    commands.spawn(Ticker::<i16, f32>::new_looper_custom(
        50,
        20,
        -50,
        0.5,
        false,
        false,
    ));


    // MUTLOOPER VARIATIONS
    commands.spawn(Ticker::<i32, f16>::new_mut_looper(
        0,
        10,
        0.1,
        true
    ));
    commands.spawn(Ticker::<i16, f32>::new_mut_looper_custom(
        1000,
        500,
        0,
        5.0,
        false,
        true,
    ));


    // ONESHOT VARIATIONS
    commands.spawn(Ticker::<i32, f32>::new_oneshot(
        0,
        100_000,
        10.0,
        true
    ));
    commands.spawn(Ticker::<i8, f64>::new_oneshot_custom(
        -10,
        0,
        10,
        0.25,
        true,
        false,
    ));


    // MUTONESHOT VARIATIONS
    commands.spawn(Ticker::<i16, f16>::new_mut_oneshot(
        0,
        30_000,
        1.0,
        true
    ));
    commands.spawn(Ticker::<i32, f32>::new_mut_oneshot_custom(
        100,
        75,
        0,
        0.016,
        false,
        true,
    ));


    // FREEZING VARIATIONS
    commands.spawn(Ticker::<i32, f16>::new_freezing(
        0,
        50,
        2.5,
        false
    ));
    commands.spawn(Ticker::<i32, f64>::new_freezing_custom(
        -100_000,
        -100_000,
        100_000,
        1.0,
        true,
        true,
    ));
}

fn print_ticker_information(
    tickers_i8_f16:   Query<&Ticker<i8, f16>>,
    tickers_i16_f16:  Query<&Ticker<i16, f16>>,
    tickers_i32_f16:  Query<&Ticker<i32, f16>>,

    tickers_i8_f32:   Query<&Ticker<i8, f32>>,
    tickers_i16_f32:  Query<&Ticker<i16, f32>>,
    tickers_i32_f32:  Query<&Ticker<i32, f32>>,

    tickers_i8_f64:   Query<&Ticker<i8, f64>>,
    tickers_i16_f64:  Query<&Ticker<i16, f64>>,
    tickers_i32_f64:  Query<&Ticker<i32, f64>>,
) {
    macro_rules! print_queries {
        ($($query:expr),* $(,)?) => {
            $(
                for ticker in &$query {
                    ticker.print_information();
                    println!();
                }
            )*
        };
    }

    print_queries!(
        tickers_i8_f16,
        tickers_i16_f16,
        tickers_i32_f16,

        tickers_i8_f32,
        tickers_i16_f32,
        tickers_i32_f32,

        tickers_i8_f64,
        tickers_i16_f64,
        tickers_i32_f64,
    );
}
