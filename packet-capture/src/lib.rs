use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn capture() -> Result<()> {
    println!("packet capture");
    Ok(())
}
