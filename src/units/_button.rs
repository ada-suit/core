use gpiod;
use super::unit;
use crate::config::details::{BUTTON_PINS, BUTTON_COUNT};

const PIN_COUNT: usize = 5;

pub struct Button {
    line: gpiod::Lines<gpiod::Input>,
    sleep: [u32; PIN_COUNT],
    pub call: u16,
}

impl unit::Input for Button {
    // Initialize and return a vector of buttons
    fn init(chip: &gpiod::Chip) -> Button {
        let pins: [u32; PIN_COUNT] = [
            25, // 0  terminate the program
            17, // 1  ports forwarded for virtual buttons
            27, // 2  ports forwarded for virtual buttons
            22, // 3  ports forwarded for virtual buttons
            // ...add new buttons here
            24  // 4  toggle shift mode
        ];

        let sleep_status: [u32; PIN_COUNT] = Default::default();

        let options = gpiod::Options::input(pins)
            .consumer("buttons");

        let connection_line = chip
            .request_lines(options)
            .expect("failed");

        return Button {
            line: connection_line, 
            sleep: sleep_status,
            call: 0,
        };
    }

    fn update(&mut self, counter: &u32) {
        if 0 == self.sleep[0] {
            println!("{counter}");
        }
    }
}

