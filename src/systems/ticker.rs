
use bevy::prelude::*;
use crate::types::Ticker;

/// Will loop through queried tickers to initiate their ticking.
pub fn ticker_ticking(
    time: Res<Time>,
    mut tickers: Query<&mut Ticker>,
) {

    let delta = time.delta();

    for mut ticker in &mut tickers {
        ticker.tick(delta);
    }
}
