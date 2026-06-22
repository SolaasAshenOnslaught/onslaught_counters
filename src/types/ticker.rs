
// Imports
use bevy::prelude::*;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, Rem, RemAssign, Sub, SubAssign};
use mirth_engine_testing_tools::{check_if_value_is_within_range};

// ####################################### VALUE TRAIT ########################################## //
/// Used to apply a generic to the start_value, current_value, and end_value within the Ticker type.
///
/// Supports i8, i16, i32 for start_value, current_value, and end_value within Ticker.  Memory size
/// for the Ticker values is adjustable due to this trait.  9 times out of 10 you'll likely just need
/// i8 tickers.
///
/// # IMPORTANT
/// The MIN and MAX declarations are present to help avoid absolute errors on integer ranges.  MAX isn't
/// really impacted by this, it's declared for readability purposes.  But MIN's assignment on value types
/// will always add 1 to an integer's minimum to avoid things like -128 in the i8 datatype becoming 128
/// after .absolute() is applied to a value.  We have to do this since 128 is outside the i8 range;
/// 127 is the max for i8.
pub trait TickerValue:
Copy                    // TickerValue types are integers, which means they're safe to copy.
+ Ord                   // TickerValue types are integers, hence Ord is necessary for comparison.
+ Display               // There are checks (that can display) to determine if the values are within their acceptable ranges.
+ Add<Output = Self>
+ Sub<Output = Self>
+ Div<Output = Self>
+ Rem<Output = Self>
+ Send                  // BEVY REQUIREMENT: For querying to recognize the V generic.
+ Sync                  // BEVY REQUIREMENT: For querying to recognize the V generic.
+ 'static               // BEVY REQUIREMENT: TickerValue types are integers, values are valid at all times.
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
// ############################################################################################## //



// ####################################### PRECISION TRAIT ###################################### //
/// Used to apply a generic to the accrued_delta and interval fields within the Ticker type.
///
/// Supports f32 and f64 for accrued_delta and interval fields within Ticker.
///
/// # IMPORTANT
/// Using f64 for Ticker precision will result in more accurate calculations inside the .tick() method.
/// Useful if accuracy is crucial, otherwise pointless.  In most cases, f64 precision is not necessary.
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
+ Send                  // BEVY REQUIREMENT: For querying to recognize the P generic.
+ Sync                  // BEVY REQUIREMENT: For querying to recognize the P generic.
+ 'static               // BEVY REQUIREMENT: TickerPrecision types are floats, values are valid at all times.
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
// ############################################################################################## //



// ################################# TICKER IMPLEMENTATION ###################################### //
/// A generic, self-contained counter that advances a value over time at a fixed interval.
///
#[derive(Component, Reflect, Debug)]
pub struct Ticker<V: TickerValue, P: TickerPrecision> {
    start_value:                V,
    current_value:              V,
    end_value:                  V,
    interval:                   P,
    accrued_delta:              P,
    is_paused:                  bool,
    is_looping:                 bool,
    is_ticking_up:              bool,
    is_handling_frame_spikes:   bool,
}

impl<V: TickerValue, P: TickerPrecision> Default for Ticker<V, P> {
    ///
    fn default() -> Self {
        Self {
            start_value:                V::from_i32(0),
            current_value:              V::from_i32(0),
            end_value:                  V::from_i32(100),
            interval:                   P::from_f64(1.0),
            accrued_delta:              P::from_f64(0.0),
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up:              true,
            is_handling_frame_spikes:   true,
        }
    }
}

impl<V: TickerValue, P: TickerPrecision> Ticker<V, P> {

    // ##################################### CONSTRUCTORS ######################################## //
    /// Use this when you need to completely define your own Ticker; full-custom.
    ///
    /// # Important
    /// Text
    pub fn new(
        start_value: V,
        current_value: V,
        end_value: V,
        interval: P,
        is_paused: bool,
        is_looping: bool,
        is_ticking_up: bool,
        is_handling_frame_spikes: bool,
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
            interval,
            accrued_delta: P::from_f64(0.0),
            is_paused,
            is_looping,
            is_ticking_up,
            is_handling_frame_spikes,
        }
    }

    ///
    pub fn new_onetime_with_frame_spike_handling(
        starting_value: V,
        end_value: V,
        interval: P,
        is_ticking_up: bool,
    ) -> Self {

        // Panic Evaluators
        check_if_value_is_within_range(starting_value, V::MIN, V::MAX);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              P::from_f64(0.0),
            is_paused:                  false,
            is_looping:                 false,
            is_ticking_up,
            is_handling_frame_spikes:   true,
        }
    }

    ///
    pub fn new_onetime_without_frame_spike_handling(
        starting_value: V,
        end_value: V,
        interval: P,
        is_ticking_up: bool,
    ) -> Self {

        // Panic Evaluators
        check_if_value_is_within_range(starting_value, V::MIN, V::MAX);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              P::from_f64(0.0),
            is_paused:                  false,
            is_looping:                 false,
            is_ticking_up,
            is_handling_frame_spikes:   false,
        }
    }

    ///
    pub fn new_looper_with_frame_spike_handling(
        starting_value: V,
        end_value: V,
        interval: P,
        is_ticking_up: bool,
    ) -> Self {

        // Panic Evaluators
        check_if_value_is_within_range(starting_value, V::MIN, V::MAX);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              P::from_f64(0.0),
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up,
            is_handling_frame_spikes:   true,
        }
    }

    ///
    pub fn new_looper_without_frame_spike_handling(
        starting_value: V,
        end_value: V,
        interval: P,
        is_ticking_up: bool,
    ) -> Self {

        // Panic Evaluators
        check_if_value_is_within_range(starting_value, V::MIN, V::MAX);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              P::from_f64(0.0),
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up,
            is_handling_frame_spikes:   false,
        }
    }
    // ######################################################################################## //



    // ##################################### GETTERS ########################################## //
    /// Returns the start_value of a Ticker.
    #[inline]
    pub fn start_value(&self) -> V {
        self.start_value
    }

    /// Returns the current_value of a Ticker.
    #[inline]
    pub fn current_value(&self) -> V {
        self.current_value
    }

    /// Returns the end_value of a Ticker.
    #[inline]
    pub fn end_value(&self) -> V {
        self.end_value
    }

    /// Returns the interval of a Ticker.
    ///
    /// # What Exactly is the Interval?
    /// The interval is what dictates how long in \[INSERT_TIME_UNIT_HERE\] that it takes for current_value to increase
    /// or decrease by 1; direction depends on is_ticking_up.
    ///
    /// # Unit Type of Interval?
    /// Ticker has no built-in concept of "seconds" or any other unit — interval and accrued_delta
    /// are just two numbers compared against each other inside .tick(). The unit they represent is
    /// determined entirely by whatever unit the caller's delta is expressed in.
    ///
    /// The ticker_ticking system happens to pass seconds (the difference in time between
    /// 2 frames, sourced from Bevy's Time resource), so interval and accrued_delta are conventionally seconds when
    /// using that system. But nothing stops a custom implementation from feeding .tick() a delta
    /// in any other unit that meaningfully represents change over some interval.  In a custom
    /// implementation, it could literally be the difference in the number of clowns seen between
    /// two blinks.
    #[inline]
    pub fn interval(&self) -> P {
        self.interval
    }

    /// Returns the accrued_delta of a Ticker.
    ///
    /// # When Should I Use This Method?
    /// Realistically speaking, this method has limited use in most cases — accrued_delta holds
    /// only the leftover remainder from the last call to .tick(), not the total elapsed time
    /// since the Ticker was created or last reset.  It exists mainly for debugging, logging, or
    /// custom structures that need to inspect or manually carry over a Ticker's in-progress
    /// timing state.
    ///
    /// # Unit Type of Accrued Delta?
    /// Ticker has no built-in concept of "seconds" or any other unit — interval and accrued_delta
    /// are just two numbers compared against each other inside .tick(). The unit they represent is
    /// determined entirely by whatever unit the caller's delta is expressed in.
    ///
    /// The ticker_ticking system happens to pass seconds (the difference in time between
    /// 2 frames, sourced from Bevy's Time resource), so interval and accrued_delta are conventionally seconds when
    /// using that system. But nothing stops a custom implementation from feeding .tick() a delta
    /// in any other unit that meaningfully represents change over some interval.  In a custom
    /// implementation, it could literally be the difference in the number of clowns seen between
    /// two blinks.
    #[inline]
    pub fn accrued_delta(&self) -> P {
        self.accrued_delta
    }

    /// Returns the paused state of a Ticker.
    #[inline]
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Returns whether or not a Ticker is set to loop upon reaching either `start_value` or `end_value`.
    #[inline]
    pub fn is_looping(&self) -> bool {
        self.is_looping
    }

    /// Returns whether or not a Ticker is set to count its `current_value` up or down.
    #[inline]
    pub fn is_ticking_up(&self) -> bool {
        self.is_ticking_up
    }

    /// Returns whether or not a Ticker is set to handle frame spikes in its .tick() method.
    #[inline]
    pub fn is_handling_frame_spikes(&self) -> bool {
        self.is_handling_frame_spikes
    }
    // ######################################################################################## //



    // ##################################### SETTERS ########################################## //
    /// Changes `start_value` to the passed value.
    ///
    /// # Important
    /// `start_value` can NOT go out of the range of `V::MIN` to `V::MAX`.
    /// Attempting to set `start_value` outside the range will cause it to be clamped down.
    #[inline]
    pub fn set_start_value(&mut self, value: V) {
        self.start_value = value.clamp(V::MIN, V::MAX);
    }

    /// Changes `current_value` to the passed value.
    ///
    /// # Important
    /// `current_value` can NOT go out of the range that `start_value` and `end_value` create.
    /// Attempting to set `current_value` outside the range will cause it to be clamped down.
    pub fn set_current_value(&mut self, value: V) {
        let min = self.start_value.min(self.end_value);
        let max = self.start_value.max(self.end_value);
        self.current_value = value.clamp(min, max);
    }

    /// Changes `end_value` to the passed value.
    ///
    /// # Important
    /// `end_value` can NOT go out of the range of `V::MIN` to `V::MAX`.
    /// Attempting to set `end_value` outside the range will cause it to be clamped down.
    #[inline]
    pub fn set_end_value(&mut self, value: V) {
        self.end_value = value.clamp(V::MIN, V::MAX);
    }

    /// Pauses a ticker's ticking.
    ///
    /// This prevents the .tick() method from doing any calculations.
    #[inline]
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Unpauses a ticker's ticking.
    ///
    /// This allows the .tick() method to resume its calculations.
    #[inline]
    pub fn unpause(&mut self) {
        self.is_paused = false;
    }

    /// Sets a ticker to loop its counting when `current_value` reaches either `start_value` or `end_value`.
    ///
    /// # Important
    /// Triggering a loop will mean for `current_value` to be set to `start_value`.
    #[inline]
    pub fn start_looping(&mut self) {
        self.is_looping = true;
    }

    /// Prevents a ticker from looping when `current_value` reaches either `start_value` or `end_value`.
    #[inline]
    pub fn stop_looping(&mut self) {
        self.is_looping = false;
    }

    /// Causes the ticker's current_value to count up.
    ///
    /// Will allow calculated ticks inside the .tick() method to add to current_value, rather than subtract.
    #[inline]
    pub fn tick_up(&mut self) {
        self.is_ticking_up = true;
    }

    /// Causes the ticker's current_value to count down.
    ///
    /// Will allow calculated ticks inside the .tick() method to subtract from current_value, rather than add.
    #[inline]
    pub fn tick_down(&mut self) {
        self.is_ticking_up = false;
    }

    ///
    #[inline]
    pub fn start_handling_frame_spikes(&mut self) {
        self.is_handling_frame_spikes = true;
    }

    ///
    #[inline]
    pub fn stop_handling_frame_spikes(&mut self) {
        self.is_handling_frame_spikes = false;
    }
    // ######################################################################################## //



    // ################################### EQUAL METHODS ###################################### //
    /// Returns true if the current_value and the start_value are equal to one another, false otherwise.
    ///
    /// # When Should I Use This Method?
    /// Use this method in onetime tickers that count to start_value if you want to determine if the
    /// onetime ticker is finished.
    #[inline]
    pub fn is_current_equal_to_start(&self) -> bool {
        self.current_value == self.start_value
    }

    /// Returns true if the current_value and the end_value are equal to one another, false otherwise.
    ///
    /// # When Should I Use This Method?
    /// Use this method in onetime tickers that count to end_value if you want to determine if the
    /// onetime ticker is finished.
    #[inline]
    pub fn is_current_equal_to_end(&self) -> bool {
        self.current_value == self.end_value
    }

    /// Returns true if the start_value and the end_value are equal to one another, false otherwise.
    ///
    /// # Why Does This Method Exist?
    /// start_value and end_value can equal one another since their values can be changed or set to
    /// the same value at the creation of a Ticker instance.
    #[inline]
    pub fn is_start_equal_to_end(&self) -> bool {
        self.start_value == self.end_value
    }
    // ######################################################################################## //



    // ################################# DIFFERENCE METHODS ################################### //
    /// Returns the difference between current_value and start_value.
    ///
    /// Will only return positive numbers, including 0.
    pub fn difference_from_start(&self) -> i64 {
        let min: i64 = self.current_value.min(self.start_value).as_i64();
        let max: i64 = self.current_value.max(self.start_value).as_i64();
        max - min
    }

    /// Returns the difference between current_value and end_value.
    ///
    /// Will only return positive numbers, including 0.
    pub fn difference_from_end(&self) -> i64 {
        let min: i64 = self.current_value.min(self.end_value).as_i64();
        let max: i64 = self.current_value.max(self.end_value).as_i64();
        max - min
    }

    /// Returns the difference between start_value and end_value.
    ///
    /// Will only return positive numbers, including 0.
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
    /// "with_drop_accounting" version - there is always a ones-place.
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
        if self.current_value.absolute() >= V::from_i32(1000) {
            ((self.current_value.absolute() / V::from_i32(1000)) % V::from_i32(10)).as_i8()
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
        if self.current_value.absolute() >= V::from_i32(10000) {
            ((self.current_value.absolute() / V::from_i32(10000)) % V::from_i32(10)).as_i8()
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
        if self.current_value.absolute() >= V::from_i32(100000) {
            ((self.current_value.absolute() / V::from_i32(100000)) % V::from_i32(10)).as_i8()
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
        if self.current_value.absolute() >= V::from_i32(1000000) {
            ((self.current_value.absolute() / V::from_i32(1000000)) % V::from_i32(10)).as_i8()
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
        if self.current_value.absolute() >= V::from_i32(10000000) {
            ((self.current_value.absolute() / V::from_i32(10000000)) % V::from_i32(10)).as_i8()
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
        if self.current_value.absolute() >= V::from_i32(100000000) {
            ((self.current_value.absolute() / V::from_i32(100000000)) % V::from_i32(10)).as_i8()
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
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
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
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
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
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_4_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(1000) {
            ((self.current_value.absolute() / V::from_i32(1000)) % V::from_i32(10)).as_i8()
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
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_5_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(10000) {
            ((self.current_value.absolute() / V::from_i32(10000)) % V::from_i32(10)).as_i8()
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
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_6_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(100000) {
            ((self.current_value.absolute() / V::from_i32(100000)) % V::from_i32(10)).as_i8()
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
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_7_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(1000000) {
            ((self.current_value.absolute() / V::from_i32(1000000)) % V::from_i32(10)).as_i8()
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
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_8_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(10000000) {
            ((self.current_value.absolute() / V::from_i32(10000000)) % V::from_i32(10)).as_i8()
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
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_9_with_dda(&self) -> i8 {
        if self.current_value.absolute() >= V::from_i32(100000000) {
            ((self.current_value.absolute() / V::from_i32(100000000)) % V::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }
    // ######################################################################################## //



    // ################################### ADD METHODS ######################################## //
    /// Adds to the start_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within [`i8::MIN`] to [`i8::MAX`] (inclusive).
    #[inline]
    pub fn add_to_start_value(&mut self, value: V) {
        self.start_value = self.start_value.sat_add(value).clamp(V::MIN, V::MAX);
    }

    /// Adds to the current_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within start_value to end_value (inclusive).
    pub fn add_to_current_value(&mut self, value: V) {
        let min = self.start_value.min(self.end_value);
        let max = self.start_value.max(self.end_value);
        self.current_value = self.current_value.sat_add(value).clamp(min, max);
    }

    /// Adds to the end_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within [`i8::MIN`] to [`i8::MAX`] (inclusive).
    #[inline]
    pub fn add_to_end_value(&mut self, value: V) {
        self.end_value = self.end_value.sat_add(value).clamp(V::MIN, V::MAX);
    }

    /// Adds to the interval of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// # IMPORTANT
    /// Interval can never be 0, a negative number, or go past f32::MAX; the reasoning for this is that it would cause the
    /// .tick method to create crazy values. If your goal is to slow time or slow an accumulation to the point that it reverses it,
    /// I suggest you flip the tick direction using .tick_up or .tick_down at a specific current_value or
    /// after the rate of slow/speed you're applying has hit a specific value.
    #[inline]
    pub fn add_to_interval(&mut self, value: P) {
        self.interval = (self.interval + value).clamp(P::MIN_POSITIVE, P::MAX);
    }
    // ######################################################################################## //



    // ################################# MISCELLANEOUS METHODS ################################ //
    /// Returns the percentage of `current_value`'s distance from `start_value`, measured across
    /// the full range from `start_value` to `end_value`.
    ///
    /// A return value of `0.0` means `current_value` is at `start_value`, and `1.0` means it is
    /// at `end_value`.
    ///
    /// # Special Case
    /// Returns `-1.0` if `start_value` and `end_value` are equal, as no meaningful range exists.
    ///
    /// # Examples
    /// ```
    /// // start_value = 0, current_value = 40, end_value = 100
    /// // Returns 0.4
    ///
    /// // start_value = -37, current_value = 40, end_value = 80
    /// // Returns ~0.6581
    ///
    /// // Equal boundaries (special case)
    /// // start_value = 100, current_value = x, end_value = 100
    /// // Returns -1.0
    /// ```
    pub fn percentage_from_start(&self) -> f32 {

        if self.start_value == self.end_value {
            return -1.0;
        }

        let start: f32 = self.start_value.as_f32();
        let current: f32 = self.current_value.as_f32();
        let end: f32 = self.end_value.as_f32();

        let range_reciprocal: f32 = 1.0 / (end - start);

        (current - start) * range_reciprocal
    }

    /// Returns the percentage of `current_value`'s distance from `end_value`, measured across
    /// the full range from `end_value` to `start_value`.
    ///
    /// A return value of `0.0` means `current_value` is at `end_value`, and `1.0` means it is
    /// at `start_value`.
    ///
    /// # Special Case
    /// Returns `-1.0` if `start_value` and `end_value` are equal, as no meaningful range exists.
    ///
    /// # Examples
    /// ```
    /// // start_value = 0, current_value = 60, end_value = 100
    /// // Returns 0.4
    ///
    /// // start_value = -37, current_value = 40, end_value = 80
    /// // Returns ~0.3419
    ///
    /// // Equal boundaries (special case)
    /// // start_value = 100, current_value = x, end_value = 100
    /// // Returns -1.0
    /// ```
    pub fn percentage_from_end(&self) -> f32 {

        if self.start_value == self.end_value {
            return -1.0;
        }

        let start: f32 = self.start_value.as_f32();
        let current: f32 = self.current_value.as_f32();
        let end: f32 = self.end_value.as_f32();

        let range_reciprocal: f32 = 1.0 / (end - start);

        (end - current) * range_reciprocal
    }

    /// Will set the current_value to be equal to the start_value.
    #[inline]
    pub fn reset(&mut self) {
        self.current_value = self.start_value;
    }

    /// Will set the current_value to be equal to the start_value and zero out the accrued_delta.
    ///
    /// # When To Use This Over Reset?
    /// Best to use when you want to completely wipe whatever has been accumulated, including the
    /// timing state.  If you need to carry over the timing state (the remainder in the last tick
    /// calculation), then do NOT use this.
    #[inline]
    pub fn hard_reset(&mut self) {
        self.current_value = self.start_value;
        self.accrued_delta = P::from_f64(0.0);
    }

    ///
    pub fn tick(&mut self, delta: P) {

        // PAUSE STATUS
        // If paused, go no further as we don't need to calculate the new current_value since the Ticker is frozen.
        if self.is_paused {
            return;
        }

        // DELTA ACCUMULATION
        // Add to the accrued delta so that we can later determine if we've gone over the interval value and need to fire another tick.
        self.accrued_delta += delta;

        // TICK COLLECTION
        // Acquiring the amount of tick fires that occurred within the given frame based on if
        // the Ticker is set to handle frame spikes.
        let ticks = match self.is_handling_frame_spikes {

            // TICK COLLECTION WHEN HANDLING FRAME SPIKES
            // When frame spike handling is active, all ticks that accumulated during a large
            // delta are collected at once.  The remainder after division is kept in accrued_delta
            // so that partial progress toward the next tick is not lost between frames.
            //
            // as_64() in tick_count_truncated_to_value_type is acting as a bridge for V and P to work
            // with one another.  It does mean that a typecast to f64 happens here, but the requested
            // precision is still maintained since the calculated ticks happened inside ticks_calculated_in_active_precision.
            // After that the value gets truncated using V::from_f64 since all V types are integers.
            true => {
                let ticks_calculated_in_active_precision: P = self.accrued_delta / self.interval;
                let tick_count_truncated_to_value_type: V = V::from_f64(ticks_calculated_in_active_precision.as_f64());
                self.accrued_delta %= self.interval; // Carrying remainder over to keep ticking accuracy.
                tick_count_truncated_to_value_type
            },

            // TICK COLLECTION WHEN ~NOT~ HANDLING FRAME SPIKES
            // When frame spike handling is inactive, only one tick is allowed to fire per call
            // regardless of how large the delta was.  One interval is subtracted from accrued_delta
            // rather than resetting to zero so that the timer remains accurate over time — any
            // leftover time beyond the single tick carries into the next frame naturally.
            false => match self.accrued_delta >= self.interval {
                true => {
                    self.accrued_delta -= self.interval;
                    V::from_i32(1)
                },
                false => V::from_i32(0),
            },
        };

        // TICK FIRE TO CHANGE CURRENT_VALUE
        // Will only ever tick fire if the accrued delta pushed ticks beyond the interval value.
        // This check ensures we aren't needlessly firing for every frame, rather we are firing
        // based on if we've passed over the interval threshold using our constant accrual.
        if ticks > V::from_i32(0) {

            // TICK FIRE DIRECTION
            // Increase or decrease current_value's new host based on if the Ticker is counting up or down.
            let new_value = match self.is_ticking_up {
                true  => self.current_value.sat_add(ticks),
                false => self.current_value.sat_sub(ticks),
            };

            // DETERMINING CURRENT_VALUE'S BOUNDARIES
            // Since start_value and end_value can be either negative or positive at any given moment,
            // we must throw both values against one another to determine whose greater/lesser than
            // the other so that we can properly clamp down current_value to its allowed range.
            let min = self.start_value.min(self.end_value);
            let max = self.start_value.max(self.end_value);

            // RESET DETERMINATION + CURRENT_VALUE ASSIGNMENT
            // Will change current_value's assignment using new_value based on if the Ticker is set to loop or not.
            match self.is_looping {

                // LOOPING IS ACTIVE
                // Assign current_value to its new host and then reset it to the Ticker's start_value
                // if either of its boundaries -- start_value and end_value -- are hit.
                true => {
                    self.current_value = new_value;
                    if self.current_value <= min || self.current_value >= max {
                        self.reset();
                    }
                },

                // LOOPING IS INACTIVE
                // current_value can assume its new host after new_value has been clamped to the allowed range.
                false => {
                    self.current_value = new_value.clamp(min, max);
                },
            };
        }
    }
}
// ############################################################################################## //
