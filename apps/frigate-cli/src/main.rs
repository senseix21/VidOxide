use anyhow::Result;
use clap::Parser;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::Deserialize;
use serde_json::Value;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about = "Minimal Frigate CLI notifier")]
struct Args {
    #[arg(long, default_value = "localhost")] broker_host: String,
    #[arg(long, default_value_t = 1883)] broker_port: u16,
    #[arg(long, default_value = "frigate/#")] topic: String,
    #[arg(long)] verbose: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FrigateEventWrapper {
    #[serde(rename = "type")]
    event_type: Option<String>,
    before: Option<FrigateEventData>,
    after: FrigateEventData,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FrigateEventData {
    id: String,
    #[serde(default)]
    camera: Option<String>,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    current_zones: Vec<String>,
    #[serde(default)]
    entered_zones: Vec<String>,
    #[serde(default)]
    score: Option<f64>,
    #[serde(default)]
    #[serde(rename = "box")]
    bounding_box: Option<Vec<f64>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let mut mqttoptions = MqttOptions::new("frigate_cli", &args.broker_host, args.broker_port);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(&args.topic, QoS::AtLeastOnce).await?;

    println!("📡 Subscribed to {}:{} topic '{}'", args.broker_host, args.broker_port, args.topic);
    println!("🔍 Listening for Frigate events...\n");
    
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                let topic = p.topic.clone();
                
                // Show all topics if verbose
                if args.verbose {
                    println!("📬 Topic: {}", topic);
                }
                
                // Parse event data
                if topic == "frigate/events" || topic.starts_with("frigate/events/") {
                    if let Ok(wrapper) = serde_json::from_slice::<FrigateEventWrapper>(&p.payload) {
                        let ev = &wrapper.after;
                        let event_type = wrapper.event_type.as_deref().unwrap_or("event");
                        let label = ev.label.as_deref().unwrap_or("object");
                        let camera = ev.camera.as_deref().unwrap_or("unknown");
                        
                        let zones_info = if !ev.current_zones.is_empty() {
                            format!(" [zones: {}]", ev.current_zones.join(", "))
                        } else {
                            String::new()
                        };
                        
                        let score_info = if let Some(score) = ev.score {
                            format!(" (confidence: {:.1}%)", score * 100.0)
                        } else {
                            String::new()
                        };
                        
                        println!(
                            "🎯 {}: {} detected on {}{}{} [id: {}]",
                            event_type,
                            label,
                            camera,
                            score_info,
                            zones_info,
                            ev.id
                        );
                    } else if args.verbose {
                        // Show raw JSON if parsing fails
                        if let Ok(json) = serde_json::from_slice::<Value>(&p.payload) {
                            println!("⚠️  Unparseable event: {}", serde_json::to_string_pretty(&json).unwrap_or_default());
                        }
                    }
                } else if topic.contains("/detection") {
                    // Detection data (raw objects detected)
                    if let Ok(json) = serde_json::from_slice::<Value>(&p.payload) {
                        println!("🔎 Detection update: {}", topic);
                        if args.verbose {
                            println!("{}", serde_json::to_string_pretty(&json).unwrap_or_default());
                        }
                    }
                } else if args.verbose {
                    // Other topics (stats, etc)
                    if let Ok(text) = std::str::from_utf8(&p.payload) {
                        println!("📊 {}: {}", topic, text);
                    }
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("❌ MQTT error: {e}");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}