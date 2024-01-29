/* units/input.rs 
 *
 * defines common structure and functions for all input components, with
 * standard implementation for different all functions.
 * */

use gpiod;
use crate::counter;

// common structure with the most essential fields
pub struct Input<const COUNT: usize> {
    line:  gpiod::Lines<gpiod::Input>,
    sleep: [u32;COUNT],
    pub call: u16,
}

// trait defining functions essential for all input components.
// got implementation for default init()
pub trait InBase<const COUNT: usize>{
    const PINS: [u32; COUNT];
    const ID: &'static str;

    // Initialize and return a vector of buttons
    fn init(chip: &gpiod::Chip) -> Input<COUNT> {
        let sleep_status: [u32; COUNT] = [0; COUNT];

        let options = gpiod::Options::input(Self::PINS)
            .consumer("{Self::ID}");

        let connection_line = chip
            .request_lines(options)
            .expect("failed");

        return Input {
            line: connection_line, 
            sleep: sleep_status,
            call: 0,
        };
    }

    fn update(&mut self, counter: &u32);
}

/* std_functions()
 * 
 * given `self` keyword in a trait redirects to the trait itself, default 
 * implementation could not be defined within the trait for functions that
 * relied on `self`.
 *
 * with this, option1 was to have functions that require itself as parameter,
 * like: `button.update(&mut button);` which is... just not very ideal,instead,
 * with defining these functions outside the trait, `button.update()` is 
 * possible.
 *
 * furthermore, with this approach, custom, i.e. unit specific, implementations
 * could be built on top of the standard ones. they could be called with custom 
 * parameters specific to the need, instead of essentially rewriting the whole
 * code again with slight changes.
 */

// standard update: should be sufficient for most needs
pub fn std_update<const COUNT: usize>(
    unit: &mut Input<COUNT>, 
    counter: &u32
) {
    unimplemented!();
}

