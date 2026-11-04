use crate::utils::*;

fn ellipsis(ticks: usize) {
    print!("{}{}", Up(1), Save);
    for t in 0..ticks {
        let t = t%4;
        println!("{}{}", Restore, ".".repeat(t) + &" ".repeat(3-t));
        sleep(250);
    }
    println!("{}...", Restore);
}

pub fn startup_sequence() {
    let debug =   |msg: &str| format!("{}  {}{}  Debug{}{} {msg}", Column(1), Magenta,  Bold, NotBold, DefaultColor);
    let info =    |msg: &str| format!("{}  {}{}   Info{}{} {msg}", Column(1), Cyan,     Bold, NotBold, DefaultColor);
    let warning = |msg: &str| format!("{}  {}{}Warning{}{} {msg}", Column(1), Yellow,   Bold, NotBold, DefaultColor);
    let error =   |msg: &str| format!("{}  {}{}  Error{}{} {msg}", Column(1), Red,      Bold, NotBold, DefaultColor);

    // I have no idea if any of this is even remotely realistic
    println!("{}Thank you for using Recon-1986-emulator!", Column(1));
    println!("{}This is version 3 of the emulator, you can view the changelog at https://reconemulator.net/v3/changelog", Column(1));
    println!("{}Starting emulator with logging set to {}{}Debug{}{}", Column(1), Magenta, Bold, NotBold, DefaultColor);
    println!("{}", info("Initializing 4kB of virtual ram"));
    ellipsis(9);
    println!("{}", info("Loading program ROM"));
    ellipsis(8);
    println!("{}", warning("Map data may have been corrupted or lost when loading"));
    sleep(250);
    println!("{}", debug("Failed to write over data in ROM at address $2000-$2FFF (map data)"));
    sleep(250);
    println!("{}", debug("Possible faulty initial RAM decay pattern"));
    sleep(250);
    println!("{}", info("Initializing virtual display"));
    ellipsis(6);
    println!("{}", info("Initializing virtual controls panel"));
    ellipsis(3);
    println!("{}", info("Initializing audio pipe"));
    ellipsis(5);
    println!("{}", error("Compatible audio driver not found, skipping"));
    println!("{}", info("Finishing setup"));
    ellipsis(9);
    println!("{}", error("Failed to read byte $3000 (forgotten), skipping"));
    print!("{}{}", debug("3000"), Save);
    for _ in 0..3000 {
        println!("{}3000{}", Restore, Save);
        sleep_micros(50);
    }
}