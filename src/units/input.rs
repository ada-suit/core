// src/units/input.rs

use gpiod;
use crate::counter;
use super::pulse::{Pulse, pace_value};

pub struct Structure<const COUNT: usize> {
    line:  gpiod::Lines<gpiod::Input>,
    sleep: [u32;COUNT],
    pub call: u16,
}

pub trait Interface<const COUNT: usize> {
    const PINS: [u8; COUNT];
    const ID: &'static str;

    // Initialize and return a vector of buttons
    fn init(chip: &gpiod::Chip) -> Structure<COUNT> {
        let sleep_status: [u32; COUNT] = [0; COUNT];

        let options = gpiod::Options::input(Self::PINS)
            .consumer("{Self::ID}");

        let connection_line = chip
            .request_lines(options)
            .expect("failed");

        return Structure {
            line: connection_line, 
            sleep: sleep_status,
            call: 0,
        };
    }

    fn update(&mut self, counter: &u32) {
        unimplemented!();
    }
}
