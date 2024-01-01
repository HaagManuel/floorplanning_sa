//! This module contains a few utilities to measure how long executing algorithms takes.

use std::time::*;

/// This function will measure how long it takes to execute the given lambda,
/// print the time and return the result of the lambda.
pub fn report_time<Out, F: FnOnce() -> Out>(name: &str, f: F) -> Out {
    let start = Instant::now();
    eprintln!("starting {}", name);
    let res = f();
    eprintln!("done {} - took: {}s", name, start.elapsed().as_secs_f64());
    eprintln!("");
    res
}

/// This function will measure how long it takes to execute the given lambda
/// and return a tuple of the result of the lambda and a duration object.
pub fn measure<Out, F: FnOnce() -> Out>(f: F) -> (Out, Duration) {
    let start = Instant::now();
    let res = f();
    (res, start.elapsed())
}

/// A struct to repeatedly measure the time passed since the timer was started
#[derive(Debug)]
pub struct Timer {
    start: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    /// Create and start a new `Timer`
    pub fn new() -> Timer {
        Timer { start: Instant::now() }
    }

    /// Reset the `Timer`
    pub fn restart(&mut self) {
        self.start = Instant::now();
    }

    /// Print the passed time in ms since the timer was started
    pub fn report_passed_ms(&self) {
        eprintln!("{}ms", self.start.elapsed().as_secs_f64() * 1000.0);
    }

    /// Return the number of ms passed since the timer was started as a `i64`
    pub fn get_passed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }

    /// Return the number of ms passed since the timer was started as a `std::time::Duration`
    pub fn get_passed(&self) -> Duration {
        self.start.elapsed()
    }
}
