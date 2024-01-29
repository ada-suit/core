/* counter.rs 
 * 
 * to define counter's functionality. ensures it loops within allowed range,
 * and no process makes it jump out of that range.
 * */

// increment counter 
// either increment its value by 1 or reset it back to 1
pub fn update(counter: &mut u32)  {
    *counter = 1 + (*counter * (*counter == u32::MAX) as u32);
    /*
     * The counter is intentionally designed to never reach 0. This ensures
     * that units and processes can halt without transitioning to the state 0.
     * In other words, functionality resumes only when the counter is 0, which
     * it never is intended to be.
     */
}

// next instance
// return wrapped counter value for after the specified span
pub fn next(counter: &u32, span: &u32) -> u32 {
    return 1 + (counter + span) % u32::MAX;
}

