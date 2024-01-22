use gpiod;
use super::unit;
use crate::counter;
use crate::config::details::{LED_PINS, LED_COUNT};

const PIN_COUNT: usize = 3;

pub struct Led {
    line: gpiod::Lines<gpiod::Output>,
    sleep: [u32; PIN_COUNT],
    blink: [unit::Pulse; PIN_COUNT],
}

impl unit::Output for Led {
    // Initialize and return a vector of LEDs
    fn init(chip: &gpiod::Chip) -> Led {
        let pins: [u32; PIN_COUNT] = [
            26, // 0  indicate power stability
            21, // 1  indicate internet connectivity
            23  // 2  indicate shift status
            // ...add more LEDs here
        ];

        let sleep_status: [u32; PIN_COUNT] = Default::default(); 
        let blink_status: [unit::Pulse; PIN_COUNT] = [
            unit::Pulse {pace: 0, count: 0,}; 
            PIN_COUNT
        ];

        let options = gpiod::Options::output(pins)
            .consumer("LEDs");

        let connection_line = chip
            .request_lines(options)
            .expect("failed to Initialize leds");

        return Led { 
            line: connection_line,
            sleep: sleep_status,
            blink: blink_status
        };
    }

    fn update(&mut self, counter: &u32) {
        let mut values = [None; PIN_COUNT];
        for i in 0..PIN_COUNT {
            if *counter == self.sleep[i] {
                // reset the sleep counter 
                self.sleep[i] = {
                    let next_val = counter::next(counter, &self.blink[i].pace);
                    let to_reset = (self.blink[i].count != 0) as u32;
                    next_val * to_reset 
                };

                // value = False when even blink, else True
                values[i] = Some(self.blink[i].count % 2 == 1);

                // update blink counter
                self.blink[i].count -= (self.blink[i].count != 0) as u8;
            }
        }
        self.line.set_values(values);
    }
}

impl Led {
    pub fn blink(&mut self, id: usize, duration: u8, pace: unit::Pace) {
        self.blink[id].count = duration * 2;
        self.blink[id].pace  = unit::pace_value(pace);
    }
}

