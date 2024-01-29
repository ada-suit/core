/* system.rs 
 *
 * basic interface to run and process system/shell commands.
 *
 * most functions return bool to indicate whether the condition they're 
 * monitoring is true or false
 */

use std::process::Command;

fn run_cmd(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("couldn't run {cmd} system command");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

pub fn power_stability() -> bool {
    return "throttled=0x0" == run_cmd("vcgencmd",  &["get_throttled"]);
}

