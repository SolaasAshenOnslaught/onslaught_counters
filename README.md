
<!-- UNIVERSAL TAG COLLECTION -->
<!-- HEADING 1:             <h1 style="color: #EAB308;"><b>INSERT_TEXT_HERE</b></h1>                                                                    -->
<!-- HEADING 2:             <h2 style="color: #F35B13;">INSERT_TEXT_HERE</h2>                                                                           -->
<!-- HEADING 3:             <h3 style="color: #FFBE6F;">INSERT_TEXT_HERE</h3>                                                                           -->
<!-- HEADING 4:             <h4 style="color: #FFFFFF;">INSERT_TEXT_HERE</h4>                                                                           -->
<!-- EMBED TEXT:            <span title="INSERT_EMBED_TEXT_HERE" style="color: #F2F5CD;">INSERT_DISPLAY_TEXT_HERE</span>                                -->
<!-- STANDARD TEXT:         <span style="color: #FFE0E0;">INSERT_TEXT_HERE                                                                              -->

<!-- DOCUMENT SPECIFIC TAG COLLECTION -->



<!-- ################################################################################################################################################## -->
<!-- INTRODUCTION -->
<h1 style="color: #EAB308;"><b>TICKERS</b></h1>
Text
<!-- ################################################################################################################################################## -->



<!-- ################################################################################################################################################## -->
<!-- HOW TO USE THE PACKAGE -->
<h2 style="color: #F35B13;">How To Use The Package?</h2>

<!-- The Recommended Way -->
<h3 style="color: #FFBE6F;">The Recommended Way</h3>
1. <span style="color: #FFE0E0;">Add the package to your project.</span>
    ```bash
    cargo add mirth_engine_tickers
2. <span style="color: #FFE0E0;">Add `.add_plugin(Tickers{})` to your Bevy app.</span>
3. <span style="color: #FFE0E0;">You can now make use of the datatypes that this package offers.</span>

<!-- The "I Choose Features" Way -->
<h3 style="color: #FFBE6F;">The <i>"I Choose Features"</i> Way</h3>
1. <span style="color: #FFE0E0;">Add the package to your project with the default features disabled, then explicitly choose which features you want.</span>
   - `ticker_reflect` : <span style="color: #FFE0E0;">Will cause all ticker types to reflect.  Will add the bevy_reflect dependency and allow for the Tickers plugin to be usable.</span>
   - `ticker_systems` : <span style="color: #FFE0E0;">Will turn on all systems for every ticker type.  Will add the bevy_time dependency and allow for the Tickers plugin to be usable.</span>
   - `ticker_serialize` : <span style="color: #FFE0E0;">Will make it so that ticker types can be serialized and deserialized.  Will enable ticker_reflect, add the serde dependency, and allow for the Tickers plugin to be usable.</span>
       ```bash
       cargo add mirth_engine_tickers --no-default-features --features ticker_reflect, ticker_systems, ticker_serialize
2. <span style="color: #FFE0E0;">If you added any of the features, add `.add_plugin(Tickers{})` to your Bevy app.</span>
3. <span style="color: #FFE0E0;">You can now make use of the datatypes that this package offers.</span>

<!-- The "I Will Do Everything Myself" Way -->
<h3 style="color: #FFBE6F;">The <i>"I Will Do Everything Myself"</i> Way</h3>
1. <span style="color: #FFE0E0;">Add the package to your project with the default features disabled.</span>
    ```bash
    cargo add mirth_engine_tickers --no-default-features
2. <span style="color: #FFE0E0;">You can now make use of the datatypes that this package offers, but you will need to create ticking systems, reflection registrations, and serialization capability yourself.</span>
<!-- ################################################################################################################################################## -->



<!-- ################################################################################################################################################## -->
<!-- WHAT ARE THE DATATYPES THIS PACKAGE OFFERS? -->
<h2 style="color: #F35B13;">What Are The Datatypes This Package Offers?</h2>
<span style="color: #FFE0E0;">Hover over any of the following type names</span>
- `Tickers`
  - <span style="color: #FFE0E0;">A Bevy plugin used to activate ticker systems and reflections for ticker types.</span>
  - <span style="color: #FFE0E0;">Use `.add_plugin(Tickers{})` inside a Bevy app to enable ticker systems and reflections.</span>


- `Ticker`
  - <span style="color: #FFE0E0;">A type used to track the time between events.</span>
  - <span style="color: #FFE0E0;">Holds the following fields:</span>
    - start_value
    - current_value
    - end_value
    - time_interval
    - is_paused
    - is_ticking_up
    - is_handling_time_spikes
    - behavior
  - <span style="color: #FFE0E0;">Is supported by the following types:</span>
    - `TickerBehaviors`
      - <span style="color: #FFE0E0;">An enum that establishes the different types of behavior a `Ticker` can use.</span>
        - <span style="color: #FFE0E0;">Supports Looper, MutLooper, Oneshot, MutOneshot, and Freezing behaviors.</span>
    - `TickerValue`
      - <span style="color: #FFE0E0;">A trait for implementing a generic to define integer primitives a `Ticker` can potentially store for its boundary values and current value.</span>
        - <span style="color: #FFE0E0;">Supports i8, i16, and i32.</span>
    - `TickerPrecision`
      - <span style="color: #FFE0E0;">A trait for implementing a generic to define float types a `Ticker` can potentially use for its precision in tracking time.</span>
        - <span style="color: #FFE0E0;">Supports f16, f32, and f64.</span>
    - `TickerFloatBridge`
      - <span style="color: #FFE0E0;">A trait which grants the ability for f16, f32, and f64 to be passed in for the float fields of a `Ticker` constructor, no matter the precision a ticker is set to.</span>
        - <span style="color: #FFE0E0;">Eases usage of `Ticker` constructor methods.</span>
<!-- ################################################################################################################################################## -->



<!-- ################################################################################################################################################## -->
<!-- WHAT CAN I DO WITH THE TICKER DATATYPE? -->
<h2 style="color: #F35B13;">What Can I Do With The `Ticker` Datatype</h2>
<span style="color: #FFE0E0;">Hover over any of the following method names to reveal more information about them.
- <span title="INSERT_EMBED_TEXT_HERE" style="color: #FFE0E0;">INSERT_DISPLAY_TEXT_HERE</span>
<!-- ################################################################################################################################################## -->
