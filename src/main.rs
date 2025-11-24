mod cpu_features;
mod cpu_info;
mod display;

use std::io;
use std::io::prelude::*;

use anyhow::Result;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press enter to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() -> Result<()> {
    // Get CPU information
    let cpu_info = cpu_info::get_cpu_info()?;
    
    // Get CPU features
    let features = cpu_info::get_cpu_features()?;
    
    // Format and display the results
    let output = display::format_output(&cpu_info, &features)?;
    println!("{}", output);

    pause();
    Ok(())
}