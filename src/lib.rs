//! Simulate the growth of ferns, from the level of
//! individual cells on up

extern crate core;

mod types;
mod ownership;
mod reference;
mod expression;
mod errorhandler;
mod structure;
mod enums;
mod traits;

pub mod plant_structures;

/// Fern structure
pub struct Fern {
    pub size: f64,
    pub growth_rate: f64
}

impl Fern {
    // function for Fern structure

    fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate
    }
}

/// Running Fern simulation in number of days
pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}

// fn main() {
//     let mut fern = Fern {
//         size: 1.0,
//         growth_rate: 0.001
//     };
//
//     run_simulation(&mut fern, 1000);
//     println!("final fern size: {}", fern.size);
// }