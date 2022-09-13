use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use dialoguer::{theme::ColorfulTheme, Input, FuzzySelect};

use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("172.105.66.226:8080").await?;

    let result = stream.write(b"shell\n").await;
    println!("Connected as shell..; success={:?}", result.is_ok());

    let selections = get_selections();

    loop {
        let input = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("What command do you want to send?")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        let input = selections[input].to_string() + "\n";
        let _ = stream.write(input.as_bytes()).await;
    }
}

fn get_selections() -> &'static [&'static str; 10] {
    let selections = &[
        "shutdown",
        "reboot",
        "00",
        "01",
        "10",
        "11",
        "20",
        "21",
        "30",
        "31",
    ];
    selections
}
