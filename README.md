
# TICKERS
Text

## How To Use The Package?

### The Recommended Way
1. Add the package to your project.
    ```bash
    cargo add mirth_engine_tickers
2. Add `.add_plugin(Tickers{})` to your Bevy app.
3. You can now make use of the datatypes that this package offers.

### The "I Choose Features Route" Way
1. Add the package to your project with the default features disabled, then explicitly choose which features you want.
- `ticker_reflect` : Will cause all ticker types to reflect.  Will add the bevy_reflect dependency and allow for the Tickers plugin to be usable.
- `ticker_systems` : Will turn on all systems for every ticker type.  Will add the bevy_time dependency and allow for the Tickers plugin to be usable.
- `ticker_serialize` : Will make it so that ticker types can be serialized and deserialized.  Will enable ticker_reflect, add the serde dependency, and allow for the Tickers plugin to be usable.
    ```bash
    cargo add mirth_engine_tickers --no-default-features --features ticker_reflect, ticker_systems, ticker_serialize
2. If you added any of the features, add `.add_plugin(Tickers{})` to your Bevy app.
3. You can now make use of the datatypes that this package offers.

### The "I Will Do Everything Myself" Way
1. Add the package to your project with the default features disabled.
    ```bash
    cargo add mirth_engine_tickers --no-default-features
3. You can now make use of the datatypes that this package offers, but you will need to create ticking systems, reflection registrations, and serialization capability yourself.



## What Are The Datatypes This Package Offers? 
- `Tickers`
  - A Bevy plugin used to activate ticker systems and reflections for ticker types.
  - Use `.add_plugin(Tickers{})` inside a Bevy app to enable ticker systems and reflections.


- `Ticker`
  - Text
  - `TickerBehaviors`
    - An enum that establishes the different types of behavior a ticker can use.  Behaviors supported are as follows:
      - Looper
      - MutLooper
      - Oneshot
      - MutOneshot
      - Freezing
  - `TickerValue`
    - A trait for implementing a generic to define integer primitives a ticker can potentially store for its boundary values and current value.
      - Supports i8, i16, and i32.
  - `TickerPrecision`
    - A trait for implementing a generic to define float types a ticker can potentially use for its precision in tracking time.
      - Supports f16, f32, and f64.
  - `TickerFloatBridge`
    - A trait which grants the ability for f16, f32, and f64 to be passed in for the float fields of a ticker constructor, no matter the precision a ticker is set to.  Eases usage of constructor methods.

## What Features Are Supported for the `Ticker` Datatype?
- <span title="Text">New Thing</span>
