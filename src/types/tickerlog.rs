// 
// // Imports
// use bevy::prelude::*;
// use crate::types::ticker::Ticker;
// 
// /// Used to create timers that can optionally store digits from the hundreds place to the thousandths
// /// place.  Digits can be preset by assigning values to digit properties and timers for each digit
// /// can be assigned independently for all the insanity that comes with declaring fancy clocks.
// ///
// /// Digits are declared with the datatype u32 and not u8 because the tick system is set up
// /// to handle frame spikes.  Frame spikes with timers inside Bevy can be dealt with using the times_finished_this_tick()
// /// method on a timer and counting the number of ticks that occurred during the time it took to pass through the
// /// delta happening mid-spike.  I'm being a bit insane by applying them to all digits.  Realistically speaking,
// /// applying u32 to the thousandth and hundredth place is absurd over going with the just_finished() option that Bevy provides on timers
// /// (nobody is gonna have a 10+ or 100+ second frame spike).  But uh...  CODE SYMMETRY (AKA OCD) CALLS FOR INEFFICIENCY!
// ///
// /// By default, Tickers within a Chronolog have nothing in them.
// /// By using the new method, Tickers will count up and repeat once they hit their max value.
// /// You can still do countdown logic with tickers moving up in time, just takes a little more work.  But
// /// if you'd like you can give a default Ticker custom timers to do countdown effects more intuitively.
// #[derive(Component, Reflect, Debug)]
// pub struct Chronolog {
//     pub start_value:                    Option<f32>,
//     pub ticker_for_hundred_thousands:   Option<Ticker>,
//     pub ticker_for_ten_thousands:       Option<Ticker>,
//     pub ticker_for_thousands:           Option<Ticker>,
//     pub ticker_for_hundreds:            Option<Ticker>,
//     pub ticker_for_tens:                Option<Ticker>,
//     pub ticker_for_ones:                Option<Ticker>,
//     pub ticker_for_tenths:              Option<Ticker>,
//     pub ticker_for_hundredths:          Option<Ticker>,
// }
// 
// impl Default for Chronolog {
//     fn default() -> Self {
//         Self {
//             start_value:                    Some(0.0),
//             ticker_for_hundred_thousands:   Some(Ticker::default()),
//             ticker_for_ten_thousands:       Some(Ticker::default()),
//             ticker_for_thousands:           Some(Ticker::default()),
//             ticker_for_hundreds:            Some(Ticker::default()),
//             ticker_for_tens:                Some(Ticker::default()),
//             ticker_for_ones:                Some(Ticker::default()),
//             ticker_for_tenths:              Some(Ticker::default()),
//             ticker_for_hundredths:          Some(Ticker::default()),
//         }
//     }
// }
// 
// impl Chronolog {
// 
//     /// Requires an Option to be thrown into it for usage, the Option may be filled or None.
//     ///
//     /// Passing in None will set the start_value to 0.0.
//     /// Passing in Some(INSERT_FLOATING_POINT_HERE) will set the start_value to INSERT_FLOATING_POINT_HERE.
//     ///
//     /// Does not work with negatives, so don't even try unless you'd like to cry like I have.
//     pub fn new(starting_value: Option<f32>) -> Self {
//         Self {
// 
//             start_value: Some(starting_value.unwrap_or(0.0)),
// 
//             ticker_for_hundred_thousands: Some(Ticker{
//                 number: Some(0),
//                 timer: Some(Timer::from_seconds(100000.0, TimerMode::Repeating)),
//             }),
// 
//             ticker_for_ten_thousands: Some(Ticker{
//                 number: Some(0),
//                 timer: Some(Timer::from_seconds(10000.0, TimerMode::Repeating)),
//             }),
// 
//             ticker_for_thousands: Some(Ticker{
//                 number: Some(0),
//                 timer: Some(Timer::from_seconds(1000.0, TimerMode::Repeating)),
//             }),
// 
//             ticker_for_hundreds: Some(Ticker{
//                 number: Some(0),
//                 timer: Some(Timer::from_seconds(100.0, TimerMode::Repeating)),
//             }),
// 
//             ticker_for_tens: Some(Ticker{
//                 number: Some(0),
//                 timer: Some(Timer::from_seconds(10.0, TimerMode::Repeating)),
//             }),
// 
//             ticker_for_ones: Some(Ticker{
//                 number: Some(0),
//                 timer: Some(Timer::from_seconds(1.0, TimerMode::Repeating)),
//             }),
// 
//             ticker_for_tenths: Some(Ticker{
//                 number: Some(0),
//                 timer: Some(Timer::from_seconds(0.1, TimerMode::Repeating)),
//             }),
// 
//             ticker_for_hundredths: Some(Ticker{
//                 number: Some(0),
//                 timer: Some(Timer::from_seconds(0.01, TimerMode::Repeating)),
//             }),
//         }
//     }
// 
//     /// Returns how long a Chronolog has until it hits the zero value.
//     ///
//     /// The start_value of a Chronolog dictates the top of the countdown and the constant increasing
//     /// values of the Chronolog are reversed through subtraction in this method to create a countdown effect.
//     pub fn get_countdown_as_float(&self) -> f32 {
// 
//         let start = self.start_value.unwrap_or(0.0);
//         let elapsed = start - self.get_time_as_float();
// 
//         // Prevents the countdown number from returning a negative value.
//         // Will return the elapsed time if it's greater than 1.0.
//         // Will return 0.0 if the elapsed time comes out as negative.
//         if elapsed > 0.0 {
//             elapsed
//         }
//         else {
//             0.0
//         }
//     }
// 
//     /// Returns a string for the current countdown value, the number of digits is based on how many
//     /// whole places is desired and how many floating places is desired.
//     ///
//     /// It's important to note that a decimal is added into the string and should be accounted for
//     /// if you're gonna try and convert the string into an indexable structure.
//     pub fn get_countdown_as_string(
//         &self,
//         number_of_whole_places: usize,
//         number_of_floating_places: usize
//     ) -> String {
// 
//         let countdown = self.get_countdown_as_float();
//         let character_count = number_of_whole_places + number_of_floating_places + 1;
// 
//         // Left side of printout is dictated implicitly by (number_of_characters - floating).
//         format!("{:0>number_of_characters$.floating$}",
//                 countdown,
//                 number_of_characters = character_count,
//                 floating = number_of_floating_places
//         )
//     }
// 
//     /// Returns the number that's in the hundred thousands' ticker if the ticker exists.  Otherwise, None is returned.
//     pub fn get_number_for_hundred_thousands(&self) -> Option<u32> {
//         self.ticker_for_hundred_thousands
//             .as_ref()
//             .and_then(|ticker| ticker.number)
//     }
// 
//     /// Returns the number that's in the ten thousands' ticker if the ticker exists.  Otherwise, None is returned.
//     pub fn get_number_for_ten_thousands(&self) -> Option<u32> {
//         self.ticker_for_ten_thousands
//             .as_ref()
//             .and_then(|ticker| ticker.number)
//     }
// 
//     /// Returns the number that's in the thousands' ticker if the ticker exists.  Otherwise, None is returned.
//     pub fn get_number_for_thousands(&self) -> Option<u32> {
//         self.ticker_for_thousands
//             .as_ref()
//             .and_then(|ticker| ticker.number)
//     }
// 
//     /// Returns the number that's in the hundreds' ticker if the ticker exists.  Otherwise, None is returned.
//     pub fn get_number_for_hundreds(&self) -> Option<u32> {
//         self.ticker_for_hundreds
//             .as_ref()
//             .and_then(|ticker| ticker.number)
//     }
// 
//     /// Returns the number that's in the tens' ticker if the ticker exists.  Otherwise, None is returned.
//     pub fn get_number_for_tens(&self) -> Option<u32> {
//         self.ticker_for_tens
//             .as_ref()
//             .and_then(|ticker| ticker.number)
//     }
// 
//     /// Returns the number that's in the ones' ticker if the ticker exists.  Otherwise, None is returned.
//     pub fn get_number_for_ones(&self) -> Option<u32> {
//         self.ticker_for_ones
//             .as_ref()
//             .and_then(|ticker| ticker.number)
//     }
// 
//     /// Returns the number that's in the tenths' ticker if the ticker exists.  Otherwise, None is returned.
//     pub fn get_number_for_tenths(&self) -> Option<u32> {
//         self.ticker_for_tenths
//             .as_ref()
//             .and_then(|ticker| ticker.number)
//     }
// 
//     /// Returns the number that's in the hundredths' ticker if the ticker exists.  Otherwise, None is returned.
//     pub fn get_number_for_hundredths(&self) -> Option<u32> {
//         self.ticker_for_hundredths
//             .as_ref()
//             .and_then(|ticker| ticker.number)
//     }
// 
//     /// Returns the number that's in the hundred_thousands' ticker as a char if the ticker exists.  Otherwise, None is returned.
//     pub fn get_char_for_hundred_thousands(&self) -> Option<char> {
//         let hundred_thousands = self.ticker_for_hundred_thousands
//             .as_ref()
//             .and_then(|ticker| ticker.number);
// 
//         // Converting the number to a character.
//         char::from_digit(hundred_thousands?, 10)
//     }
// 
//     /// Returns the number that's in the ten_thousands' ticker as a char if the ticker exists.  Otherwise, None is returned.
//     pub fn get_char_for_ten_thousands(&self) -> Option<char> {
//         let ten_thousands = self.ticker_for_ten_thousands
//             .as_ref()
//             .and_then(|ticker| ticker.number);
// 
//         // Converting the number to a character.
//         char::from_digit(ten_thousands?, 10)
//     }
// 
//     /// Returns the number that's in the thousands' ticker as a char if the ticker exists.  Otherwise, None is returned.
//     pub fn get_char_for_thousands(&self) -> Option<char> {
//         let thousands = self.ticker_for_thousands
//             .as_ref()
//             .and_then(|ticker| ticker.number);
// 
//         // Converting the number to a character.
//         char::from_digit(thousands?, 10)
//     }
// 
//     /// Returns the number that's in the hundreds' ticker as a char if the ticker exists.  Otherwise, None is returned.
//     pub fn get_char_for_hundreds(&self) -> Option<char> {
//         let hundreds = self.ticker_for_hundreds
//             .as_ref()
//             .and_then(|ticker| ticker.number);
// 
//         // Converting the number to a character.
//         char::from_digit(hundreds?, 10)
//     }
// 
//     /// Returns the number that's in the tens' ticker as a char if the ticker exists.  Otherwise, None is returned.
//     pub fn get_char_for_tens(&self) -> Option<char> {
//         let tens = self.ticker_for_tens
//             .as_ref()
//             .and_then(|ticker| ticker.number);
// 
//         // Converting the number to a character.
//         char::from_digit(tens?, 10)
//     }
// 
//     /// Returns the number that's in the ones' ticker as a char if the ticker exists.  Otherwise, None is returned.
//     pub fn get_char_for_ones(&self) -> Option<char> {
//         let ones = self.ticker_for_ones
//             .as_ref()
//             .and_then(|ticker| ticker.number);
// 
//         // Converting the number to a character.
//         char::from_digit(ones?, 10)
//     }
// 
//     /// Returns the number that's in the tenths' ticker as a char if the ticker exists.  Otherwise, None is returned.
//     pub fn get_char_for_tenths(&self) -> Option<char> {
//         let tenths = self.ticker_for_tenths
//             .as_ref()
//             .and_then(|ticker| ticker.number);
// 
//         // Converting the number to a character.
//         char::from_digit(tenths?, 10)
//     }
// 
//     /// Returns the number that's in the hundredths' ticker as a char if the ticker exists.  Otherwise, None is returned.
//     pub fn get_char_for_hundredths(&self) -> Option<char> {
//         let hundredths = self.ticker_for_hundredths
//             .as_ref()
//             .and_then(|ticker| ticker.number);
// 
//         // Converting the number to a character.
//         char::from_digit(hundredths?, 10)
//     }
// 
//     /// Will return the current value of the Chronolog.  Any unused digits will be labeled as 0.
//     pub fn get_time_as_float(&self) -> f32 {
// 
//         let mut total_time: f32 = 0.0;
// 
//         // Closure function to flatten out Options.
//         // Doing this since tickers for each digit are optional and we want to return a clean f32.
//         let digit = |ticker: &Option<Ticker>| {
//             ticker
//                 .as_ref()
//                 .and_then(|tocker| tocker.number)
//                 .unwrap_or(0) as f32
//         };
// 
//         let hundred_thousands=  digit(&self.ticker_for_hundred_thousands)   * 100000.0;
//         let ten_thousands=      digit(&self.ticker_for_ten_thousands)       * 10000.0;
//         let thousands =         digit(&self.ticker_for_thousands)           * 1000.0;
//         let hundreds =          digit(&self.ticker_for_hundreds)            * 100.0;
//         let tens =              digit(&self.ticker_for_tens)                * 10.0;
//         let ones =              digit(&self.ticker_for_ones)                * 1.0;
//         let tenths =            digit(&self.ticker_for_tenths)              * 0.1;
//         let hundredths =        digit(&self.ticker_for_hundredths)          * 0.01;
// 
//         total_time += hundred_thousands;
//         total_time += ten_thousands;
//         total_time += thousands;
//         total_time += hundreds;
//         total_time += tens;
//         total_time += ones;
//         total_time += tenths;
//         total_time += hundredths;
// 
//         total_time
//     }
// 
//     /// Will return the current value of the Chronolog as a string.  Any unused digits will be labeled as 0.
//     pub fn get_time_as_string(&self) -> String {
// 
//         let mut total_time = String::new();
// 
//         // Closure function to flatten out Options.
//         // Doing this since tickers for each digit are optional and we want to return a clean f32.
//         // Will also convert numbers into characters at the end so that we can push them into total_time.
//         let digit = |ticker: &Option<Ticker>| {
//             ticker
//                 .as_ref()
//                 .and_then(|tocker| tocker.number)
//                 .and_then(|number| char::from_digit(number, 10))
//         };
// 
//         // Pushing char digits onto total_time.
//         if let Some(character_digit) = digit(&self.ticker_for_hundred_thousands) { total_time.push(character_digit); }
//         if let Some(character_digit) = digit(&self.ticker_for_ten_thousands)     { total_time.push(character_digit); }
//         if let Some(character_digit) = digit(&self.ticker_for_thousands)         { total_time.push(character_digit); }
//         if let Some(character_digit) = digit(&self.ticker_for_hundreds)          { total_time.push(character_digit); }
//         if let Some(character_digit) = digit(&self.ticker_for_tens)              { total_time.push(character_digit); }
//         if let Some(character_digit) = digit(&self.ticker_for_ones)              { total_time.push(character_digit); }
// 
//         // Only add a floating point and floating numbers if their tickers are present.
//         if digit(&self.ticker_for_tenths).is_some() || digit(&self.ticker_for_hundredths).is_some() {
//             total_time.push('.');
//             if let Some(character_digit) = digit(&self.ticker_for_tenths)        { total_time.push(character_digit); }
//             if let Some(character_digit) = digit(&self.ticker_for_hundredths)    { total_time.push(character_digit); }
//         }
// 
//         total_time
//     }
// 
//     /// Used to advance all the tickers inside a Chronolog.  Takes in a time.delta() call off
//     /// the Res<Time> resource that Bevy provides.
//     ///
//     /// If you're making a custom ticking system and have stripped out the ticking systems provided
//     /// in the systems of this plugin, then please note that you must run this each frame for time to move normally.
//     pub fn tick(&mut self, delta: std::time::Duration) {
//         if let Some(ticker) = &mut self.ticker_for_hundred_thousands  { ticker.tick(delta); }
//         if let Some(ticker) = &mut self.ticker_for_ten_thousands      { ticker.tick(delta); }
//         if let Some(ticker) = &mut self.ticker_for_thousands          { ticker.tick(delta); }
//         if let Some(ticker) = &mut self.ticker_for_hundreds           { ticker.tick(delta); }
//         if let Some(ticker) = &mut self.ticker_for_tens               { ticker.tick(delta); }
//         if let Some(ticker) = &mut self.ticker_for_ones               { ticker.tick(delta); }
//         if let Some(ticker) = &mut self.ticker_for_tenths             { ticker.tick(delta); }
//         if let Some(ticker) = &mut self.ticker_for_hundredths         { ticker.tick(delta); }
//     }
// 
//     /// Will cause for a Chronolog to reset its tickers to 0.0, will continue ticking afterwards.
//     pub fn reset(&mut self) {
//         *self = Chronolog::new(None);
//     }
// 
//     /// Will wipe out all the tickers in a Chronolog.  Can be used to create a blank slate of tickers.
//     pub fn blank(&mut self) {
//         *self = Chronolog::default();
//     }
// }
