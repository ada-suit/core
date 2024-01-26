// units/output.rs

use gpiod;
use crate::counter;
use super::pulse::{Pulse, Pace, pace_value};

pub struct Structure<const COUNT: usize> {
    pub line:  gpiod::Lines<gpiod::Output>,
    pub sleep: [u32; COUNT],
    pub blink: [Pulse; COUNT]
}

pub trait Interface<const COUNT: usize> {
    const PINS: [u8; COUNT];
    const ID: &'static str;

    fn init(chip: &gpiod::Chip) -> Structure<COUNT>;
    fn update(&mut self, counter: &u32);
    fn blink(&mut self, id: usize, duration: u8, pace: Pace);
}

/*
 * Default Functions:
 *   defining default implemetation outside the trait as a temporary solution.
 *   need to find something better that works; need to find a way to define
 *   functionality within the trait definition itself.
 * */
fn default_init<const COUNT: usize>(
    chip: &gpiod::Chip, 
    pins: [u8; COUNT],
    id: &str
) -> Structure<COUNT> {
    let sleep_status: [u32; COUNT] = [0; COUNT]; 

    let blink_status: [Pulse; COUNT] = [
        Pulse { pace: 0, count: 0 }; 
        COUNT
    ];

    let options = gpiod::Options::output(pins)
        .consumer(id);

    let connection_line = chip
        .request_lines(options)
        .expect("Failed to initialize {id}");
    
    return Structure {
        line: connection_line,
        sleep: sleep_status,
        blink: blink_status
    };
}

fn default_update<const COUNT: usize>(counter: &u32) {
            let mut values = [None; COUNT];
        for i in 0..COUNT {
            if *counter ==  self.sleep[i] {
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

fn default_blink(id: usize, duration: u8, pace: Pace) {
    // Implementation for blink method
}

