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
    
    let client_id = format!("frigate_cli_{}", std::process::id());
    let mut mqttoptions = MqttOptions::new(&client_id, &args.broker_host, args.broker_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    mqttoptions.set_clean_session(true);
    
    if args.verbose {
        println!("🔧 Client ID: {}", client_id);
        println!("🔧 Broker: {}:{}", args.broker_host, args.broker_port);
    }
    
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 100);
    
    println!("🔌 Connecting to MQTT broker at {}:{}...", args.broker_host, args.broker_port);
    
    let mut connected = false;
    let mut message_count = 0u64;
    
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                if args.verbose {
                    println!("✅ Connected to MQTT broker");
                } else {
                    println!("✅ Connected to MQTT broker");
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
                client.subscribe(&args.topic, QoS::AtMostOnce).await?;
                tokio::time::sleep(Duration::from_millis(500)).await;
                if args.verbose {
                    println!("📡 Subscribed to topic '{}' with QoS 0", args.topic);
                } else {
                    println!("📡 Subscribed to topic '{}'", args.topic);
                }
                println!("🔍 Listening for Frigate events...\n");
                connected = true;
            }
            Ok(Event::Incoming(Packet::Publish(p))) => {
                message_count += 1;
                if !connected {
                    connected = true;
                }
                let topic = p.topic.clone();
                let payload_size = p.payload.len();
                
                // Show all topics if verbose
                if args.verbose {
                    println!("📬 Topic: {} ({} bytes)", topic, payload_size);
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
                        } else {
                            println!("⚠️  Non-JSON payload on {}", topic);
                        }
                    }
                } else if topic.contains("/detection_status") {
                    // No detection status messages
                    if let Ok(json) = serde_json::from_slice::<Value>(&p.payload) {
                        if let Some(status) = json.get("status").and_then(|v| v.as_str()) {
                            if status == "idle" {
                                let camera = json.get("camera").and_then(|v| v.as_str()).unwrap_or("unknown");
                                let last = json.get("last_detection").and_then(|v| v.as_str()).unwrap_or("never");
                                println!("💤 {} - No objects detected (last: {})", camera, last);
                            }
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
                        println!("📊 {}: {}", topic, text.trim());
                    } else {
                        println!("📊 {} [binary: {} bytes]", topic, payload_size);
                    }
                }
            }
            Ok(Event::Incoming(Packet::PingResp)) => {
                if args.verbose {
                    println!("💓 Keepalive (msgs: {})", message_count);
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("❌ MQTT error: {e}");
                connected = false;
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}