
// Imports
use bevy::ecs::component::Component;
use bevy::reflect::Reflect;
use bevy::time::Timer;

/// By themselves, tickers can be used to create simple timers.  Although they are best used in conjunction
/// with a Chronolog to create some wicked tickety-tocking.
///
/// Non-chronolog tickers are only capable of the range of 0 to 9.  But with a repeating timer, good maffffs,
/// and an accumulator that's paired with a condition or two then you can go up to or below any number you want
/// with a non-chronolog ticker.
#[derive(Component, Reflect, Debug)]
pub struct Ticker {
    pub number: Option<u32>,
    pub timer: Option<Timer>,
}

impl Default for Ticker {
    fn default() -> Self {
        Self {
            number: None,
            timer: None,
        }
    }
}

impl Ticker {

    /// Used to advance a ticker.  Takes in a time.delta() call off the Res<Time> resource that Bevy provides.
    ///
    /// If you're making a custom ticking system and have stripped out the ticking systems provided
    /// in the systems of this plugin, then please note that you must run this each frame for time to move normally.
    pub fn tick(&mut self, delta: std::time::Duration) {

        // Checking/destructuring to see if a timer and a number exist inside the Ticker before trying to manipulate its contents.
        if let (Some(timer), Some(number)) = (&mut self.timer, &mut self.number) {

            // Advance timer by the difference in time between frames.
            // This .tick is Bevy's tick method for their timers, this isn't a recursive action.
            timer.tick(delta);

            // Handling frame spiking.
            let ticks = timer.times_finished_this_tick();
            if ticks > 0 {
                // Don't get rid of my modulo!  It's what's allowing digits to be processed correctly
                // inside chronologs.  The "number" property only goes up to 9 intentionally to properly
                // maintain digits inside chronologs.
                *number = (*number + ticks) % 10;
            }
        }
    }

    /// Will return the current value of the number stored in Ticker.
    pub fn get_number(&self) -> u32 {
        self.number.unwrap_or(0)
    }

    /// Will return the current value of the number stored in Ticker as a string.
    pub fn get_string(&self) -> String {
        format!("{}", self.number.unwrap_or(0))
    }

    /// Pauses a timer within the ticker.
    pub fn pause(&mut self) {
        if let Some(timer) = &mut self.timer {
            // This .pause is Bevy's pause method for their timers, this isn't a recursive action.
            timer.pause();
        }
    }

    /// Unpauses a timer within a ticker.
    pub fn unpause(&mut self) {
        if let Some(timer) = &mut self.timer {
            // This .unpause is Bevy's unpause method for their timers, this isn't a recursive action.
            timer.unpause();
        }
    }
}
