use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use dialoguer::{theme::ColorfulTheme, Input};

use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("172.105.66.226:8080").await?;

    let result = stream.write(b"shell\n").await;
    println!("Connected as shell..; success={:?}", result.is_ok());

    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What command do you want to send?")
        .interact_text()
        .unwrap();

    let input = input + "\n";
    let _ = stream.write(input.as_bytes()).await;

    Ok(())
}
