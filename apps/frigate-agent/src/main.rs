mod config; mod mqtt; mod web;
use config::Config;
use serde::Deserialize;
use tokio::{task, time::{sleep, Duration}};
use tracing::{info, error};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
struct FrigateEvent { id: String, #[allow(dead_code)] #[serde(rename="type")] kind: Option<String> }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    let cfg = Config::from_env();
    info!(?cfg, "agent starting");

    // Start health server
    let web = web::app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let _web_task = task::spawn(async move {
        axum::serve(listener, web).await.unwrap();
    });

    // MQTT loop
    let (mut client, mut conn) = mqtt::connect(&cfg.broker_host, cfg.broker_port, "frigate_agent");
    mqtt::subscribe(&mut client, &cfg.topic);
    info!(topic = %cfg.topic, "subscribed");

    loop {
        if let Some(payload) = mqtt::next_publish(&mut conn) {
            match serde_json::from_slice::<FrigateEvent>(&payload) {
                Ok(ev) => info!(id=%ev.id, "event"),
                Err(e) => error!(error=?e, "bad payload"),
            }
        } else {
            sleep(Duration::from_millis(50)).await;
        }
    }
}