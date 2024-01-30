/* units/_led.rs */

use super::output::*;
use crate::config::{details::{LED_PINS, LED_COUNT}, purpose::led_match};
use super::pulse::Pace;

pub type Led = Output<LED_COUNT>;

impl OutBase<LED_COUNT> for Led {
    const PINS: [u32; LED_COUNT] = LED_PINS;
    const ID: &'static str = "Leds";

    fn update(&mut self, counter: &u32) {
        std_update(self, counter);
    }

    fn blink(&mut self, id: &str, duration: u8, pace: Pace) {
        std_blink(self, led_match(id), duration, pace);
    }

    fn set(&mut self, id: &str, status: bool) {
        std_set(self, led_match(id), status);
    }
}

