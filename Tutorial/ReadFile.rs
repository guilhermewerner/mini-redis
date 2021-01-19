#![allow(non_snake_case)]

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = Vec::new();
    let mut file = File::open("README.md").await?;

    // read the whole file
    file.read_to_end(&mut buffer).await?;

    println!("The bytes: {:?}", &buffer);

    Ok(())
}
