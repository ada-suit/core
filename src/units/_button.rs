/* units/_buttons.rs */

use super::input::*;
use crate::config::details::{BUTTON_PINS, BUTTON_COUNT};

pub type Button = Input<BUTTON_COUNT>;

impl InBase<BUTTON_COUNT> for Button {
    const PINS: [u32; BUTTON_COUNT] = BUTTON_PINS;
    const ID: &'static str = "Buttons";

    fn update(&mut self, counter: &u32) {
        std_update(self, counter);
    }
}

