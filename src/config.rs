/* config.rs 
 *
 * only file to modify.
 *
 * should there be a need to modify pin numbers, unit purpose, chip name, or 
 * even to add/remove new units, modifying this file would be suffecient for 
 * these scenarious.
 *
 * however, ofcourse, adding new more hardware would also require adding 
 * commands for dealing with it elsewhere in the project.
 *
 * */

// defines all components with their pins 
pub mod details {

    // Chip
    pub const CHIP: &str = "gpiochip0";

    // LEDs
    pub const LED_COUNT: usize = 3;
    pub const LED_PINS: [u32; LED_COUNT] = [
        26, // 0  indicate power stability
        21, // 1  indicate internet connectivity
        23  // 2  indicate shift status
            // ...add more LEDs here
    ];

    // Buttons
    pub const BUTTON_COUNT: usize = 5;
    pub const BUTTON_PINS: [u32; BUTTON_COUNT] = [
        25, // 0  terminate the program
        17, // 1  ports forwarded for virtual buttons
        27, // 2  ports forwarded for virtual buttons
        22, // 3  ports forwarded for virtual buttons
            // ...add new buttons here
        24  // 4  toggle shift mode
    ];

    // Buzzers
    pub const BUZZER_COUNT: usize = 1;
    pub const BUZZER_PINS: [u32; BUZZER_COUNT] = [
        13  // one pin
    ];
}

// define purpose/id for all pins with their index in the above pin array
pub mod purpose {

    pub fn led_match(request: &str) -> usize {
        return match request {
            "POWER"   => 0, 
            "NETWORK" => 1,
            "SHIFT"   => 2,
            _ => panic!(),
        };
    }

    pub fn button_match(request: &str) -> usize {
        return match request {
            "OFF"    => 0, 
            "VB1"    => 1,
            "VB2"    => 2,
            "VB3"    => 3,
            "SHIFT"  => 4,
            _ => panic!(),
        };
    }

    pub fn buzzer_match(request: &str) -> usize {
        return match request {
            "ONE"    => 0,
            _ => panic!(),
        }
    }
}

