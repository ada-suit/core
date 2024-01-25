// units/_led.rs

use super::output::{Structure, Interface};
use crate::config::details::{LED_PINS, LED_COUNT};

pub type Led = Structure<LED_COUNT>;

impl Interface<LED_COUNT> for Led {
    const PINS: [u8; LED_COUNT] = LED_PINS;
    const ID: &'static str = "Leds";
}

