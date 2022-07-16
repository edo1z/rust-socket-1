use std::error;
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn request(address: &str) -> Result<()> {
    println!("TCP CLIENT {}", address);
    Ok(())
}
