/* units/pulse.rs 
 *
 * defines beep/blink functionality for output components, with also specifying
 * a few speed options, from `Still` (i.e. no blink), to very `Rapid` blink.
 * */

// defines different paces/speeds for pulse/blink 
#[derive(Debug, Copy, Clone)]
pub enum Pace {
    Rapid,
    Fast,
    Moderate,
    Slow,
    Torpid,
    Still,
}

// assigns numerical values to different paces 
pub fn pace_value(pace: Pace) -> u32 {
    match pace {
        Pace::Rapid    => 500,
        Pace::Fast     => 1000,
        Pace::Moderate => 2500,
        Pace::Slow     => 5000,
        Pace::Torpid   => 50000,
        Pace::Still    => 0,
    }
}

// basic pulse structure, storing essential info
#[derive(Debug, Copy, Clone)]
pub struct Pulse {
    pub pace: u32, // speed 
    pub count: u8, // number of times to pulse 
}

