use std::{process::Command, time::Duration};
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // get the two arguments
    let args: Vec<String> = std::env::args().collect();
    let start_arg = "42".to_string();
    let end_arg = "1337".to_string();
    let start = args.get(0).unwrap_or(&start_arg);
    let end = args.get(1).unwrap_or(&end_arg);

    let mut ban_count = 0;
    for _ in 0..5 {
        // Command
        let mut sum = 0;
        for i in 0..4 {
            let mut cmd = Command::new("node");

            cmd
                .arg("./src/index.js")
                .arg(start.clone())
                .arg(end.clone());

            // read stdout
            let output = cmd.output().context("Failed to execute command")?;
            let output = String::from_utf8(output.stdout)?.lines().take(1).collect::<String>().parse::<i32>().context("Failed to parse output")?;

            println!("lucky number {}: {}", i, output);
            sum += output;
        }

        println!("your sum is {}", sum);
        ban_count += if sum % 2 == 0 { 1 } else { 0 };
    }
    println!("you are...");
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("ban_count {}", ban_count);
    return Ok(());
}
