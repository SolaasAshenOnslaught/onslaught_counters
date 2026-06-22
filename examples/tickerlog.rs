
use bevy::prelude::*;
use bevy_time_structures::Tickers;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Tickers{})
        .run();
}
