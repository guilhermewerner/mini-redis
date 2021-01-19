#![allow(non_snake_case)]

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = [1; 10];
    let mut file = File::open("README.md").await?;

    // read up to 10 bytes
    let n = file.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);

    Ok(())
}
