
// Imports
use bevy::prelude::*;

/// By themselves, tickers can be used to create simple timers.  Although they are best used in conjunction
/// as an inner element to a greater time structure to create some wicked tickety-tocking.
///
/// All fields of Ticker have getters, and only digit has no setter.
///
/// # TICKING LOOPS AT [`LOOP_POINT`]
/// Tickers don't stop ticking.  Once the next tick addition hits [`LOOP_POINT`] it will zero out current_value using to_zero().
/// **This is crucial to understand.** Not recognizing that ticking loops on these structures will make for poor usage of them.
/// Tickers are a building block element to making larger time structures or for highly compartmentalized timer usage.
/// **If you're okay with values from [`i8::MIN`] to [`TICKER_MAX_VALUE`] for your timers, then feel free to go ham with Tickers.**
/// Otherwise, I recommend the Chronolog structure.
#[derive(Component, Reflect, Debug)]
pub struct Ticker {
    pub start_value: i8,
    pub current_value: i8,
    pub interval: f32,
    pub is_paused: bool,
    pub is_looping: bool,
    pub is_ticking_up: bool,
    pub is_handling_frame_spikes: bool,
    accrued_delta: f32,
}

impl Default for Ticker {

    /// The default ticker counts up every second when its .tick method is used and all other fields start at 0.
    fn default() -> Self {
        Self {
            start_value:                0,
            current_value:              0,
            interval:                   1.0,
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up:              true,
            is_handling_frame_spikes:   true,
            accrued_delta:              0.0,
        }
    }
}

impl Ticker {

    /// Develops a new Ticker using a passed value for its start_value.
    ///
    /// Valid start values are [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] (inclusive).
    /// **Values outside this range will cause a panic.**
    ///
    /// When a second passes, the timer within the Ticker fires (increases current_value by 1 for each second that passes).
    pub fn new(starting_value: i8) -> Self {
        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            interval:                   1.0,
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up:              true,
            is_handling_frame_spikes:   true,
            accrued_delta:              0.0,
        }
    }

    /// Develops a new Ticker using a passed value for its start_value.
    ///
    /// Valid start values are [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] (inclusive).
    /// **Values outside this range will cause a panic.**
    ///
    /// When the passed duration in second(s) passes, the timer within the Ticker fires (increases current_value by 1 for each duration that passes).
    pub fn new_with_interval(starting_value: i8, seconds_required_for_a_tick: f32) -> Self {
        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            interval:                   seconds_required_for_a_tick,
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up:              true,
            is_handling_frame_spikes:   true,
            accrued_delta:              0.0,
        }
    }

    /// Creates a Ticker for countdown purposes.  Pass in the desired countdown duration as a number of seconds to pass.
    ///
    /// Valid countdown durations are 1 to [`i8::MAX`] (pass 10 in for a 10-second countdown); inclusive range.
    /// **Values outside this range will cause a panic.**
    ///
    /// The start_value for Tickers that use this constructor is calculated by ([`LOOP_POINT`] - DURATION).
    pub fn new_countdown(trigger_value: i8, countdown_duration: i8) -> Self {
        Self {
            start_value:                trigger_value,
            current_value:              countdown_duration,
            interval:                   1.0,
            is_paused:                  false,
            is_looping:                 false,
            is_ticking_up:              false,
            is_handling_frame_spikes:   true,
            accrued_delta:              0.0,
        }
    }

    /// Returns the current_value of a Ticker.
    pub fn get_current_value(&self) -> i8 {
        self.current_value
    }

    /// Returns the start_value of a Ticker.
    ///
    /// It's important to note that start_value can change through set_start_value(), so don't
    /// treat it as a consistent value.
    pub fn get_start_value(&self) -> i8 {
        self.start_value
    }

    /// Returns the difference between current_value and start_value, which does mean that negative values
    /// are a possibility for the result.
    ///
    /// A negative result indicates that the current_value is less than the start_value of the Ticker.
    ///
    /// A positive result indicates that the current_value is greater than the start_value of the Ticker.
    ///
    /// RESULT = CURRENT_VALUE - START_VALUE
    pub fn get_difference(&self) -> i16 {
        self.current_value as i16 - self.start_value as i16
    }

    /// Returns the digit in the ones-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// #### Small Note
    /// It is not possible for the ones digit to be dropped, hence the reason why this method has no
    /// "with_drop_accounting" version.  There is always a ones-place.
    pub fn get_ones_digit(&self) -> i8 {
        self.current_value.abs() % 10
    }

    /// Returns the digit in the tens-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    ///
    /// Use the "get_tens_digit_with_drop_accounting" version of this method to differentiate between
    /// digit drops and the 0 digit.
    pub fn get_tens_digit(&self) -> i8 {
        (self.current_value.abs() / 10) % 10
    }

    /// Returns the digit in the hundreds-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    ///
    /// Use the "get_hundreds_digit_with_drop_accounting" version of this method to differentiate
    /// between digit drops and the 0 digit.
    pub fn get_hundreds_digit(&self) -> i8 {
        (self.current_value.abs() / 100) % 10
    }

    /// Returns the digit in the tens-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// ### Example
    /// If current_value is 6, then -1 would be returned.  The flipside would be if current_value is
    /// 103, then 0 would be returned.  The potential for -1 to be returned allows for differentiation
    /// between a digit dropping and if a digit is simply 0 at a given time.
    pub fn get_tens_digit_with_drop_accounting(&self) -> i8 {

        if self.current_value.abs() < 10 {
            (self.current_value.abs() / 10) % 10
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
    /// ### Example
    /// If current_value is 17, then -1 would be returned.  The flipside would be if current_value is
    /// 1032 (not realistic for i8, just an example), then 0 would be returned.  The potential for -1
    /// to be returned allows for differentiation between a digit dropping and if a digit is simply 0 at
    /// a given time.
    pub fn get_hundreds_digit_with_drop_accounting(&self) -> i8 {

        if self.current_value.abs() < 100 {
            (self.current_value.abs() / 100) % 10
        }
        else {
            -1
        }
    }

    /// Returns true if the current_value of the Ticker is below its start_value, false otherwise.
    pub fn is_below_start_value(&self) -> bool {
        self.current_value < self.start_value
    }

    /// Returns true if the current_value of the Ticker is above its start_value, false otherwise.
    pub fn is_above_start_value(&self) -> bool {
        self.current_value > self.start_value
    }

    /// Returns true if the current_value and the start_value are equal to one another, false otherwise.
    ///
    /// When relying solely on frames, I think this would be rather difficult to trigger.  However,
    /// using the reset method and setters may allow for this to return true often depending on
    /// how said methods are used.
    pub fn is_equal_to_start_value(&self) -> bool {
        self.current_value == self.start_value
    }

    /// Pauses a ticker's ticking.
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Unpauses a ticker's ticking.
    pub fn unpause(&mut self) {
        self.is_paused = false;
    }

    /// Causes the ticker's current_value to count up.
    pub fn tick_up(&mut self) {
        self.is_ticking_up = true;
    }

    /// Causes the ticker's current_value to count down.
    pub fn tick_down(&mut self) {
        self.is_ticking_up = false;
    }

    /// Will set the current_value to be equal to the start_value and the digit field of the Ticker
    /// will be changed according to the new ones-place value that is seen after current_value's reset.
    ///
    /// Digit is always to reflect current_value's ones-place.
    pub fn reset(&mut self) {
        self.current_value = self.start_value;
    }

    /// Adds to the start_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] (inclusive).
    pub fn add_to_start_value(&mut self, value: i8) {
        self.start_value = self.start_value.saturating_add(value);
    }

    /// Adds to the current_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] (inclusive).
    pub fn add_to_current_value(&mut self, value: i8) {
        self.current_value = self.current_value.saturating_add(value);
    }

    /// Used to advance a ticker.  Takes in a time.delta() call off the time resource (Res<Time>) that Bevy provides.
    ///
    /// If you're making a custom ticking system and have stripped out the ticking systems provided
    /// in the systems of this plugin, then please note that you must run this each frame for time to move normally.
    ///
    /// # TICKING LOOPS AT [`LOOP_POINT`]
    /// Tickers don't stop ticking.  Once the next tick addition hits [`LOOP_POINT`] it will zero out current_value.
    /// **This is crucial to understand.** Not recognizing that ticking loops on these structures will make for poor usage of them.
    /// Tickers are a building block element to making larger time structures or for highly compartmentalized timer usage.
    /// **If you're okay with values from [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] for your timers, then feel free to go ham with Tickers.**
    /// Otherwise, I recommend the Chronolog structure.
    pub fn tick(&mut self, delta: f32) {

        // PAUSE STATUS
        // If paused, go no further as we don't need to calculate the new current_value since the Ticker is frozen.
        if self.is_paused {
            return;
        }

        // DELTA ACCUMULATION
        // Add to the accrued delta so that we can later determine if we've gone over the interval value and need to fire another tick.
        self.accrued_delta += delta;

        // TICK COLLECTION (TC)
        // Acquiring the amount of tick fires that occurred within the given frame based on if
        // the Ticker is set to handle frame spikes.
        let ticks = match self.is_handling_frame_spikes {

            // TC FOR HANDLING FRAME SPIKES
            // When frame spike handling is active, all ticks that accumulated during a large
            // delta are collected at once.  The remainder after division is kept in accrued_delta
            // so that partial progress toward the next tick is not lost between frames.
            true => {
                let tocks = (self.accrued_delta / self.interval) as i8;
                self.accrued_delta %= self.interval;    // Carrying remainder over to keep ticking accuracy.
                tocks
            },

            // TC FOR ~NOT~ HANDLING FRAME SPIKES
            // When frame spike handling is inactive, only one tick is allowed to fire per call
            // regardless of how large the delta was.  One interval is subtracted from accrued_delta
            // rather than resetting to zero so that the timer remains accurate over time — any
            // leftover time beyond the single tick carries into the next frame naturally.
            false => match self.accrued_delta >= self.interval {
                true => {
                    self.accrued_delta -= self.interval;
                    1
                },
                false => 0,
            },
        };

        // TICK FIRE TO CHANGE CURRENT_VALUE
        // Will only ever tick fire if the accrued delta pushed ticks beyond the interval value.
        // This check ensures we aren't needlessly firing for every frame, rather we are firing
        // based on if we've passed over the interval threshold using our constant accrual.
        if ticks > 0 {

            // TICK FIRE DIRECTION
            // Increase or decrease current_value's new host based on if the Ticker is counting up or down.
            let new_value = match self.is_ticking_up {
                true  => self.current_value.saturating_add(ticks),
                false => self.current_value.saturating_sub(ticks),
            };

            // RESET DETERMINATION + CURRENT_VALUE ASSIGNMENT
            // Will change current_value's assignment using new_value based on if the Ticker is set to loop or not.
            match self.is_looping {

                // LOOPING IS ACTIVE
                // Assign current_value to its new host and then reset it to the Ticker's start_value
                // if either of the i8 boundaries were hit.
                true => {
                    self.current_value = new_value;
                    if self.current_value == i8::MAX || self.current_value == i8::MIN {
                        self.current_value = self.start_value;
                    }
                },

                // LOOPING IS INACTIVE
                // current_value can assume its new host without worry.
                false => {
                    self.current_value = new_value;
                },
            };
        }
    }
}

/// Determines if the value is within the acceptable range that is passed in.  Will cause a panic if the value is out of the range.
fn check_value(value: i8, minimum: i8, maximum: i8) {
    assert!(value >= minimum && value <= maximum, "TICKER PANIC: Value must be between {} and {} (inclusive). Got {}.", minimum, maximum, value);
}
