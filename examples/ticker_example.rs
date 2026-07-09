
// Imports
use half::f16;
use std::thread::sleep;
use std::time::{Duration, Instant};
use onslaught_counters::{Ticker, TickerBehaviors, TickerPrecision};

fn main() {

    // CUSTOMIZE YOUR TICKER
    // Change the fields to whatever you want to test any kind of ticker.
    // Remember, the mut keyword is what makes ticking work.  To handle mutability of values, change the behavior.
    let mut ticker: Ticker<i8, f64> = Ticker::new(
        0,
        0,
        100,
        1.0,
        false,
        true,
        true,
        TickerBehaviors::Looper,
    );


    // TICKING
    // The below code is my way to run things by seconds (it's rough, it's just there to be a quick example),
    // feel free to use whatever calculation you want to test out the tick method.  Figure out your delta.
    let mut last_time = Instant::now();
    loop {
        // Printing out all the values within each field of the ticker.
        ticker.print_information();
        println!();

        // Compute time elapsed since last tick, then reset the reference point.
        let now = Instant::now();
        let delta_secs = (now - last_time).as_secs_f64();
        last_time = now;

        // Tick the ticker according to its set precision.
        ticker.tick(TickerPrecision::from_f64(delta_secs));

        sleep(Duration::from_millis(16)); // ~60 iterations/sec, mimics a frame rate
    }
}
