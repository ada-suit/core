/* units/pulse.rs 
 *
 * defines beep/blink functionality for output components, with also specifying
 * a few speed options, from `Still` (i.e. no blink), to very `Rapid` blink.
 * */

// defines different paces/speeds for pulse/blink 
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Default)]
pub enum Pace {
    Rapid    = 500,
    Fast     = 1000,
    Moderate = 2500,
    Slow     = 5000,
    Torpid   = 50000,
    #[default]
    Still    = 0,
}

// basic pulse structure, storing essential info
#[derive(Debug, Copy, Clone, Default)]
pub struct Pulse {
    pub pace: u32, // speed 
    pub count: u8, // number of times to pulse 
}

