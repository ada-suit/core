/* trigger.rs 
 * 
 * defines actions to be triggered; either automatically or through user call.
 * */

use super::system;
use crate::units::{Led, Button, Buzzer, OutBase, InBase}; 

// background processes
// cmds that are triggered every cycle; required by the program
pub fn auto(counter: &u32, leds: &mut Led, _buzzers: &mut Buzzer) {
    if 1 == *counter {
        leds.set("POWER", system::power_stability());
    }
}

// user requested actions
// actions that were triggered by user's explicit action 
pub fn call(shift: &mut bool, cmd: u8) {
    let status: u8 = cmd << (*shift as u8);

    match status {
        // ======== BUTTON 1 ======== //
        0b0010 => {
            *shift = true;
            // turn led off
        },

        0b0011 => {
            *shift = false;
            // turn led on
        },

        // ======== BUTTON 2 ======== //
        0b0100 => { },

        0b0101 => { },


        // ======== BUTTON 3 ======== //
        0b0110 => { },

        0b0111 => { },


        // ======== BUTTON 4 ======== //
        0b1000 => { },

        0b1001 => { },


        // ======== BUTTON 5 ======== //
        0b1010 => { },

        0b1011 => { },


        // ======== BUTTON 6 ======== //
        0b1100 => { },

        0b1101 => { },


        // ======== BUTTON 7 ======== //
        0b1110 => { },

        0b1111 => { },

        // ========================= //
        _ => {}
    }
}

// confirm user hasn't requested to quit
pub fn keep_running(buttons: &Button) -> bool {
    unimplemented!();
}

