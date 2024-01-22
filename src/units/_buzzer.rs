use gpiod;
use super::unit;
use crate::config::details::{BUZZER_PINS, BUZZER_COUNT};

const PIN_COUNT: usize = 1;

pub struct Buzzer {
    line: gpiod::Lines<gpiod::Output>,
    sleep: [u32; PIN_COUNT],
}

    // Initialize and return a vector of buzzers
impl unit::Output for Buzzer {
    fn init(chip: &gpiod::Chip) -> Buzzer {
        let pins: [u32; PIN_COUNT] = [
            13  // one pin
        ];

        let sleep_status: [u32; PIN_COUNT] = Default::default();

        let options = gpiod::Options::output(pins)
            .consumer("Buzzers");

        let connection_line = chip
            .request_lines(options)
            .expect("failed to Initialize buzzers");

        return Buzzer { line: connection_line, sleep: sleep_status };
    }

    fn update(&mut self, counter: &u32) {
        // Update logic for buzzer
    }
}

