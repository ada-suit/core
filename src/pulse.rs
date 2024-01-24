// pulse.rs

#[derive(Debug, Copy, Clone)]
pub enum Pace {
    Rapid,
    Fast,
    Moderate,
    Slow,
    Torpid,
    Still,
}

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

#[derive(Debug, Copy, Clone)]
pub struct Pulse {
    pub pace: u32,
    pub count: u8,
}

