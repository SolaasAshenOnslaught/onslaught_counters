
use bevy::prelude::*;
use crate::ticker::{TickerPrecision, TickerValue};
use crate::types::Ticker;

/// Will loop through queried tickers and tick them for every time this Bevy system is used.
///
/// This tick system is based on Bevy's Time resource, and more specifically its delta.  Best
/// used in Bevy's scheduler under a frame schedule (Update, First, Last, etcetera).
pub fn tick_tickers<V: TickerValue, P: TickerPrecision>(
    time: Res<Time>,
    mut tickers: Query<&mut Ticker<V, P>>,
) {

    // P::from_f64 will either keep time.delta_secs_f64 as a f64 value or convert it to f32
    // depending on the type of tick_tickers system being used.
    let delta_in_seconds: P = P::from_f64(time.delta_secs_f64());

    for mut ticker in tickers.iter_mut() {
        ticker.tick(delta_in_seconds);
    }
}
