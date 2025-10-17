use anyhow::Result;
use clap::Parser;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::Deserialize;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about = "Minimal Frigate CLI notifier")]
struct Args {
    #[arg(long, default_value = "localhost")] broker_host: String,
    #[arg(long, default_value_t = 1883)] broker_port: u16,
    #[arg(long, default_value = "frigate/events")] topic: String,
}

#[derive(Debug, Deserialize)]
struct FrigateEvent {
    id: String,
    camera: Option<String>,
    label: Option<String>,
    #[serde(rename = "type")] kind: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let mut mqttoptions = MqttOptions::new("frigate_cli", &args.broker_host, args.broker_port);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    mqttoptions.set_max_packet_size(1024 * 1024, 1024 * 1024); // 1MB max packet size for large payloads

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(&args.topic, QoS::AtLeastOnce).await?;

    println!("subscribed to {}:{} topic {}", args.broker_host, args.broker_port, args.topic);
    
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                if let Ok(ev) = serde_json::from_slice::<FrigateEvent>(&p.payload) {
                    println!(
                        "{}: {} on {} (id={})",
                        ev.kind.as_deref().unwrap_or("event"),
                        ev.label.as_deref().unwrap_or("object"),
                        ev.camera.as_deref().unwrap_or("camera"),
                        ev.id
                    );
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("mqtt error: {e}");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}