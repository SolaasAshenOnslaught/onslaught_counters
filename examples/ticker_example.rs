
// Imports
use half::f16;
use lcc_counters::{Ticker, TickerBehavior, TickerPrecision};

fn main() {

    // CUSTOMIZE YOUR TICKER
    // Change the fields to whatever you want to test any kind of ticker.
    // Remember to declare mutability if you want to make use of methods that change the ticker's
    // fields, or you can make use of the copy constructors following this ticker instantiation.
    let mut ticker: Ticker<i8, f16> = Ticker::new(
        0,
        5,
        100,
        1.0,
        false,
        true,
        true,
        TickerBehavior::Looper,
    );

    loop {
        // Printing out all the values within each field of the ticker.
        ticker.print_information();

        // 1.0 / 60.0 == Loop 60 Times Per 1 Second
        // Change the calculation to w/e you want for testing.
        ticker.tick(TickerPrecision::from_f64(1.0 / 60.0));
    }
}
