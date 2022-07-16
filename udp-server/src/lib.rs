use std::error;
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn serve(address: &str) -> Result<()> {
    println!("UDP SERVER {}", address);
    Ok(())
}
