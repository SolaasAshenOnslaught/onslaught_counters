
// Imports
use bevy_app::prelude::*;
use crate::types::*;
use crate::systems::*;

/// A structure that acts as the main plugin for all the types and systems of the mirth_engine_tickers package.
/// Will enable types and systems within the package depending on which features have been enabled
/// using the "cargo --features" command.
pub struct Tickers {}
impl Plugin for Tickers {
    fn build(&self, app: &mut App) {

        // Reflecting Ticker Types
        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i8, f32>>();

        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i16, f32>>();

        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i32, f32>>();

        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i8, f64>>();

        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i16, f64>>();

        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i32, f64>>();

        // Ticker Systems
        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_f32_tickers::<i8>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_f32_tickers::<i16>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_f32_tickers::<i32>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_f64_tickers::<i8>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_f64_tickers::<i16>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_f64_tickers::<i32>);
    }
}
