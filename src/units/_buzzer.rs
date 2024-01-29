/* units/_buzzer.rs */

use super::output::*;
use crate::config::{details::{BUZZER_PINS, BUZZER_COUNT}, purpose::buzzer_match};
use super::pulse::Pace;

pub type Buzzer = Output<BUZZER_COUNT>;

impl OutBase<BUZZER_COUNT> for Buzzer {
    const PINS: [u32; BUZZER_COUNT] = BUZZER_PINS;
    const ID: &'static str = "Buzzers";
    
    fn update(&mut self, counter: &u32) {
        std_update(self, counter);
    }

    fn blink(&mut self, id: usize, duration: u8, pace: Pace) {
        std_blink(self, id, duration, pace);
    }

    fn set(&mut self, id: &str, status: bool) {
        std_set(self, buzzer_match(id), status);
    }
}

