// units/_buttons.rs

use super::input::{Structure, Interface};
use crate::config::details::{BUTTON_PINS, BUTTON_COUNT};

pub type Button = Structure<BUTTON_COUNT>;

impl Interface<BUTTON_COUNT> for Button {
    const PINS: [u8; BUTTON_COUNT] = BUTTON_PINS;
    const ID: &'static str = "Buttons";
}

