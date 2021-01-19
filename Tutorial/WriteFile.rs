#![allow(non_snake_case)]

use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = File::create("File.txt").await?;

    buffer.write_all(b"some bytes").await?;

    Ok(())
}
