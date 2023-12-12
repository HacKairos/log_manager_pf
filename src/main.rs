use chrono::prelude::*;
use reqwest;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead};
use tokio::time::{sleep, Duration};

async fn send_log_to_endpoint(log: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://your-endpoint.com")
        .body(log.to_string())
        .send()
        .await?;

    println!("Response: {}", res.status());

    Ok(())
}

#[tokio::main]
async fn main() {
    loop {
        let stdin = io::stdin();

        let dt = Local::now();
        let filename = format!("output_{}.txt", dt.format("%Y-%m-%d"));

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&filename)
            .unwrap();

        for line in stdin.lock().lines() {
            let line = line.unwrap();
            writeln!(file, "{}", line).unwrap();

            // Enviar log para o endpoint
            let _ = send_log_to_endpoint(&line).await;
        }

        // Aguarde um pouco antes de verificar novos logs
        sleep(Duration::from_secs(3)).await;
    }
}