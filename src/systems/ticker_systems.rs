
use bevy_ecs::prelude::*;
use bevy_time::Time;
use crate::types::ticker_type::*;

/// Advances every `Ticker<V, f32>` in the world by the time elapsed since the last frame,
/// using `f32` precision for delta time.
///
/// This tick system is based on Bevy's Time resource, and more specifically its delta.  Best
/// used in Bevy's scheduler under a frame schedule (Update, First, Last, etcetera).
pub fn tick_f32_tickers<V: TickerValue>(
    time: Res<Time>,
    mut tickers: Query<&mut Ticker<V, f32>>,
) {
    let delta_in_seconds: f32 = time.delta_secs();
    for mut ticker in tickers.iter_mut() {
        ticker.tick(delta_in_seconds);
    }
}

/// Advances every `Ticker<V, f64>` in the world by the time elapsed since the last frame,
/// using `f64` precision for delta time.
///
/// This tick system is based on Bevy's Time resource, and more specifically its delta.  Best
/// used in Bevy's scheduler under a frame schedule (Update, First, Last, etcetera).
pub fn tick_f64_tickers<V: TickerValue>(
    time: Res<Time>,
    mut tickers: Query<&mut Ticker<V, f64>>,
) {
    let delta_in_seconds: f64 = time.delta_secs_f64();
    for mut ticker in tickers.iter_mut() {
        ticker.tick(delta_in_seconds);
    }
}