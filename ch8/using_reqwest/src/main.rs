use std::error::Error;

use reqwest;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.rust-lang.org";

    let body = reqwest::blocking::get(url)?.text()?;
    println!("body = {:?}", body);
    Ok(())
}
