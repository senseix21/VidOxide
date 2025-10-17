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
#[allow(dead_code)]  // Suppress unused field warning
struct FrigateEventWrapper {
    #[serde(rename = "type")]
    event_type: Option<String>,
    before: Option<FrigateEventData>,
    after: FrigateEventData,
}

#[derive(Debug, Deserialize)]
struct FrigateEventData {
    id: String,
    camera: Option<String>,
    label: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let mut mqttoptions = MqttOptions::new("frigate_cli", &args.broker_host, args.broker_port);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    // Don't set max packet size - let broker decide
    
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(&args.topic, QoS::AtLeastOnce).await?;

    println!("subscribed to {}:{} topic {}", args.broker_host, args.broker_port, args.topic);
    
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                // Try to parse the event wrapper
                if let Ok(wrapper) = serde_json::from_slice::<FrigateEventWrapper>(&p.payload) {
                    let ev = &wrapper.after;
                    println!(
                        "{}: {} on {} (id={})",
                        wrapper.event_type.as_deref().unwrap_or("event"),
                        ev.label.as_deref().unwrap_or("object"),
                        ev.camera.as_deref().unwrap_or("camera"),
                        ev.id
                    );
                }
                // Silently skip unparseable messages
            }
            Ok(_) => {}  // Ignore other packet types
            Err(e) => {
                eprintln!("mqtt error: {e}");
                // Wait longer before retry to avoid reconnection storm
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}