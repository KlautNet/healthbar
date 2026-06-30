use std::time::{Duration, Instant};

use crate::model::{Check, Status, Target};

pub async fn check_target(client: &reqwest::Client, target: &Target) -> (Status, Option<String>) {
    let timeout = Duration::from_millis(target.timeout_ms);
    match &target.check {
        Check::Http {
            url,
            expected_status,
        } => {
            let started = Instant::now();
            match client.get(url).timeout(timeout).send().await {
                Ok(resp) => {
                    if let Some(expected) = expected_status {
                        if resp.status().as_u16() != *expected {
                            return (Status::Degraded, Some("Unexpected status".into()));
                        }
                    }
                    if let Some(degraded_ms) = target.degraded_ms {
                        if started.elapsed().as_millis() as u64 > degraded_ms {
                            return (Status::Degraded, Some("Slow response".into()));
                        }
                    }
                    (Status::Up, None)
                }
                Err(_) => (Status::Down, Some("Request failed".into())),
            }
        }
        Check::Tcp { host, port } => {
            let addr = format!("{}:{}", host, port);
            let connect = tokio::net::TcpStream::connect(addr);
            match tokio::time::timeout(timeout, connect).await {
                Ok(Ok(_)) => (Status::Up, None),
                Ok(Err(_)) => (Status::Down, Some("Connection failed".into())),
                Err(_) => (Status::Down, Some("Connection timed out".into())),
            }
        }
    }
}

#[tauri::command]
pub async fn get_target_status(target: Target) -> Result<Status, String> {
    let client = reqwest::Client::new();
    let (status, _message) = check_target(&client, &target).await;
    Ok(status)
}
