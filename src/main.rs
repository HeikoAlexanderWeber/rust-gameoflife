#[macro_use]
extern crate log;
extern crate env_logger;

use std::io;

fn main() -> io::Result<()> {
    env_logger::init();
    info!("Program is running.");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    println!("Output: {}", buffer.trim());
    return Ok(());
}
