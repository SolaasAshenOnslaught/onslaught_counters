
// Imports
use half::f16;
use onslaught_counters::{Ticker, TickerBehaviors, TickerPrecision};

fn main() {

    // CUSTOMIZE YOUR TICKER
    // Change the fields to whatever you want to test any kind of ticker.
    // Remember, the mut keyword is what makes ticking work.  To handle mutability of values, change the behavior.
    let mut ticker: Ticker<i8, f16> = Ticker::new(
        0,
        5,
        100,
        1.0,
        false,
        true,
        true,
        TickerBehaviors::Looper,
    );

    loop {
        // Printing out all the values within each field of the ticker.
        ticker.print_information();

        // 1.0 / 60.0 == Loop 60 Times Per 1 Second
        // Change the calculation to w/e you want for testing.
        ticker.tick(TickerPrecision::from_f64(1.0 / 60.0));
    }
}
