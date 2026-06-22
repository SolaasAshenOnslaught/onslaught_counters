
// Imports
use bevy::prelude::*;
use crate::types::*;
use crate::systems::*;

/// A structure that acts as the main plugin for all the types and systems of the mirth_engine_tickers package.
/// Will enable types and systems within the package depending on which features have been enabled
/// using the "cargo --features" command.
pub struct Tickers {}
impl Plugin for Tickers {
    fn build(&self, app: &mut App) {

        // ############################### TICKER FEATURES ###################################### //
        // Types
        #[cfg(feature = "ticker_type_i8_f32")]
        app.register_type::<Ticker<i8, f32>>();

        #[cfg(feature = "ticker_type_i16_f32")]
        app.register_type::<Ticker<i16, f32>>();

        #[cfg(feature = "ticker_type_i32_f32")]
        app.register_type::<Ticker<i32, f32>>();

        #[cfg(feature = "ticker_type_i8_f64")]
        app.register_type::<Ticker<i8, f64>>();

        #[cfg(feature = "ticker_type_i16_f64")]
        app.register_type::<Ticker<i16, f64>>();

        #[cfg(feature = "ticker_type_i32_f64")]
        app.register_type::<Ticker<i32, f64>>();

        // Systems
        #[cfg(feature = "ticker_systems_i8_f32")]
        app.add_systems(First, tick_tickers::<i8, f32>);

        #[cfg(feature = "ticker_systems_i16_f32")]
        app.add_systems(First, tick_tickers::<i16, f32>);

        #[cfg(feature = "ticker_systems_i32_f32")]
        app.add_systems(First, tick_tickers::<i32, f32>);

        #[cfg(feature = "ticker_systems_i8_f64")]
        app.add_systems(First, tick_tickers::<i8, f64>);

        #[cfg(feature = "ticker_systems_i16_f64")]
        app.add_systems(First, tick_tickers::<i16, f64>);

        #[cfg(feature = "ticker_systems_i32_f64")]
        app.add_systems(First, tick_tickers::<i32, f64>);
        // ###################################################################################### //



        // ############################### TICKERLOG FEATURES ################################### //
        // Types

        // Systems
        // ###################################################################################### //
    }
}
