
// Imports
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, Rem, RemAssign, Sub, SubAssign};

/// Used to apply a generic to the `start_value`, `current_value`, and `end_value` within the ticker type.
///
/// Supports i8, i16, i32 for `start_value`, `current_value`, and `end_value` within Ticker.
///
/// #### Why Add 1 to MIN?
/// The MIN addition is present to help avoid absolute errors on integer ranges.  MIN's
/// assignment on value types will always add 1 to an integer's minimum to avoid things like -128 in
/// the i8 datatype becoming 128 after .absolute() is applied to a value.  We have to do this since
/// 128 is outside the i8 range; 127 is the max for i8.
pub trait TickerValue:
Copy                    // TickerValue types are integers, which means they're safe to copy.
+ Ord                   // TickerValue types are integers, hence Ord is necessary for comparison.
+ Display               // There are checks (that can display) to determine if the values are within their acceptable ranges.
+ Add<Output = Self>
+ Sub<Output = Self>
+ Div<Output = Self>
+ Rem<Output = Self>
+ Send                  // Needed for Bevy queries; also lets Tickers move safely across threads.
+ Sync                  // Needed for Bevy queries; also lets Tickers be shared safely across threads.
+ 'static               // Needed for Bevy queries; also enforces that TickerValue types own their data, with no borrowed lifetimes.
{
    const MIN: Self;
    const MAX: Self;
    fn absolute(self)               -> Self;
    fn sat_add(self, value: Self)   -> Self;
    fn sat_sub(self, value: Self)   -> Self;
    fn as_f32(self)                 -> f32;
    fn as_i8(self)                  -> i8;
    fn as_i64(self)                 -> i64;
    fn from_f64(value: f64)         -> Self;
    fn from_i32(val: i32)           -> Self;
}

impl TickerValue for i8 {
    const MIN: Self                 = i8::MIN + 1;
    const MAX: Self                 = i8::MAX;
    fn absolute(self)               -> Self { self.abs() }
    fn sat_add(self, value: Self)   -> Self { self.saturating_add(value) }
    fn sat_sub(self, value: Self)   -> Self { self.saturating_sub(value) }
    fn as_f32(self)                 -> f32  { self as f32 }
    fn as_i8(self)                  -> i8   { self }
    fn as_i64(self)                 -> i64  { self as i64 }
    fn from_f64(value: f64)         -> Self { value as i8 }
    fn from_i32(value: i32)         -> Self { value as i8 }
}

impl TickerValue for i16 {
    const MIN: Self                 = i16::MIN + 1;
    const MAX: Self                 = i16::MAX;
    fn absolute(self)               -> Self { self.abs() }
    fn sat_add(self, value: Self)   -> Self { self.saturating_add(value) }
    fn sat_sub(self, value: Self)   -> Self { self.saturating_sub(value) }
    fn as_f32(self)                 -> f32  { self as f32 }
    fn as_i8(self)                  -> i8   { self as i8 }
    fn as_i64(self)                 -> i64  { self as i64 }
    fn from_f64(value: f64)         -> Self { value as i16 }
    fn from_i32(value: i32)         -> Self { value as i16 }
}

impl TickerValue for i32 {
    const MIN: Self                 = i32::MIN + 1;
    const MAX: Self                 = i32::MAX;
    fn absolute(self)               -> Self { self.abs() }
    fn sat_add(self, value: Self)   -> Self { self.saturating_add(value) }
    fn sat_sub(self, value: Self)   -> Self { self.saturating_sub(value) }
    fn as_f32(self)                 -> f32  { self as f32 }
    fn as_i8(self)                  -> i8   { self as i8 }
    fn as_i64(self)                 -> i64  { self as i64 }
    fn from_f64(value: f64)         -> Self { value as i32 }
    fn from_i32(value: i32)         -> Self { value }
}



/// Used to apply a generic to the `stored_time` and `time_interval` fields within the ticker type.
///
/// Supports f32 and f64 for `stored_time` and `time_interval` fields within Ticker.
///
/// #### Why Add Precision?
/// f32 and f64 types for precision determine how accurate the calculations inside the .tick() method are.
/// f32 being less accurate, f64 being more. Precision control is useful if ridiculous levels of accuracy
/// is crucial, otherwise pointless.  In most cases, f64 precision is not necessary.
///
/// #### When to Consider More Precision?
/// I'd say the only scenarios where the precision jump becomes important is for big clocks (world clocks)
/// that can impact many entities, or if PvP is involved in a game and the timing of things should be as
/// accurate as possible to reduce frustration.
pub trait TickerPrecision:
Copy                    // TickerPrecision types are floats, which means they're safe to copy.
+ PartialOrd            // TickerPrecision types are floats, hence PartialOrd is necessary for comparisons.
+ Add<Output = Self>
+ Sub<Output = Self>
+ Div<Output = Self>
+ Rem<Output = Self>
+ AddAssign
+ SubAssign
+ RemAssign
+ Send                  // Needed for Bevy queries; also lets Tickers move safely across threads.
+ Sync                  // Needed for Bevy queries; also lets Tickers be shared safely across threads.
+ 'static               // Needed for Bevy queries; also enforces that TickerPrecision types own their data, with no borrowed lifetimes.
{
    const MIN_POSITIVE: Self;
    const MAX: Self;
    fn clamp(self, min: Self, max: Self)    -> Self;
    fn as_f64(self)                         -> f64;
    fn from_f64(value: f64)                 -> Self;
}

impl TickerPrecision for f32 {
    const MIN_POSITIVE: Self                =   f32::MIN_POSITIVE;
    const MAX: Self                         =   f32::MAX;
    fn clamp(self, min: Self, max: Self)    ->  Self { self.clamp(min, max) }
    fn as_f64(self)                         ->  f64  { self as f64 }
    fn from_f64(value: f64)                 ->  Self { value as f32 }
}

impl TickerPrecision for f64 {
    const MIN_POSITIVE: Self                =   f64::MIN_POSITIVE;
    const MAX: Self                         =   f64::MAX;
    fn clamp(self, min: Self, max: Self)    ->  Self { self.clamp(min, max) }
    fn as_f64(self)                         ->  f64  { self }
    fn from_f64(value: f64)                 ->  Self { value }
}



/// Provides tickers with a variety of different behaviors.  Here is each behavior explained:
///
/// - **`Looper`**
///     - The ticker is **immutable** and will loop when current_value hits either start_value or end_value.  When a loop triggers, current_value is reset back to start_value.
///
///
/// - **`MutLooper`**
///     - The ticker is **mutable** and will loop when current_value hits either start_value or end_value.  When a loop triggers, current_value is reset back to start_value.
///
///
/// - **`Oneshot`**
///     - The ticker is **immutable** and will assign current_value to a boundary's value if current_value were to hit start_value or end_value; start and end values are the boundaries.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
///
///
/// - **`MutOneshot`**
///     - The ticker is **mutable** and will assign current_value to a boundary's value if current_value were to hit start_value or end_value; start and end values are the boundaries.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
///
///
/// - **`Freezing`**
///     - The ticker begins **mutable**, but it will become **immutable** once current_value hits end_value.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "ticker_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ticker_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub enum TickerBehaviors {
    Looper,
    MutLooper,
    Oneshot,
    MutOneshot,
    Freezing,
}



/// #### What Is A Ticker?
/// In short, a ticker is a type used to track the time between events.
///
/// In long, a ticker is a self-contained counter that advances a value (current_value) between two boundaries
/// (start_value and end_value) at a fixed rate, driven by however much time passes between
/// calls to .tick(). Depending on its behavior, a Ticker can loop back to start_value when
/// it reaches a boundary, stop at the boundary it hits, or become locked in place once it
/// reaches end_value.
///
/// "Time" here is intentionally generic — .tick() doesn't assume seconds, frames, or any
/// specific unit. It just compares whatever delta you feed .tick() against time_interval and
/// advances current_value accordingly, which is what lets the same ticker logic drive a
/// frame-timed cooldown, a world clock, or anything else that changes over some unit of "time".
///
/// A ticker's unit of time will be equal to the unit of the number you pass into the ticker's .tick() call(s).
///
/// ---
///
/// #### What Are The Fields of a Ticker And What Do They Do?
///
/// - **`start_value`**
///     - Represents the beginning value of a ticker and acts as one of the boundaries for current_value.
///     - Can be manipulated through addition and setter methods if the ticker is mutable.
///
/// - **`current_value`**
///     - Represents the value a ticker is currently at.
///     - current_value is bound to the range that start_value and end_value create.
///     - Ticking causes current_value to change, even if the ticker is immutable.
///     - Can be manipulated through addition and setter methods if the ticker is mutable.
///     - **For Looping Tickers**
///         - current_value will be set to start_value when a loop triggers.
///         - current_value hitting end_value will cause a loop to trigger.
///     - **For Oneshot Tickers**
///         - current_value hitting a boundary will assign current_value to be the boundary's value.
///     - **For Freezing Tickers**
///         - current_value hitting end_value will cause the ticker to become immutable.
///
/// - **`end_value`**
///     - Represents the ending value of a ticker and acts as one of the boundaries for current_value.
///     - Can be manipulated through addition and setter methods if the ticker is mutable.
///
/// - **`time_interval`**
///     - The amount of time that it takes for current_value to change by 1.
///     - Use this field to slow or speed up current_value's change.
///     - Can be manipulated through addition and setter methods if the ticker is mutable.
///
/// - **`stored_time`**
///     - Represents the remainder of time from the last .tick() call.
///     - Used by the .tick() method to keep the timing accurate.
///
/// - **`is_paused`**
///     - Represents whether a ticker is paused or not. A paused ticker prevents .tick() calls from doing anything.
///     - Can be manipulated through a setter method if the ticker is mutable.
///
/// - **`is_ticking_up`**
///     - Represents the tick direction of a ticker.
///     - Ticking up means that current_value will increase by 1 when the value of time stored in time_interval passes.
///     - Ticking down means that current_value will decrease by 1 when the value of time stored in time_interval passes.
///     - Can be manipulated through a setter method if the ticker is mutable.
///
/// - **`is_handling_time_spikes`**
///     - **If True**
///         - Will make it so that .tick() calls on a ticker are to add or subtract all built-up integer time since
///         the last .tick() call to current_value; addition/subtraction is dependent on is_ticking_up.
///         Any floating remainder gets put into stored_time for the next .tick() call.
///     - **If False**
///         - Will make it so that .tick() calls on a ticker are to add or subtract 1 to current_value;
///         addition/subtraction is dependent on is_ticking_up.
///     - Can be manipulated through a setter method if the ticker is mutable.
///
/// - **`behavior`**
///     - Dictates the type of behavior a ticker is currently set to.
///     - Can be used to stop a ticker from looping, or to start a ticker to loop.
///     - Can be used to change the mutability of a ticker.
///     - Can be manipulated through a setter method if the ticker is mutable **or** immutable.
///
/// ---
///
/// #### What Are the Different Behaviors a Ticker Can Have?
///
/// - **`Looper`**
///     - The ticker is **immutable** and will loop when current_value hits either start_value or end_value.  When a loop triggers, current_value is reset back to start_value.
///
///
/// - **`MutLooper`**
///     - The ticker is **mutable** and will loop when current_value hits either start_value or end_value.  When a loop triggers, current_value is reset back to start_value.
///
///
/// - **`Oneshot`**
///     - The ticker is **immutable** and will assign current_value to a boundary's value if current_value were to hit start_value or end_value; start and end values are the boundaries.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
///
///
/// - **`MutOneshot`**
///     - The ticker is **mutable** and will assign current_value to a boundary's value if current_value were to hit start_value or end_value; start and end values are the boundaries.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
///
///
/// - **`Freezing`**
///     - The ticker begins **mutable**, but it will become **immutable** once current_value hits end_value.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
///
/// ---
///
/// #### What Exactly is Changeable in Tickers?
/// - **`Ticker is Mutable`**
///     - Every field besides stored_time can be manipulated directly.  stored_time can be changed indirectly through the .hard_reset() method.
/// - **`Ticker is Immutable`**
///     - The only field that can be changed is behavior; this does mean that you can flip immutable tickers to mutable ones.
/// - **`Ticker is Mutable or Immutable`**
///     - current_value and stored_time will be changed if a ticker is ticking.  How and when these fields change is based on the ticker's boolean fields, what behavior the ticker is set to, and when .tick() gets called.
///
/// ---
///
/// #### What Are the Different Ticker Datatypes?
/// - **`Ticker<i8, f32>`** : 91+ Bits
/// - **`Ticker<i16, f32>`** : 115+ Bits
/// - **`Ticker<i32, f32>`** : 163+ Bits
/// - **`Ticker<i8, f64>`** : 155+ Bits
/// - **`Ticker<i16, f64>`** : 179+ Bits
/// - **`Ticker<i32, f64>`** : 227+ Bits
///
/// The "+" in the bit count is to recognize that the behavior field holds an enum value, and I have no idea
/// how many bits an enum declaration represents.
///
/// ---
///
/// #### How Do I Make A Ticker Tick?
/// Call the .tick() method on a ticker and pass in the time between 2 events, ticking is *usually* for events
/// that happen both consistently and constantly (such as when frames render); the .tick() method does have the
/// ability to take in **any** delta time.
///
/// Due to the complexity of the .tick() method, it can not be properly summarized here. Go read its
/// documentation if you'd like to know more about the method.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "ticker_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ticker_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub struct Ticker<V: TickerValue, P: TickerPrecision> {
    start_value:                V,
    current_value:              V,
    end_value:                  V,
    time_interval:              P,
    stored_time:                P,
    is_paused:                  bool,
    is_ticking_up:              bool,
    is_handling_time_spikes:    bool,
    behavior:                   TickerBehaviors,
}

impl<V: TickerValue, P: TickerPrecision> Default for Ticker<V, P> {

    /// Creates a MutLooper ticker that has the following properties:
    /// - start_value is set to 0.
    /// - Ticks `current_value` from 0 to the ticker's max integer value ([`V::MAX`]).
    /// - end_value is set to the ticker's max integer value ([`V::MAX`]).
    /// - time interval is set to 1.0.
    /// - Begins unpaused.
    /// - Will tick up.
    /// - Will handle time spikes.
    ///
    /// #### What is the Behavior of a MutLooper Ticker?
    /// The ticker is **mutable** and will loop when `current_value` hits either `start_value` or `end_value`.
    /// When a loop triggers, `current_value` is reset back to `start_value`.
    fn default() -> Self {
        Self {
            start_value:                V::from_i32(0),
            current_value:              V::from_i32(0),
            end_value:                  V::MAX,
            time_interval:              P::from_f64(1.0),
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up:              true,
            is_handling_time_spikes:    true,
            behavior:                   TickerBehaviors::MutLooper,
        }
    }
}

impl<V: TickerValue, P: TickerPrecision> Ticker<V, P> {

    // ##################################### CONSTRUCTORS ######################################## //
    /// Used for completely defining a custom ticker.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new(0, 10, 100, 1.0, false, true, true, TickerBehaviors::MutLooper);
    /// assert_eq!(ticker.behavior(), TickerBehaviors::MutLooper);
    /// ```
    pub fn new(
        start_value:                V,
        current_value:              V,
        end_value:                  V,
        time_interval:              P,
        is_paused:                  bool,
        is_ticking_up:              bool,
        is_handling_time_spikes:    bool,
        behavior:                   TickerBehaviors,
    ) -> Self {

        let min = start_value.min(end_value);
        let max = start_value.max(end_value);

        // Panic Evaluators
        check_if_value_is_within_range(start_value, V::MIN, V::MAX);
        check_if_value_is_within_range(current_value, min, max);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value,
            current_value,
            end_value,
            time_interval,
            stored_time: P::from_f64(0.0),
            is_paused,
            is_ticking_up,
            is_handling_time_spikes,
            behavior,
        }
    }

    /// Creates an unpaused Looper ticker that ticks `current_value` from the supplied `starting_value` to the
    /// passed `end_value`.
    ///
    /// #### What is the Behavior of a Looper Ticker?
    /// The ticker is **immutable** and will loop when `current_value` hits either `start_value` or `end_value`.
    /// When a loop triggers, `current_value` is reset back to `start_value`.
    ///
    /// #### What Is the Tick Direction If My Initial start_value and end_value Are Equal?
    /// Up.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new_looper(0, 100, 1.0, true);
    /// assert!(ticker.is_ticking_up());
    /// assert_eq!(ticker.behavior(), TickerBehaviors::Looper);
    /// ```
    pub fn new_looper(
        starting_value:             V,
        end_value:                  V,
        time_interval:              P,
        is_handling_time_spikes:    bool,
    ) -> Self {

        // Panic Evaluators
        check_if_value_is_within_range(starting_value, V::MIN, V::MAX);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up:              starting_value <= end_value,
            is_handling_time_spikes,
            behavior:                   TickerBehaviors::Looper,
        }
    }

    /// Creates an unpaused Looper ticker.
    ///
    /// #### What is the Behavior of a Looper Ticker?
    /// The ticker is **immutable** and will loop when `current_value` hits either `start_value` or `end_value`.
    /// When a loop triggers, `current_value` is reset back to `start_value`.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new_looper_custom(0, 25, 100, 1.0, true, true);
    /// assert_eq!(ticker.behavior(), TickerBehaviors::Looper);
    /// ```
    pub fn new_looper_custom(
        start_value:                V,
        current_value:              V,
        end_value:                  V,
        time_interval:              P,
        is_ticking_up:              bool,
        is_handling_time_spikes:    bool,
    ) -> Self {

        let min = start_value.min(end_value);
        let max = start_value.max(end_value);

        // Panic Evaluators
        check_if_value_is_within_range(start_value, V::MIN, V::MAX);
        check_if_value_is_within_range(current_value, min, max);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value,
            current_value,
            end_value,
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up,
            is_handling_time_spikes,
            behavior:                   TickerBehaviors::Looper,
        }
    }

    /// Creates an unpaused MutLooper ticker that ticks `current_value` from the supplied `starting_value` to the
    /// passed `end_value`.
    ///
    /// #### What is the Behavior of a MutLooper Ticker?
    /// The ticker is **mutable** and will loop when `current_value` hits either `start_value` or `end_value`.
    /// When a loop triggers, `current_value` is reset back to `start_value`.
    ///
    /// #### What Is the Tick Direction If My Initial start_value and end_value Are Equal?
    /// Up.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper(10, 0, 1.0, true);
    /// assert!(ticker.is_ticking_down());
    /// assert_eq!(ticker.behavior(), TickerBehaviors::MutLooper);
    /// ```
    pub fn new_mut_looper(
        starting_value:             V,
        end_value:                  V,
        time_interval:              P,
        is_handling_time_spikes:    bool,
    ) -> Self {

        // Panic Evaluators
        check_if_value_is_within_range(starting_value, V::MIN, V::MAX);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up:              starting_value <= end_value,
            is_handling_time_spikes,
            behavior:                   TickerBehaviors::MutLooper,
        }
    }

    /// Creates an unpaused MutLooper ticker.
    ///
    /// #### What is the Behavior of a MutLooper Ticker?
    /// The ticker is **mutable** and will loop when `current_value` hits either `start_value` or `end_value`.
    /// When a loop triggers, `current_value` is reset back to `start_value`.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 50, 100, 1.0, true, true);
    /// assert_eq!(ticker.behavior(), TickerBehaviors::MutLooper);
    /// ```
    pub fn new_mut_looper_custom(
        start_value:                V,
        current_value:              V,
        end_value:                  V,
        time_interval:              P,
        is_ticking_up:              bool,
        is_handling_time_spikes:    bool,
    ) -> Self {

        let min = start_value.min(end_value);
        let max = start_value.max(end_value);

        // Panic Evaluators
        check_if_value_is_within_range(start_value, V::MIN, V::MAX);
        check_if_value_is_within_range(current_value, min, max);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value,
            current_value,
            end_value,
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up,
            is_handling_time_spikes,
            behavior:                   TickerBehaviors::MutLooper,
        }
    }

    /// Creates an unpaused Oneshot ticker that ticks `current_value` from the supplied `starting_value` to the
    /// passed `end_value`.
    ///
    /// #### What is the Behavior of a Oneshot Ticker?
    /// The ticker is **immutable** and will assign `current_value` to a boundary's value if `current_value` were to hit `start_value` or `end_value`; start and end values are the boundaries.
    ///
    /// Additionally, the ticker's `stored_time` is set to 0.0 when `current_value` hits `end_value`.  This ensures the time state is completely reset once it reaches the end.
    ///
    /// #### What Is the Tick Direction If My Initial start_value and end_value Are Equal?
    /// Up.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new_oneshot(0, 10, 1.0, true);
    /// assert_eq!(ticker.behavior(), TickerBehaviors::Oneshot);
    /// ```
    pub fn new_oneshot(
        starting_value:             V,
        end_value:                  V,
        time_interval:              P,
        is_handling_time_spikes:    bool,
    ) -> Self {

        // Panic Evaluators
        check_if_value_is_within_range(starting_value, V::MIN, V::MAX);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up:              starting_value <= end_value,
            is_handling_time_spikes,
            behavior:                   TickerBehaviors::Oneshot,
        }
    }

    /// Creates an unpaused Oneshot ticker.
    ///
    /// #### What is the Behavior of a Oneshot Ticker?
    /// The ticker is **immutable** and will assign `current_value` to a boundary's value if `current_value` were to hit `start_value` or `end_value`; start and end values are the boundaries.
    ///
    /// Additionally, the ticker's `stored_time` is set to 0.0 when `current_value` hits `end_value`.  This ensures the time state is completely reset once it reaches the end.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new_oneshot_custom(0, 5, 10, 1.0, true, true);
    /// assert_eq!(ticker.behavior(), TickerBehaviors::Oneshot);
    /// ```
    pub fn new_oneshot_custom(
        start_value:                V,
        current_value:              V,
        end_value:                  V,
        time_interval:              P,
        is_ticking_up:              bool,
        is_handling_time_spikes:    bool,
    ) -> Self {

        let min = start_value.min(end_value);
        let max = start_value.max(end_value);

        // Panic Evaluators
        check_if_value_is_within_range(start_value, V::MIN, V::MAX);
        check_if_value_is_within_range(current_value, min, max);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value,
            current_value,
            end_value,
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up,
            is_handling_time_spikes,
            behavior:                   TickerBehaviors::Oneshot,
        }
    }

    /// Creates an unpaused MutOneshot ticker that ticks `current_value` from the supplied `starting_value` to the
    /// passed `end_value`.
    ///
    /// #### What is the Behavior of a MutOneshot Ticker?
    /// The ticker is **mutable** and will assign `current_value` to a boundary's value if `current_value` were to hit `start_value` or `end_value`; start and end values are the boundaries.
    ///
    /// Additionally, the ticker's `stored_time` is set to 0.0 when `current_value` hits `end_value`.  This ensures the time state is completely reset once it reaches the end.
    ///
    /// #### What Is the Tick Direction If My Initial start_value and end_value Are Equal?
    /// Up.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_oneshot(0, 100, 2.0, false);
    /// assert_eq!(ticker.behavior(), TickerBehaviors::MutOneshot);
    /// ```
    pub fn new_mut_oneshot(
        starting_value:             V,
        end_value:                  V,
        time_interval:              P,
        is_handling_time_spikes:    bool,
    ) -> Self {

        // Panic Evaluators
        check_if_value_is_within_range(starting_value, V::MIN, V::MAX);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up:              starting_value <= end_value,
            is_handling_time_spikes,
            behavior:                   TickerBehaviors::MutOneshot,
        }
    }

    /// Creates an unpaused MutOneshot ticker.
    ///
    /// #### What is the Behavior of a MutOneshot Ticker?
    /// The ticker is **mutable** and will assign `current_value` to a boundary's value if `current_value` were to hit `start_value` or `end_value`; start and end values are the boundaries.
    ///
    /// Additionally, the ticker's `stored_time` is set to 0.0 when `current_value` hits `end_value`.  This ensures the time state is completely reset once it reaches the end.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_oneshot_custom(10, 20, 30, 1.0, true, true);
    /// assert_eq!(ticker.behavior(), TickerBehaviors::MutOneshot);
    /// ```
    pub fn new_mut_oneshot_custom(
        start_value:                V,
        current_value:              V,
        end_value:                  V,
        time_interval:              P,
        is_ticking_up:              bool,
        is_handling_time_spikes:    bool,
    ) -> Self {

        let min = start_value.min(end_value);
        let max = start_value.max(end_value);

        // Panic Evaluators
        check_if_value_is_within_range(start_value, V::MIN, V::MAX);
        check_if_value_is_within_range(current_value, min, max);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value,
            current_value,
            end_value,
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up,
            is_handling_time_spikes,
            behavior:                   TickerBehaviors::MutOneshot,
        }
    }

    /// Creates an unpaused Freezing ticker that ticks `current_value` from the supplied `starting_value` to the
    /// passed `end_value`.
    ///
    /// #### What is the Behavior of a Freezing Ticker?
    /// The ticker begins **mutable**, but it will become **immutable** once `current_value` hits `end_value`.
    ///
    /// Additionally, the ticker's `stored_time` is set to 0.0 when `current_value` hits `end_value`.  This ensures the time state is completely reset once it reaches the end.
    ///
    /// #### What Is the Tick Direction If My Initial start_value and end_value Are Equal?
    /// Up.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new_freezing(0, 100, 1.0, true);
    /// assert_eq!(ticker.behavior(), TickerBehaviors::Freezing);
    /// ```
    pub fn new_freezing(
        starting_value:             V,
        end_value:                  V,
        time_interval:              P,
        is_handling_time_spikes:    bool,
    ) -> Self {

        // Panic Evaluators
        check_if_value_is_within_range(starting_value, V::MIN, V::MAX);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up:              starting_value <= end_value,
            is_handling_time_spikes,
            behavior:                   TickerBehaviors::Freezing,
        }
    }

    /// Creates an unpaused Freezing ticker.
    ///
    /// #### What is the Behavior of a Freezing Ticker?
    /// The ticker begins **mutable**, but it will become **immutable** once `current_value` hits `end_value`.
    ///
    /// Additionally, the ticker's `stored_time` is set to 0.0 when `current_value` hits `end_value`.  This ensures the time state is completely reset once it reaches the end.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new_freezing_custom(0, 0, 10, 1.0, true, true);
    /// assert_eq!(ticker.behavior(), TickerBehaviors::Freezing);
    /// ```
    pub fn new_freezing_custom(
        start_value:                V,
        current_value:              V,
        end_value:                  V,
        time_interval:              P,
        is_ticking_up:              bool,
        is_handling_time_spikes:    bool,
    ) -> Self {

        let min = start_value.min(end_value);
        let max = start_value.max(end_value);

        // Panic Evaluators
        check_if_value_is_within_range(start_value, V::MIN, V::MAX);
        check_if_value_is_within_range(current_value, min, max);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value,
            current_value,
            end_value,
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up,
            is_handling_time_spikes,
            behavior:                   TickerBehaviors::Freezing,
        }
    }
    // ######################################################################################## //



    // ##################################### GETTERS ########################################## //
    /// Returns the start_value of a ticker.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(5, 5, 10, 1.0, true, true);
    /// assert_eq!(ticker.start_value(), 5);
    /// ```
    #[inline]
    pub fn start_value(&self) -> V {
        self.start_value
    }

    /// Returns the current_value of a Ticker.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 7, 10, 1.0, true, true);
    /// assert_eq!(ticker.current_value(), 7);
    /// ```
    #[inline]
    pub fn current_value(&self) -> V {
        self.current_value
    }

    /// Returns the end_value of a Ticker.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 20, 1.0, true, true);
    /// assert_eq!(ticker.end_value(), 20);
    /// ```
    #[inline]
    pub fn end_value(&self) -> V {
        self.end_value
    }

    /// Returns the time_interval of a Ticker.
    ///
    /// #### What Exactly is time_interval?
    /// The time_interval is what dictates how long in \[INSERT_TIME_UNIT_HERE\] that it takes for
    /// current_value to increase or decrease by 1; direction depends on `is_ticking_up`.
    ///
    /// #### Unit Type of time_interval?
    /// ticker has no built-in concept of "seconds" or any other unit — time_interval and stored_time
    /// are just two numbers compared against each other inside the .tick() method. The unit they represent is
    /// determined entirely by whatever unit they pass into the .tick() method.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 2.5, true, true);
    /// assert_eq!(ticker.time_interval(), 2.5);
    /// ```
    #[inline]
    pub fn time_interval(&self) -> P {
        self.time_interval
    }

    /// Returns the stored_time of a Ticker.
    ///
    /// #### When Should I Use This Method?
    /// Realistically speaking, this method has limited use in most cases — stored_time holds
    /// only the leftover remainder from the last call to .tick(), not the total elapsed time
    /// since the ticker was created or last reset.  It exists mainly for debugging, logging, or
    /// custom structures that need to inspect or manually carry over a Ticker's in-progress
    /// timing state.
    ///
    /// #### Unit Type of stored_time?
    /// ticker has no built-in concept of "seconds" or any other unit — time_interval and stored_time
    /// are just two numbers compared against each other inside the .tick() method. The unit they represent is
    /// determined entirely by whatever unit they pass into the .tick() method.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert_eq!(ticker.stored_time(), 0.0);
    /// ```
    #[inline]
    pub fn stored_time(&self) -> P {
        self.stored_time
    }

    /// Returns true if the ticker is paused, false otherwise.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert!(!ticker.is_paused());
    /// ticker.pause();
    /// assert!(ticker.is_paused());
    /// ```
    #[inline]
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Returns true if a ticker is set to tick its `current_value` up, false otherwise.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert!(ticker.is_ticking_up());
    /// ```
    #[inline]
    pub fn is_ticking_up(&self) -> bool {
        self.is_ticking_up
    }

    /// Returns true if a ticker is set to tick its `current_value` down, false otherwise.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(10, 10, 0, 1.0, false, true);
    /// assert!(ticker.is_ticking_down());
    /// ```
    #[inline]
    pub fn is_ticking_down(&self) -> bool {
        !self.is_ticking_up
    }

    /// Returns true if the ticker can fire more than once in a single .tick() call, false otherwise.
    ///
    /// - `TRUE`: The ticker can change `current_value` by any number greater than or equal to 0 per
    /// .tick() call; the ticker will catch up all at once.
    ///
    /// - `FALSE`: The ticker can `change current_value` by 1 or 0 per .tick() call, no matter how much
    /// time has built up between .tick() calls.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert!(ticker.is_handling_time_spikes());
    /// ```
    #[inline]
    pub fn is_handling_time_spikes(&self) -> bool {
        self.is_handling_time_spikes
    }

    /// Returns the behavior type of the ticker.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper(0, 10, 1.0, true);
    /// assert_eq!(ticker.behavior(), TickerBehaviors::MutLooper);
    /// ```
    #[inline]
    pub fn behavior(&self) -> TickerBehaviors {
        self.behavior
    }
    // ######################################################################################## //



    // ##################################### SETTERS ########################################## //
    /// Changes `start_value` to the passed value.
    ///
    ///
    /// #### What Happens If Setting start_value Pushes current_value Out of Bounds?
    /// If the new `start_value` shifts the valid range such that `current_value` is left outside
    /// the boundaries, `current_value` is automatically clamped to the nearest valid edge.
    ///
    ///
    /// #### Important
    /// `start_value` can NOT go out of the range of `V::MIN` to `V::MAX`.
    /// Attempting to set `start_value` outside the range will cause it to be clamped down.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 20, 100, 1.0, true, true);
    /// ticker.set_start_value(40);
    ///
    /// assert_eq!(ticker.start_value(), 40);
    /// assert_eq!(ticker.current_value(), 40); // Clamped from 20 up to the new start_value 40
    /// ```
    pub fn set_start_value(&mut self, value: V) {
        if self.is_mutable() {
            // 1. Set and clamp the new start value.
            self.start_value = value.clamp(V::MIN, V::MAX);

            // 2. Identify the minimum and maximum boundaries between start and end.
            let min_boundary = self.start_value.min(self.end_value);
            let max_boundary = self.start_value.max(self.end_value);

            // 3. Clamp current_value to stay within the updated boundaries.
            self.current_value = self.current_value.clamp(min_boundary, max_boundary);
        }
    }

    /// Changes `current_value` to the passed value.
    ///
    /// #### Important
    /// `current_value` can NOT go out of the range that `start_value` and `end_value` create.
    /// Attempting to set `current_value` outside the range will cause it to be clamped down.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// ticker.set_current_value(5);
    /// assert_eq!(ticker.current_value(), 5);
    /// ```
    #[inline]
    pub fn set_current_value(&mut self, value: V) {
        if self.is_mutable() {
            let min = self.start_value.min(self.end_value);
            let max = self.start_value.max(self.end_value);
            self.current_value = value.clamp(min, max);
        }
    }

    /// Changes `end_value` to the passed value.
    ///
    /// #### What Happens If Setting end_value Pushes current_value Out of Bounds?
    /// If the new `end_value` shifts the valid range such that `current_value` is left outside
    /// the boundaries, `current_value` is automatically clamped to the nearest valid edge.
    ///
    /// #### Important
    /// `end_value` can NOT go out of the range of `V::MIN` to `V::MAX`.
    /// Attempting to set `end_value` outside the range will cause it to be clamped down.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 80, 100, 1.0, true, true);
    /// ticker.set_end_value(50);
    ///
    /// assert_eq!(ticker.end_value(), 50);
    /// assert_eq!(ticker.current_value(), 50); // Clamped from 80 down to the new end_value 50
    /// ```
    pub fn set_end_value(&mut self, value: V) {
        if self.is_mutable() {
            // 1. Set and clamp the new end value.
            self.end_value = value.clamp(V::MIN, V::MAX);

            // 2. Identify the minimum and maximum boundaries between start and end.
            let min_boundary = self.start_value.min(self.end_value);
            let max_boundary = self.start_value.max(self.end_value);

            // 3. Clamp current_value to stay within the updated boundaries.
            self.current_value = self.current_value.clamp(min_boundary, max_boundary);
        }
    }

    /// Prevents .tick() calls on a ticker from doing their job.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// ticker.pause();
    /// assert!(ticker.is_paused());
    /// ```
    #[inline]
    pub fn pause(&mut self) {
        if self.is_mutable() {
            self.is_paused = true;
        }
    }

    /// Allows .tick() calls on a ticker to do their job.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// ticker.pause();
    /// ticker.unpause();
    /// assert!(!ticker.is_paused());
    /// ```
    #[inline]
    pub fn unpause(&mut self) {
        if self.is_mutable() {
            self.is_paused = false;
        }
    }

    /// Causes the ticker's current_value to count up.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(5, 5, 0, 1.0, false, true);
    /// assert!(ticker.is_ticking_down());
    /// ticker.tick_up();
    /// assert!(ticker.is_ticking_up());
    /// ```
    #[inline]
    pub fn tick_up(&mut self) {
        if self.is_mutable() {
            self.is_ticking_up = true;
        }
    }

    /// Causes the ticker's current_value to count down.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert!(ticker.is_ticking_up());
    /// ticker.tick_down();
    /// assert!(ticker.is_ticking_down());
    /// ```
    #[inline]
    pub fn tick_down(&mut self) {
        if self.is_mutable() {
            self.is_ticking_up = false;
        }
    }

    /// Will make it so that .tick() calls on a ticker are to add or subtract all built-up integer time since
    /// the last .tick() call to `current_value`; addition/subtraction is dependent on `is_ticking_up`.
    /// Any floating remainder gets put into stored_time for the next .tick() call.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, false);
    /// assert!(!ticker.is_handling_time_spikes());
    /// ticker.start_handling_time_spikes();
    /// assert!(ticker.is_handling_time_spikes());
    /// ```
    #[inline]
    pub fn start_handling_time_spikes(&mut self) {
        if self.is_mutable() {
            self.is_handling_time_spikes = true;
        }
    }

    /// Will make it so that .tick() calls on a ticker are to add or subtract 1 to `current_value`;
    /// addition/subtraction is dependent on `is_ticking_up`.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert!(ticker.is_handling_time_spikes());
    /// ticker.stop_handling_time_spikes();
    /// assert!(!ticker.is_handling_time_spikes());
    /// ```
    #[inline]
    pub fn stop_handling_time_spikes(&mut self) {
        if self.is_mutable() {
            self.is_handling_time_spikes = false;
        }
    }

    /// Switches the behavior of a ticker to the passed TickerBehavior type.
    ///
    /// #### Important Aspects of This Setter
    /// - Can be used to switch the mutability of a Ticker.
    /// - Can be used to turn on/off the looping behavior of a Ticker.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::{Ticker, TickerBehaviors};
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// ticker.set_behavior(TickerBehaviors::Freezing);
    /// assert_eq!(ticker.behavior(), TickerBehaviors::Freezing);
    /// ```
    #[inline]
    pub fn set_behavior(&mut self, new_behavior: TickerBehaviors) {
        self.behavior = new_behavior;
    }
    // ######################################################################################## //



    // ################################### EQUALITY METHODS ##################################### //
    /// Returns true if the `current_value` and the `start_value` are equal to one another, false otherwise.
    ///
    /// # When Should I Use This Method?
    /// Use this method in oneshot tickers that count to `start_value` if you want to determine if the
    /// oneshot is finished.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert!(ticker.is_current_at_start());
    /// ```
    #[inline]
    pub fn is_current_at_start(&self) -> bool {
        self.current_value == self.start_value
    }

    /// Returns true if the `current_value` and the `end_value` are equal to one another, false otherwise.
    ///
    /// # When Should I Use This Method?
    /// Use this method in oneshot tickers that count to `end_value` if you want to determine if the
    /// oneshot is finished.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 10, 10, 1.0, true, true);
    /// assert!(ticker.is_current_at_end());
    /// ```
    #[inline]
    pub fn is_current_at_end(&self) -> bool {
        self.current_value == self.end_value
    }

    /// Returns true if the `start_value` and the `end_value` are equal to one another, false otherwise.
    ///
    /// # Why Does This Method Exist?
    /// `start_value` and `end_value` can equal one another since their values can be changed or set to
    /// the same value at the creation of a ticker instance.
    ///
    /// # When Should I Use This Method?
    /// Only scenario I can think for using this would be when the bounds of a ticker are slowly tightening
    /// and you need something to check when they have finally met one another.  It is possible to tighten
    /// the bounds by constantly setting `start_value` and `end_value` to new numbers.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(10, 10, 10, 1.0, true, true);
    /// assert!(ticker.is_start_at_end());
    /// ```
    #[inline]
    pub fn is_start_at_end(&self) -> bool {
        self.start_value == self.end_value
    }
    // ######################################################################################## //



    // ################################# DIFFERENCE METHODS ################################### //
    /// Returns the difference between `current_value` and `start_value`.
    ///
    /// Will only return positive numbers, including 0.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 35, 100, 1.0, true, true);
    /// assert_eq!(ticker.difference_from_start(), 35);
    /// ```
    pub fn difference_from_start(&self) -> i64 {
        let min: i64 = self.current_value.min(self.start_value).as_i64();
        let max: i64 = self.current_value.max(self.start_value).as_i64();
        max - min
    }

    /// Returns the difference between `current_value` and `end_value`.
    ///
    /// Will only return positive numbers, including 0.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 35, 100, 1.0, true, true);
    /// assert_eq!(ticker.difference_from_end(), 65);
    /// ```
    pub fn difference_from_end(&self) -> i64 {
        let min: i64 = self.current_value.min(self.end_value).as_i64();
        let max: i64 = self.current_value.max(self.end_value).as_i64();
        max - min
    }

    /// Returns the difference between `start_value` and `end_value`.
    ///
    /// Will only return positive numbers, including 0.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(10, 35, 100, 1.0, true, true);
    /// assert_eq!(ticker.difference_from_start_to_end(), 90);
    /// ```
    pub fn difference_from_start_to_end(&self) -> i64 {
        let min: i64 = self.start_value.min(self.end_value).as_i64();
        let max: i64 = self.start_value.max(self.end_value).as_i64();
        max - min
    }
    // ######################################################################################## //



    // ################################# DIGIT METHODS ######################################## //
    /// Returns the digit in the ones-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// #### Small Note
    /// It is NOT possible for the ones digit to be dropped, hence the reason why this method has no
    /// digit-drop accounting version - there is always a ones-place.
    ///
    /// #### No Conditional in Implementation?
    /// This digit method does not need to check if current_value is holding a value that contains this digit
    /// since all integer types can contain at least 3 digits.  0 will still be returned if the digit isn't being used.
    #[inline]
    pub fn digit_1(&self) -> i8 {
        (self.current_value.absolute() % V::from_i32(10)).as_i8()
    }

    /// Returns the digit in the tens-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    ///
    /// #### No Conditional in Implementation?
    /// This digit method does not need to check if current_value is holding a value that contains this digit
    /// since all integer types can contain at least 3 digits.  0 will still be returned if the digit isn't being used.
    #[inline]
    pub fn digit_2(&self) -> i8 {
        ((self.current_value.absolute() / V::from_i32(10)) % V::from_i32(10)).as_i8()
    }

    /// Returns the digit in the hundreds-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    ///
    /// #### No Conditional in Implementation?
    /// This digit method does not need to check if current_value is holding a value that contains this digit
    /// since all integer types can contain at least 3 digits.  0 will still be returned if the digit isn't being used.
    #[inline]
    pub fn digit_3(&self) -> i8 {
        ((self.current_value.absolute() / V::from_i32(100)) % V::from_i32(10)).as_i8()
    }

    /// Returns the digit in the thousands-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_4(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(1_000) {
            ((self.current_value.absolute() / V::from_i32(1_000)) % V::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the ten-thousands-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_5(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(10_000) {
            ((self.current_value.absolute() / V::from_i32(10_000)) % V::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the hundred-thousands-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_6(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(100_000) {
            ((self.current_value.absolute() / V::from_i32(100_000)) % V::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the millions-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_7(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(1_000_000) {
            ((self.current_value.absolute() / V::from_i32(1_000_000)) % V::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the ten-millions-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_8(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(10_000_000) {
            ((self.current_value.absolute() / V::from_i32(10_000_000)) % V::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the hundred-millions-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_9(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(100_000_000) {
            ((self.current_value.absolute() / V::from_i32(100_000_000)) % V::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the tens-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// #### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// #### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_2_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(10) {
            ((self.current_value.absolute() / V::from_i32(10)) % V::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the hundreds-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// #### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// #### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_3_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(100) {
            ((self.current_value.absolute() / V::from_i32(100)) % V::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the thousands-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// #### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// #### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_4_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(1_000) {
            ((self.current_value.absolute() / V::from_i32(1_000)) % V::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the ten-thousands-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// #### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// #### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_5_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(10_000) {
            ((self.current_value.absolute() / V::from_i32(10_000)) % V::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the hundred-thousands-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// #### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// #### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_6_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(100_000) {
            ((self.current_value.absolute() / V::from_i32(100_000)) % V::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the millions-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// #### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// #### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_7_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(1_000_000) {
            ((self.current_value.absolute() / V::from_i32(1_000_000)) % V::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the ten-millions-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// #### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// #### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_8_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(10_000_000) {
            ((self.current_value.absolute() / V::from_i32(10_000_000)) % V::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the hundred-millions-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// #### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// #### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_9_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(100_000_000) {
            ((self.current_value.absolute() / V::from_i32(100_000_000)) % V::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }
    // ######################################################################################## //



    // ################################### ADD METHODS ######################################## //
    /// Adds to the `start_value` of the ticker by the passed value.  Can take in negatives for subtraction.
    /// Will not let the result of summing cause overflow or wrapping; results will always be within
    /// `V::MIN` to `V::MAX` (inclusive).
    ///
    /// #### What Happens If Adding To start_value Pushes current_value Out of Bounds?
    /// If the new `start_value` shifts the valid range such that `current_value` is left outside
    /// the boundaries, `current_value` is automatically clamped to the nearest valid edge.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// // Case 1: Increasing start_value shifts the lower bound up, clamping current_value
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 20, 100, 1.0, true, true);
    /// ticker.add_to_start_value(40); // New start_value becomes 40
    ///
    /// assert_eq!(ticker.start_value(), 40);
    /// assert_eq!(ticker.current_value(), 40); // Clamped from 20 up to 40
    ///
    /// // Case 2: Swapping directions where start > end
    /// let mut ticker_down = Ticker::<i32, f32>::new_mut_looper_custom(100, 90, 50, 1.0, false, true);
    /// ticker_down.add_to_start_value(-20); // New start_value becomes 80 (Range is now 80 down to 50)
    ///
    /// assert_eq!(ticker_down.start_value(), 80);
    /// assert_eq!(ticker_down.current_value(), 80); // Clamped from 90 down to 80
    /// ```
    pub fn add_to_start_value(&mut self, value: V) {
        if self.is_mutable() {
            // 1. Calculate the new start value safely.
            self.start_value = self.start_value.sat_add(value).clamp(V::MIN, V::MAX);

            // 2. Identify the minimum and maximum boundaries between start and end.
            let min_boundary = self.start_value.min(self.end_value);
            let max_boundary = self.start_value.max(self.end_value);

            // 3. Clamp current_value to stay within the updated boundaries.
            self.current_value = self.current_value.clamp(min_boundary, max_boundary);
        }
    }

    /// Adds to the `current_value` of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within `start_value` to `end_value` (inclusive).
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 40, 100, 1.0, true, true);
    /// ticker.add_to_current_value(15);
    /// assert_eq!(ticker.current_value(), 55);
    /// ```
    #[inline]
    pub fn add_to_current_value(&mut self, value: V) {
        if self.is_mutable() {
            let min = self.start_value.min(self.end_value);
            let max = self.start_value.max(self.end_value);
            self.current_value = self.current_value.sat_add(value).clamp(min, max);
        }
    }

    /// Adds to the `end_value` of the ticker by the passed value.  Can take in negatives for subtraction.
    /// Will not let the result of summing cause overflow or wrapping; results will always be within
    /// `V::MIN` to `V::MAX` (inclusive).
    ///
    /// #### What Happens If Adding To end_value Pushes current_value Out of Bounds?
    /// If the new `end_value` shifts the valid range such that `current_value` is left outside
    /// the boundaries, `current_value` is automatically clamped to the nearest valid edge.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// // Case 1: Shrinking the range pushes current_value out
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 80, 100, 1.0, true, true);
    /// ticker.add_to_end_value(-50); // New end_value becomes 50
    ///
    /// assert_eq!(ticker.end_value(), 50);
    /// assert_eq!(ticker.current_value(), 50); // Clamped from 80 down to 50
    ///
    /// // Case 2: Swapping directions where start > end
    /// let mut ticker_down = Ticker::<i32, f32>::new_mut_looper_custom(100, 55, 50, 1.0, false, true);
    /// ticker_down.add_to_end_value(10); // New end_value becomes 60 (Range is now 100 down to 60)
    /// assert_eq!(ticker_down.end_value(), 60);
    /// assert_eq!(ticker_down.current_value(), 60); // Clamped from 55 up to 60
    /// ```
    pub fn add_to_end_value(&mut self, value: V) {
        if self.is_mutable() {
            // 1. Calculate the new end value safely.
            self.end_value = self.end_value.sat_add(value).clamp(V::MIN, V::MAX);

            // 2. Identify the minimum and maximum boundaries between start and end.
            let min_boundary = self.start_value.min(self.end_value);
            let max_boundary = self.start_value.max(self.end_value);

            // 3. Clamp current_value to stay within the updated boundaries.
            self.current_value = self.current_value.clamp(min_boundary, max_boundary);
        }
    }

    /// Adds to the `time_interval` of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// #### What Values Can time_interval Be Set To?
    /// time_interval can never be 0, a negative number, or go past `P::MAX`; the reasoning for this is that it would cause the
    /// .tick() method to create crazy values.
    ///
    /// #### Can Interval Flip Direction of a Ticker?
    /// No. If your goal is to slow time or slow an accumulation to the point that it reverses it,
    /// I suggest you flip the tick direction using .tick_up() or .tick_down() at a specific current_value or
    /// after the rate of speed you're applying has hit a specific value.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper(0, 10, 1.0, true);
    ///
    /// // Add time to the interval
    /// ticker.add_to_time_interval(0.5);
    /// assert_eq!(ticker.time_interval(), 1.5);
    ///
    /// // Value cannot fall below or equal to 0.0, so subtracting too much will clamp it
    /// ticker.add_to_time_interval(-5.0);
    /// assert!(ticker.time_interval() > 0.0);
    /// ```
    #[inline]
    pub fn add_to_time_interval(&mut self, value: P) {
        if self.is_mutable() {
            self.time_interval = (self.time_interval + value).clamp(P::MIN_POSITIVE, P::MAX);
        }
    }
    // ########################################################################################## //



    // ################################### PERCENTAGE METHODS ################################### //
    /// Returns the exact percentage of completion from `start_value` to `end_value` as a floating point.
    ///
    /// A return value of `0.0` means `current_value` is at `start_value`, and `1.0` means it is
    /// at `end_value`.
    ///
    /// #### Special Case
    /// Returns `-1.0` if `start_value` and `end_value` are equal, as no meaningful range exists.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 40, 100, 1.0, true, true);
    ///
    /// // Since it deals with floats, tolerate tiny inaccuracies
    /// let percentage = ticker.percentage_completed();
    /// assert!((percentage - 0.4).abs() < f32::EPSILON);
    /// ```
    pub fn percentage_completed(&self) -> f32 {

        if self.start_value == self.end_value {
            return -1.0;
        }

        let start: f32 = self.start_value.as_f32();
        let current: f32 = self.current_value.as_f32();
        let end: f32 = self.end_value.as_f32();

        let range_reciprocal: f32 = 1.0 / (end - start);

        (current - start) * range_reciprocal
    }

    /// Returns the remaining percentage needed to reach `end_value` as a floating point.
    ///
    /// A return value of `0.0` means `current_value` is at `end_value`, and `1.0` means it is
    /// at `start_value`.
    ///
    /// #### Special Case
    /// Returns `-1.0` if `start_value` and `end_value` are equal, as no meaningful range exists.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 25, 100, 1.0, true, true);
    ///
    /// // Since it deals with floats, tolerate tiny inaccuracies
    /// let remaining = ticker.percentage_remaining();
    /// assert!((remaining - 0.75).abs() < f32::EPSILON);
    /// ```
    pub fn percentage_remaining(&self) -> f32 {

        if self.start_value == self.end_value {
            return -1.0;
        }

        let start: f32 = self.start_value.as_f32();
        let current: f32 = self.current_value.as_f32();
        let end: f32 = self.end_value.as_f32();

        let range_reciprocal: f32 = 1.0 / (end - start);

        (end - current) * range_reciprocal
    }
    // ########################################################################################## //



    // ##################################### RESET METHODS ###################################### //
    /// Resets `current_value` back to `start_value`.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 40, 100, 1.0, true, true);
    /// assert_eq!(ticker.current_value(), 40);
    ///
    /// ticker.soft_reset();
    /// assert_eq!(ticker.current_value(), 0);
    #[inline]
    pub fn soft_reset(&mut self) {
        if self.is_mutable() {
            self.current_value = self.start_value;
        }
    }

    /// Resets `current_value` back to `start_value`, and zeroes out `stored_time`.
    ///
    /// #### When To Use This Over Reset?
    /// Best to use when you want to completely wipe whatever has been accumulated, including the
    /// timing state.  If you need to carry over the timing state (the remainder in the last .tick()
    /// calculation) then do NOT use this.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 40, 100, 1.0, true, true);
    /// ticker.tick(1.3);
    /// ticker.hard_reset();
    ///
    /// assert_eq!(ticker.current_value(), 0);
    /// assert_eq!(ticker.stored_time(), 0.0);
    /// ```
    #[inline]
    pub fn hard_reset(&mut self) {
        if self.is_mutable() {
            self.current_value = self.start_value;
            self.stored_time = P::from_f64(0.0);
        }
    }
    // ########################################################################################## //



    // ########################### THE TICK (THE MOST IMPORTANT METHOD) ######################### //
    /// #### Description of .tick()
    /// Used to advance a ticker by taking in a passing of time between 2 events, *usually* for events
    /// that happen both consistently and constantly (such as when frames render); it does have the
    /// ability to take in **any** delta time.
    ///
    /// ---
    ///
    /// #### What The Hell Does Ticking Do?
    /// The simplified version (read the method's code for the complex version) of calling the .tick()
    /// method on a ticker is as follows:
    /// 1. Increase stored_time by the value passed to the .tick() call (elapsed_time_between_events).
    /// 2. If stored_time is greater than or equal to the time_interval, change current_value.
    /// 3. Reassign stored_time to the result of `stored_time %= time_interval`.  We do this to carry
    /// over our remainder to keep the state of a Ticker's time accurate to the events that are being
    /// tracked.
    ///
    /// ---
    ///
    /// #### What Impacts Ticking And How?
    /// The fields of a ticker which impact the calculations inside this method are as follows:
    /// - `is_paused`
    ///     - **True** : Prevent the tick method from doing anything.
    ///     - **False** : Tick method will determine if current_value needs to be changed.
    /// - `is_ticking_up`
    ///     - **True** : .tick() calls will increase current_value.
    ///     - **False** : .tick() calls will decrease current_value.
    /// - `is_handling_time_spikes`
    ///     - **True** : The full integer magnitude from the result of `elapsed_time + stored_time` will be used to change current_value.
    ///     - **False** : As long as stored_time is greater than or equal to the time_interval, current_value will change by 1.
    /// - `behavior`
    ///     - **Looper** : Reset current_value to start_value when a ticker boundary is hit; boundaries are start_value and end_value.
    ///     - **MutLooper** :  Reset current_value to start_value when a ticker boundary is hit; boundaries are start_value and end_value.
    ///     - **Oneshot** : current_value will be set to the boundary it hits or goes past.
    ///     - **MutOneshot** : current_value will be set to the boundary it hits or goes past.
    ///     - **Freezing** : current_value will be set to the boundary it hits or goes past.
    ///
    /// ---
    ///
    /// #### Does .tick() Impact a Ticker's Units?
    /// Yes.  The units for a Ticker's integers and float fields are based on what is passed into the
    /// .tick() method.  If the passed value represents the difference in seconds between 2 frames,
    /// the Ticker's time_interval and stored_time units would be seconds.  From there, the start_value,
    /// current_value, and end_value would be based on time_interval's value and unit.  For instance,
    /// if time_interval is set to 37.0 and .tick() took in seconds, then a 1 inside any of the value
    /// fields will be equal to 37 seconds; a 2 in current_value in this example would be 74 seconds.
    ///
    /// ---
    ///
    /// #### Example of .tick() in Action
    /// Consider the following factors first:
    /// - `The Horror You've Undergone` : You've seen 8 clowns between 2 blinks.  You decide to make a ticker to calculate this horror.
    /// - `Ticker's Starting time_interval Value` : 1.7
    /// - `Ticker's Starting stored_time Value` : 0.0
    /// - A .tick() call produced 4 for its magnitude_of_time_that_passed.
    /// - At the end of the .tick() call, stored_time is 0.705882353.
    /// - The ticker you're using to track all of this has is_handling_time_spikes set to true.
    ///
    /// Now, here's a step-by-step scenario involving our known factors:
    /// 1. You've undergone horror.
    /// 2. You throw in the 8 clowns to a .tick() call as it represents the time between your known events (blinks).
    /// 2. Your stored_time started at 0 and now adds up to 8 clowns seen.  Taking in these clowns has turned your Ticker's units to "clowns seen".
    ///     - The clowns seen is our time unit.  An equivalent to this would be seconds as a unit between frames being rendered.
    ///     - A blink is the event that tells us when to log things.  An equivalent to this would be a new frame being rendered.
    /// 3. Your time_interval is set to 1.7 clowns seen, which will mean that we only add 1 to current_value after we've seen 1.7 clowns.
    /// 4. Since is_handling_time_spikes is set to true, we take our 8 clowns and divide that by our time_interval of 1.7.  4.705882353 is the result.
    /// 5. .tick() will truncate 4.705882353 to 4 and the 0.705882353 will act as a remainder.
    /// 6. The remainder of 0.705882353 gets assigned to stored_time so that the next .tick() call can account for how many clowns were carried over from the last blink period.
    /// 7. The 4 is what gets added or subtracted from current_value depending on if is_ticking_up is set to true or false.
    /// 8. Voila. .tick() call is complete.
    pub fn tick(&mut self, elapsed_time_between_events: P) {

        // POTENTIAL RETURN
        // If paused, go no further as we don't need to calculate the new current_value since the ticker is frozen.
        if self.is_paused {
            return;
        }

        // TIME ACCUMULATION
        // Add to the stored_time so that we can later determine if we've gone over the time_interval value and need to fire another tick.
        self.stored_time += elapsed_time_between_events;

        // DETERMINING PASSED TIME
        // Acquiring the integer magnitude of time that passed between events.
        // How this is done is dependent on what is_handling_time_spikes is set to.
        let magnitude_of_time_that_passed = match self.is_handling_time_spikes {

            // PASSED TIME WHEN HANDLING TIME SPIKES
            // When time spike handling is active, the entire integer value of time that passed since
            // the last .tick() call will be used for magnitude_of_time_that_passed.  The floating
            // remainder after division is kept in stored_time so that partial progress toward the
            // next tick is not lost between the events that are being timed.
            //
            // as_64() in tick_count_truncated_to_value_type is acting as a bridge for the V and P generics
            // to work with one another.  It does mean that a typecast to f64 happens here, but the requested
            // precision is still maintained since the calculated magnitude_of_time_that_passed happened
            // inside the variable "magnitude_of_time_that_passed_in_active_precision".
            // After that the value gets truncated using V::from_f64 since all V types are integers.
            true => {
                let magnitude_of_time_that_passed_in_active_precision: P = self.stored_time / self.time_interval;
                let magnitude_of_time_that_passed_truncated_to_value_type: V = V::from_f64(magnitude_of_time_that_passed_in_active_precision.as_f64());
                self.stored_time %= self.time_interval; // Carrying remainder over to keep ticking accuracy.
                magnitude_of_time_that_passed_truncated_to_value_type
            },

            // PASSED TIME WHEN ~NOT~ HANDLING TIME SPIKES
            // When time spike handling is inactive, only 1 tick is allowed to fire per call
            // regardless of how large the elapsed_time_between_events was.  The value of time_interval
            // is subtracted from stored_time rather than resetting to zero so that the timer remains
            // accurate over continuous .tick() calls — any leftover time beyond the single tick carries
            // into the next .tick() call.
            //
            // We subtract by time_interval (rather than just discarding stored_time) specifically because
            // is_handling_time_spikes can be toggled at runtime.  If this flag is permanently false for
            // a given ticker, the leftover precision wouldn't matter — each call only ever checks "has
            // one time_interval passed, yes or no" regardless of how much excess sits in stored_time.  But
            // since the flag can flip to true later, preserving the leftover ensures that switch correctly
            // picks up every unit of time that was quietly accumulating while spike handling was off,
            // rather than discarding that history the moment is_handling_time_spikes gets re-enabled.
            false => match self.stored_time >= self.time_interval {
                true => {
                    self.stored_time -= self.time_interval;
                    V::from_i32(1)
                },
                false => V::from_i32(0),
            },
        };

        // DETERMINING IF CURRENT_VALUE SHOULD BE CHANGED
        // Will only ever change current_value if the stored_time pushed the magnitude_of_time_that_passed
        // beyond the time_interval value.  This check ensures we aren't needlessly adding to current_value
        // for every .tick() call; for specifically frame logic, this prevents changing current_value every frame.
        //
        // To be perfectly clear, magnitude_of_time_that_passed can only be greater than 0 if the stored_time went past the
        // time_interval value.  Greater than 0 means 1 or higher in this case, decimals in between 0 and 1
        // don't count.
        if magnitude_of_time_that_passed > V::from_i32(0) {

            // CURRENT_VALUE ADDITION OR SUBTRACTION?
            // Increase or decrease current_value's new host based on if the ticker is ticking up or down.
            let new_value = match self.is_ticking_up {
                true  => self.current_value.sat_add(magnitude_of_time_that_passed),
                false => self.current_value.sat_sub(magnitude_of_time_that_passed),
            };

            // DETERMINING CURRENT_VALUE'S BOUNDARIES
            // Since start_value and end_value can be either negative or positive at any given moment,
            // we must throw both values against one another to determine whose greater/lesser than
            // the other so that we can properly clamp down current_value to its allowed range.
            let min = self.start_value.min(self.end_value);
            let max = self.start_value.max(self.end_value);

            // RESET DETERMINATION + CURRENT_VALUE ASSIGNMENT
            // Will change current_value's assignment using new_value based on a ticker's behavior.
            match self.behavior {

                // LOOPER LOGIC
                // Assign current_value to its new host and then reset it to the ticker's start_value
                // if either of its boundaries - start_value and end_value - are hit.
                TickerBehaviors::Looper |
                TickerBehaviors::MutLooper => {
                    self.current_value = new_value;
                    if self.current_value <= min || self.current_value >= max {
                        self.current_value = self.start_value;
                    }
                },

                // ONESHOT + FREEZING LOGIC
                // current_value can assume its new host after new_value has been clamped to the allowed range.
                // Additionally, stored_time will be zeroed out if current_value hits end_value.  We
                // do this wipe for stored_time since oneshotters and freezings are purposed to clear their
                // time storage upon hitting their end destination.
                TickerBehaviors::Oneshot |
                TickerBehaviors::MutOneshot |
                TickerBehaviors::Freezing => {
                    self.current_value = new_value.clamp(min, max);
                    if self.current_value == self.end_value {
                        self.stored_time = P::from_f64(0.0);
                    }
                },
            };
        }
    }
    // ############################################################################################## //



    // ###################################### HELPER METHODS ######################################## //
    /// Returns true if the current behavior of the ticker is mutable, otherwise false.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_tickers::Ticker;
    ///
    /// let looper = Ticker::<i32, f32>::new_looper(0, 10, 1.0, true);
    /// assert!(!looper.is_mutable());
    ///
    /// let mut_looper = Ticker::<i32, f32>::new_mut_looper(0, 10, 1.0, true);
    /// assert!(mut_looper.is_mutable());
    /// ```
    #[inline]
    pub fn is_mutable(&self) -> bool {
        match self.behavior {
            TickerBehaviors::Looper     => false,
            TickerBehaviors::MutLooper  => true,
            TickerBehaviors::Oneshot    => false,
            TickerBehaviors::MutOneshot => true,
            TickerBehaviors::Freezing   => self.current_value != self.end_value,
        }
    }
    // ############################################################################################## //
}



/// Checks if a value falls within the provided minimum and maximum range (inclusive).
///
/// Accepts any type that implements [`PartialOrd`] and [`Display`], meaning
/// all numeric primitives, [`char`], [`String`], and [`&str`] are valid inputs.
///
/// #### Panics
/// Panics if the value is outside the provided range.
///
/// #### Example
/// ```ignore
/// check_if_value_is_within_range(5, 1, 10);    // Passes
/// check_if_value_is_within_range(15, 1, 10);   // Panics
/// ```
fn check_if_value_is_within_range<T: PartialOrd + Display>(value: T, minimum: T, maximum: T) {
    assert!(
        value >= minimum && value <= maximum,
        "{}[FAIL]{} Ticker value must be between {} and {} (inclusive). Got {}.",
        "\x1b[31m", "\x1b[0m", minimum, maximum, value
    );
}
