extern crate core;

mod r#type;
mod ownership;
mod reference;
mod expression;
mod errorhandler;

pub mod plant_structures;

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