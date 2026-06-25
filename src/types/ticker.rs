use std::cmp::PartialEq;
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
/// #### Why Declare a MIN and MAX?
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
/// Used to apply a generic to the stored_time and time_interval fields within the Ticker type.
///
/// Supports f32 and f64 for stored_time and time_interval fields within Ticker.
///
/// #### Why Have f32 and f64 for Precision?
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

/// ADD SHTUFFF HERE!
#[derive(PartialEq, Reflect, Debug)]
pub enum TickerForms {
    Looper,
    MutLooper,
    Oneshot,
    MutOneshot,
    Freezing,
}

// ################################# TICKER IMPLEMENTATION ###################################### //
/// A generic, self-contained counter that advances a value over time at a fixed time_interval.
///
/// MAKE SURE TO EXPLAIN TIME IN VARIOUS WAYS, DON'T JUST USE FRAMES!  USE THE CLOWNS OVER BLINKS EXAMPLE!
#[derive(Component, Reflect, Debug)]
pub struct Ticker<V: TickerValue, P: TickerPrecision> {
    start_value:                V,
    current_value:              V,
    end_value:                  V,
    time_interval:              P,
    stored_time:                P,
    is_paused:                  bool,
    is_ticking_up:              bool,
    is_handling_time_spikes:    bool,
    form:                       TickerForms,
}

impl<V: TickerValue, P: TickerPrecision> Default for Ticker<V, P> {
    ///
    fn default() -> Self {
        Self {
            start_value:                V::from_i32(0),
            current_value:              V::from_i32(0),
            end_value:                  V::from_i32(100),
            time_interval:              P::from_f64(1.0),
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up:              true,
            is_handling_time_spikes:    true,
            form:                       TickerForms::MutLooper,
        }
    }
}

impl<V: TickerValue, P: TickerPrecision> Ticker<V, P> {

    // ##################################### CONSTRUCTORS ######################################## //
    /// Use this when you need to completely define your own Ticker; full-custom.
    ///
    /// #### Important
    /// Text
    pub fn new(
        start_value:                V,
        current_value:              V,
        end_value:                  V,
        time_interval:              P,
        is_paused:                  bool,
        is_ticking_up:              bool,
        is_handling_time_spikes:    bool,
        form:                       TickerForms,
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
            form,
        }
    }

    ///
    pub fn new_looper(
        starting_value:             V,
        end_value:                  V,
        time_interval:              P,
        is_ticking_up:              bool,
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
            is_ticking_up,
            is_handling_time_spikes,
            form:                       TickerForms::Looper,
        }
    }

    ///
    pub fn new_oneshot(
        starting_value:             V,
        end_value:                  V,
        time_interval:              P,
        is_ticking_up:              bool,
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
            is_ticking_up,
            is_handling_time_spikes,
            form:                       TickerForms::Oneshot,
        }
    }

    ///
    pub fn new_mut_looper(
        starting_value:             V,
        end_value:                  V,
        time_interval:              P,
        is_ticking_up:              bool,
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
            is_ticking_up,
            is_handling_time_spikes,
            form:                       TickerForms::MutLooper,
        }
    }


    ///
    pub fn new_mut_oneshot(
        starting_value:             V,
        end_value:                  V,
        time_interval:              P,
        is_ticking_up:              bool,
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
            is_ticking_up,
            is_handling_time_spikes,
            form:                       TickerForms::MutOneshot,
        }
    }


    ///
    pub fn new_freezing(
        starting_value:             V,
        end_value:                  V,
        time_interval:              P,
        is_ticking_up:              bool,
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
            is_ticking_up,
            is_handling_time_spikes,
            form:                       TickerForms::Freezing,
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

    /// Returns the time_interval of a Ticker.
    ///
    /// # What Exactly is the Interval?
    /// The time_interval is what dictates how long in \[INSERT_TIME_UNIT_HERE\] that it takes for current_value to increase
    /// or decrease by 1; direction depends on is_ticking_up.
    ///
    /// # Unit Type of Interval?
    /// Ticker has no built-in concept of "seconds" or any other unit — time_interval and stored_time
    /// are just two numbers compared against each other inside .tick(). The unit they represent is
    /// determined entirely by whatever unit the caller's delta is expressed in.
    ///
    /// The ticker_ticking system happens to pass seconds (the difference in time between
    /// 2 frames, sourced from Bevy's Time resource), so time_interval is conventionally seconds when
    /// using that system. But nothing stops a custom implementation from feeding .tick() a delta
    /// in any other unit that meaningfully represents change over some time_interval.  In a custom
    /// implementation, it could literally be the difference in the number of clowns seen between
    /// two blinks.  In such a case, time_interval would have clowns as its unit.
    #[inline]
    pub fn time_interval(&self) -> P {
        self.time_interval
    }

    /// Returns the stored_time of a Ticker.
    ///
    /// # When Should I Use This Method?
    /// Realistically speaking, this method has limited use in most cases — stored_time holds
    /// only the leftover remainder from the last call to .tick(), not the total elapsed time
    /// since the Ticker was created or last reset.  It exists mainly for debugging, logging, or
    /// custom structures that need to inspect or manually carry over a Ticker's in-progress
    /// timing state.
    ///
    /// # Unit Type of Accrued Delta?
    /// Ticker has no built-in concept of "seconds" or any other unit — time_interval and stored_time
    /// are just two numbers compared against each other inside .tick(). The unit they represent is
    /// determined entirely by whatever unit the caller's delta is expressed in.
    ///
    /// The ticker_ticking system happens to pass seconds (the difference in time between
    /// 2 frames, sourced from Bevy's Time resource), so stored_time is conventionally seconds between frames when
    /// using that system. But nothing stops a custom implementation from feeding .tick() a delta
    /// in any other unit that meaningfully represents change over some time_interval.  In a custom
    /// implementation, it could literally be the difference in the number of clowns seen between
    /// two blinks.  In such a case, stored_time would be clowns seen between blinks.
    #[inline]
    pub fn stored_time(&self) -> P {
        self.stored_time
    }

    /// Returns the paused state of a Ticker.
    #[inline]
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Returns true if a Ticker is set to tick its `current_value` up, false otherwise.
    #[inline]
    pub fn is_ticking_up(&self) -> bool {
        self.is_ticking_up
    }

    /// Returns true if a Ticker is set to tick its `current_value` down, false otherwise.
    #[inline]
    pub fn is_ticking_down(&self) -> bool {
        if self.is_ticking_up == false {
            true
        }
        else {
            false
        }
    }

    /// Returns whether or not this Ticker can fire more than one tick in a single .tick() call.
    ///
    /// - `TRUE`: When a lot of time (or whatever unit delta in .tick() represents) has built up since the
    /// last call, the Ticker will catch up all at once — firing every tick it's owed in one go.
    ///
    /// - `FALSE`: The Ticker only ever fires one tick per call, no matter how much has built up.
    /// Anything extra is saved and carried over to the next call instead of being used right away.
    #[inline]
    pub fn is_handling_time_spikes(&self) -> bool {
        self.is_handling_time_spikes
    }

    ///
    #[inline]
    pub fn form(&self) -> &TickerForms {
        &self.form
    }
    // ######################################################################################## //



    // ##################################### SETTERS ########################################## //
    /// Changes `start_value` to the passed value.
    ///
    /// # Important
    /// `start_value` can NOT go out of the range of `V::MIN` to `V::MAX`.
    /// Attempting to set `start_value` outside the range will cause it to be clamped down.
    pub fn set_start_value(&mut self, value: V) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                self.start_value = value.clamp(V::MIN, V::MAX);
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    self.start_value = value.clamp(V::MIN, V::MAX);
                }
            }
            _ => {}
        }
    }

    /// Changes `current_value` to the passed value.
    ///
    /// # Important
    /// `current_value` can NOT go out of the range that `start_value` and `end_value` create.
    /// Attempting to set `current_value` outside the range will cause it to be clamped down.
    pub fn set_current_value(&mut self, value: V) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                let min = self.start_value.min(self.end_value);
                let max = self.start_value.max(self.end_value);
                self.current_value = value.clamp(min, max);
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    let min = self.start_value.min(self.end_value);
                    let max = self.start_value.max(self.end_value);
                    self.current_value = value.clamp(min, max);
                }
            }
            _ => {}
        }
    }

    /// Changes `end_value` to the passed value.
    ///
    /// # Important
    /// `end_value` can NOT go out of the range of `V::MIN` to `V::MAX`.
    /// Attempting to set `end_value` outside the range will cause it to be clamped down.
    pub fn set_end_value(&mut self, value: V) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                self.end_value = value.clamp(V::MIN, V::MAX);
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    self.end_value = value.clamp(V::MIN, V::MAX);
                }
            }
            _ => {}
        }
    }

    /// Pauses a ticker's ticking.
    ///
    /// This prevents the .tick() method from doing any calculations.
    pub fn pause(&mut self) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                self.is_paused = true;
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    self.is_paused = true;
                }
            }
            _ => {}
        }
    }

    /// Unpauses a ticker's ticking.
    ///
    /// This allows the .tick() method to resume its calculations.
    pub fn unpause(&mut self) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                self.is_paused = false;
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    self.is_paused = false;
                }
            }
            _ => {}
        }
    }

    /// Causes the ticker's current_value to count up.
    ///
    /// Will allow calculated ticks inside the .tick() method to add to current_value, rather than subtract.
    pub fn tick_up(&mut self) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                self.is_ticking_up = true;
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    self.is_ticking_up = true;
                }
            }
            _ => {}
        }
    }

    /// Causes the ticker's current_value to count down.
    ///
    /// Will allow calculated ticks inside the .tick() method to subtract from current_value, rather than add.
    pub fn tick_down(&mut self) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                self.is_ticking_up = false;
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    self.is_ticking_up = false;
                }
            }
            _ => {}
        }
    }

    ///
    pub fn start_handling_time_spikes(&mut self) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                self.is_handling_time_spikes = true;
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    self.is_handling_time_spikes = true;
                }
            }
            _ => {}
        }
    }

    ///
    pub fn stop_handling_time_spikes(&mut self) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                self.is_handling_time_spikes = false;
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    self.is_handling_time_spikes = false;
                }
            }
            _ => {}
        }
    }

    /// ADD MORE TO THIS DOC COMMENT
    ///
    /// #### Important
    /// Can be used to stop looping by switching the form from looper to MutOneshot or oneshot variants.
    /// Can be used to switch from immutable forms to mutable ones.
    #[inline]
    pub fn set_form(&mut self, new_form: TickerForms) {
        self.form = new_form;
    }
    // ######################################################################################## //



    // ################################### EQUAL METHODS ###################################### //
    /// Returns true if the current_value and the start_value are equal to one another, false otherwise.
    ///
    /// # When Should I Use This Method?
    /// Use this method in oneshot tickers that count to start_value if you want to determine if the
    /// oneshot ticker is finished.
    #[inline]
    pub fn is_current_at_start(&self) -> bool {
        self.current_value == self.start_value
    }

    /// Returns true if the current_value and the end_value are equal to one another, false otherwise.
    ///
    /// # When Should I Use This Method?
    /// Use this method in oneshot tickers that count to end_value if you want to determine if the
    /// oneshot ticker is finished.
    #[inline]
    pub fn is_current_at_end(&self) -> bool {
        self.current_value == self.end_value
    }

    /// Returns true if the start_value and the end_value are equal to one another, false otherwise.
    ///
    /// # Why Does This Method Exist?
    /// start_value and end_value can equal one another since their values can be changed or set to
    /// the same value at the creation of a Ticker instance.
    ///
    /// # When Should I Use This Method?
    /// Only scenario I can think for using this would be when the bounds of a Ticker are slowly tightening
    /// and you need something to check when they have finally met one another.  It is possible to tighten
    /// the bounds by constantly setting `start_value` and `end_value` to new numbers.
    #[inline]
    pub fn is_start_at_end(&self) -> bool {
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
    /// Will not let the result of summing cause overflow or wrapping; results will always be within `V::MIN` to `V::MAX` (inclusive).
    pub fn add_to_start_value(&mut self, value: V) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                self.start_value = self.start_value.sat_add(value).clamp(V::MIN, V::MAX);
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    self.start_value = self.start_value.sat_add(value).clamp(V::MIN, V::MAX);
                }
            }
            _ => {}
        }
    }

    /// Adds to the current_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within `start_value` to `end_value` (inclusive).
    pub fn add_to_current_value(&mut self, value: V) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                let min = self.start_value.min(self.end_value);
                let max = self.start_value.max(self.end_value);
                self.current_value = self.current_value.sat_add(value).clamp(min, max);
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    let min = self.start_value.min(self.end_value);
                    let max = self.start_value.max(self.end_value);
                    self.current_value = self.current_value.sat_add(value).clamp(min, max);
                }
            }
            _ => {}
        }
    }

    /// Adds to the end_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within `V::MIN` to `V::MAX` (inclusive).
    pub fn add_to_end_value(&mut self, value: V) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                self.end_value = self.end_value.sat_add(value).clamp(V::MIN, V::MAX);
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    self.end_value = self.end_value.sat_add(value).clamp(V::MIN, V::MAX);
                }
            }
            _ => {}
        }
    }

    /// Adds to the time_interval of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// #### What Values Can Interval Be Set To?
    /// Interval can never be 0, a negative number, or go past `P::MAX`; the reasoning for this is that it would cause the
    /// .tick method to create crazy values.
    ///
    /// #### Can Interval Flip Direction of a Ticker?
    /// No. If your goal is to slow time or slow an accumulation to the point that it reverses it,
    /// I suggest you flip the tick direction using .tick_up() or .tick_down() at a specific current_value or
    /// after the rate of slow/speed you're applying has hit a specific value.
    pub fn add_to_time_interval(&mut self, value: P) {
        match self.form {
            TickerForms::MutLooper |
            TickerForms::MutOneshot => {
                self.time_interval = (self.time_interval + value).clamp(P::MIN_POSITIVE, P::MAX);
            }
            TickerForms::Freezing => {
                if self.current_value != self.end_value {
                    self.time_interval = (self.time_interval + value).clamp(P::MIN_POSITIVE, P::MAX);
                }
            }
            _ => {}
        }
    }
    // ######################################################################################## //



    // ################################# MISCELLANEOUS METHODS ################################ //
    /// Returns the percentage of `current_value`'s distance from `start_value`, measured across
    /// the full range from `start_value` to `end_value`.
    ///
    /// A return value of `0.0` means `current_value` is at `start_value`, and `1.0` means it is
    /// at `end_value`.
    ///
    /// #### Special Case
    /// Returns `-1.0` if `start_value` and `end_value` are equal, as no meaningful range exists.
    ///
    /// #### Examples
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
    /// #### Special Case
    /// Returns `-1.0` if `start_value` and `end_value` are equal, as no meaningful range exists.
    ///
    /// #### Examples
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

    /// Will set the current_value to be equal to the start_value and zero out the stored_time.
    ///
    /// #### When To Use This Over Reset?
    /// Best to use when you want to completely wipe whatever has been accumulated, including the
    /// timing state.  If you need to carry over the timing state (the remainder in the last .tick()
    /// calculation) then do NOT use this.
    #[inline]
    pub fn hard_reset(&mut self) {
        self.current_value = self.start_value;
        self.stored_time = P::from_f64(0.0);
    }

    /// #### Description of Method
    /// Used to advance a Ticker by taking in a passing of time between 2 events, *usually* for events
    /// that happen both consistently and constantly (such as frames rendering); it does have the
    /// ability to take in **any** delta time.
    ///
    /// #### What The Hell Does Ticking Do?
    /// The simplified version (read the method's code for the complex version) of calling the .tick()
    /// method on a Ticker is as follows:
    /// 1. Increase stored_time by the value passed to the .tick() call (elapsed_time_between_events).
    /// 2. If stored_time is greater than or equal to time_interval, change current_value.
    /// 3. Reassign stored_time to the result of (stored_time %= time_interval).  We do this to carry
    /// over our remainder that will not be part of current_value's change.
    ///
    /// #### What Impacts This Method And How?
    /// The fields of a Ticker which impact the calculations inside this method are as follows:
    /// - `is_paused`
    ///     - **True** : Prevent the tick method from doing anything.
    ///     - **False** : Tick method will determine if current_value needs to be changed.
    /// - `is_looping`
    ///     - **True** : Reset current_value to start_value when a Ticker boundary is hit; boundaries are start_value and end_value.
    ///     - **False** :  current_value will be set to the boundary it hits or goes past.
    /// - `is_ticking_up`
    ///     - **True** : .tick() calls will increase current_value.
    ///     - **False** : .tick() calls will decrease current_value.
    /// - `is_handling_time_spikes`
    ///     - **True** : The full integer magnitude from the result of `elapsed_time + stored_time` will be used to change current_value.
    ///     - **False** : As long as stored_time is greater than or equal to the time_interval, current_value will change by 1.
    ///
    /// #### Does This Method Impact a Ticker's Units?
    /// Yes.  The units for a Ticker's integers and float fields are based on what is passed into the
    /// .tick() method.  If the passed value represents the difference in seconds between 2 frames,
    /// the Ticker's time_interval and stored_time units would be seconds; the start_value,
    /// current_value, and end_value would also have seconds as their unit.
    ///
    /// #### Example of Method in Action
    /// Consider the following factors first:
    /// - `The Horror You've Undergone` : You've seen 8 clowns between 2 blinks.  You decide to make a Ticker to calculate this horror.
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

        // POTENTIAL RETURN: Ticker is Paused
        // If paused, go no further as we don't need to calculate the new current_value since the Ticker is frozen.
        if self.is_paused {
            return;
        }

        // TIME ACCUMULATION
        // Add to the stored_time so that we can later determine if we've gone over the time_interval value and need to fire another tick.
        self.stored_time += elapsed_time_between_events;

        // DETERMINING PASSED TIME
        // Acquiring the magnitude of time that passed between events.
        // How this is done is dependent on whether is_handling_time_spikes is set to true or false.
        let magnitude_of_time_that_passed = match self.is_handling_time_spikes {

            // PASSED TIME WHEN HANDLING TIME SPIKES
            // When time spike handling is active, all magnitude_of_time_that_passed that accumulated during a large
            // elapsed_time_between_events are collected at once.  The remainder after division is kept in stored_time
            // so that partial progress toward the next tick is not lost between the events that are
            // being timed.
            //
            // as_64() in tick_count_truncated_to_value_type is acting as a bridge for V and P to work
            // with one another.  It does mean that a typecast to f64 happens here, but the requested
            // precision is still maintained since the calculated magnitude_of_time_that_passed happened inside magnitude_of_time_that_passed_calculated_in_active_precision.
            // After that the value gets truncated using V::from_f64 since all V types are integers.
            true => {
                let magnitude_of_time_that_passed_in_active_precision: P = self.stored_time / self.time_interval;
                let magnitude_of_time_that_passed_truncated_to_value_type: V = V::from_f64(magnitude_of_time_that_passed_in_active_precision.as_f64());
                self.stored_time %= self.time_interval; // Carrying remainder over to keep ticking accuracy.
                magnitude_of_time_that_passed_truncated_to_value_type
            },

            // PASSED TIME WHEN ~NOT~ HANDLING TIME SPIKES
            // When time spike handling is inactive, only 1 tick is allowed to fire per call
            // regardless of how large the elapsed_time_between_events was.  One time_interval is subtracted from stored_time
            // rather than resetting to zero so that the timer remains accurate over time — any
            // leftover time beyond the single tick carries into the next .tick() call.
            //
            // We subtract by time_interval (rather than just discarding stored_time) specifically because
            // is_handling_time_spikes can be toggled at runtime.  If this flag is permanently false for
            // a given Ticker, the leftover precision wouldn't matter — each call only ever checks "has
            // one time_interval passed, yes or no" regardless of how much excess sits in stored_time.  But
            // since the flag can flip to true later, preserving the leftover ensures that switch correctly
            // picks up every tick that was quietly accumulating while spike handling was off, rather than
            // discarding that history the moment multi-tick firing gets re-enabled.
            false => match self.stored_time >= self.time_interval {
                true => {
                    self.stored_time -= self.time_interval;
                    V::from_i32(1)
                },
                false => V::from_i32(0),
            },
        };

        // TICK FIRE TO CHANGE CURRENT_VALUE
        // Will only ever tick fire if the stored_time pushed magnitude_of_time_that_passed beyond the time_interval value.
        // This check ensures we aren't needlessly firing for every frame, rather we are firing
        // based on if we've passed over the time_interval threshold using our constant accrual.
        //
        // To be perfectly clear, magnitude_of_time_that_passed can only be greater than 0 if the stored_time went past the
        // time_interval value.  Greater than 0 means 1 or higher in this case, decimals in between 0 and 1
        // don't count.
        if magnitude_of_time_that_passed > V::from_i32(0) {

            // TICK FIRE DIRECTION
            // Increase or decrease current_value's new host based on if the Ticker is counting up or down.
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
            // Will change current_value's assignment using new_value based on the Ticker form.
            match self.form {

                // LOOPER LOGIC
                // Assign current_value to its new host and then reset it to the Ticker's start_value
                // if either of its boundaries - start_value and end_value - are hit.
                TickerForms::Looper |
                TickerForms::MutLooper => {
                    self.current_value = new_value;
                    if self.current_value <= min || self.current_value >= max {
                        self.current_value = self.start_value;
                    }
                },

                // ONESHOT + FREEZING LOGIC
                // current_value can assume its new host after new_value has been clamped to the allowed range.
                // Additionally, stored_time will be zeroed out if current_value hits end_value.  We
                // do this stored_time wipe since oneshotters and freezings are purposed to clear their
                // time storage upon hitting their end destination.
                TickerForms::Oneshot |
                TickerForms::MutOneshot |
                TickerForms::Freezing => {
                    self.current_value = new_value.clamp(min, max);
                    if self.current_value == self.end_value {
                        self.stored_time = P::from_f64(0.0);
                    }
                },
            };
        }
    }
}
// ############################################################################################## //
