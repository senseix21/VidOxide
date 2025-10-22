use anyhow::Result;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tokio::time::interval;

#[derive(Debug, Deserialize)]
struct FrigateEventWrapper {
    #[serde(rename = "type")]
    event_type: Option<String>,
    after: FrigateEventData,
}

#[derive(Debug, Deserialize)]
struct FrigateEventData {
    id: String,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    camera: Option<String>,
}

#[derive(Debug, Serialize)]
struct NoDetectionMessage {
    status: String,
    camera: String,
    message: String,
    timestamp: u64,
    last_detection: Option<String>,
}

type ActiveDetections = Arc<RwLock<HashSet<String>>>;
type LastDetectionTime = Arc<RwLock<Option<SystemTime>>>;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Detection Monitor starting...");
    
    let broker_host = std::env::var("BROKER_HOST").unwrap_or_else(|_| "localhost".into());
    let broker_port: u16 = std::env::var("BROKER_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1883);
    let camera = std::env::var("CAMERA_NAME").unwrap_or_else(|_| "demo".into());
    
    let mut mqttoptions = MqttOptions::new("detection_monitor", &broker_host, broker_port);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    
    // Shared state
    let active_detections: ActiveDetections = Arc::new(RwLock::new(HashSet::new()));
    let last_detection_time: LastDetectionTime = Arc::new(RwLock::new(None));
    
    // Clone for tasks
    let active_detections_clone = active_detections.clone();
    let last_detection_time_clone = last_detection_time.clone();
    let client_clone = client.clone();
    let camera_clone = camera.clone();
    
    // Task 1: Monitor MQTT events
    tokio::spawn(async move {
        // Subscribe after connection
        tokio::time::sleep(Duration::from_secs(2)).await;
        client.subscribe("frigate/events", QoS::AtLeastOnce).await.ok();
        println!("📡 Subscribed to frigate/events");
        
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    if p.topic == "frigate/events" {
                        if let Ok(wrapper) = serde_json::from_slice::<FrigateEventWrapper>(&p.payload) {
                            let event_type = wrapper.event_type.as_deref().unwrap_or("unknown");
                            let obj_id = wrapper.after.id.clone();
                            let label = wrapper.after.label.as_deref().unwrap_or("object");
                            
                            let mut detections = active_detections.write().await;
                            let mut last_time = last_detection_time.write().await;
                            
                            match event_type {
                                "new" | "update" => {
                                    detections.insert(obj_id.clone());
                                    *last_time = Some(SystemTime::now());
                                    println!("✅ Detection active: {} ({})", label, obj_id);
                                }
                                "end" => {
                                    detections.remove(&obj_id);
                                    println!("❌ Detection ended: {} ({})", label, obj_id);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("❌ MQTT error: {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });
    
    // Task 2: Publish "no detection" every 2 seconds
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(2));
        
        loop {
            interval.tick().await;
            
            let detections = active_detections_clone.read().await;
            let last_time = last_detection_time_clone.read().await;
            
            if detections.is_empty() {
                let last_detection = if let Some(time) = *last_time {
                    let elapsed = SystemTime::now()
                        .duration_since(time)
                        .unwrap_or(Duration::from_secs(0));
                    Some(format!("{} seconds ago", elapsed.as_secs()))
                } else {
                    Some("never".to_string())
                };
                
                let msg = NoDetectionMessage {
                    status: "idle".to_string(),
                    camera: camera_clone.clone(),
                    message: "No objects detected".to_string(),
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    last_detection,
                };
                
                let payload = serde_json::to_string(&msg).unwrap();
                let topic = format!("frigate/{}/detection_status", camera_clone);
                
                client_clone
                    .publish(&topic, QoS::AtLeastOnce, false, payload.as_bytes())
                    .await
                    .ok();
                    
                println!("🔇 No detections - published status");
            } else {
                println!("🎯 Active detections: {}", detections.len());
            }
        }
    });
    
    // Keep main task alive
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
