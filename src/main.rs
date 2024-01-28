mod units;
mod trigger;
mod counter;
mod config;

use gpiod::Chip;
use config::details::CHIP;
use units::{Led, Button, Buzzer, OutBase, InBase}; 

fn main() {
    let mut counter: u32  = 1;
    let mut run    : bool = true;
    let mut shift  : bool = false;

    // initialise all components
    let chip = Chip::new(CHIP)
        .expect("Failed to open GPIO chip");
    
    let mut leds    = Led::init(&chip);
    let mut buttons = Button::init(&chip);
    let mut buzzers = Buzzer::init(&chip);
    
    while run {
        // update
        leds.update(&counter);
        buttons.update(&counter);
        buzzers.update(&counter);

        // actions
        trigger::auto(&counter, &leds, &buzzers);
        trigger::call(&mut shift, buttons.call);

        // counter update
        counter::update(&mut counter);

        // if program must keep running
        run = trigger::keep_running(&buttons);
    }
}

