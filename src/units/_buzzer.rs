// units/_buzzer.rs

pub use super::output::{Structure, Interface};
use crate::config::details::{BUZZER_PINS, BUZZER_COUNT};

pub type Buzzer = Structure<BUZZER_COUNT>;

impl Interface<BUZZER_COUNT> for Buzzer {
    const PINS: [u8; BUZZER_COUNT] = BUZZER_PINS;
    const ID: &'static str = "Buzzers";
}

