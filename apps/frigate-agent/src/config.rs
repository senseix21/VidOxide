use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub broker_host: String,
    pub broker_port: u16,
    pub topic: String,
    pub frigate_base: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            broker_host: env::var("BROKER_HOST").unwrap_or_else(|_| "localhost".into()),
            broker_port: env::var("BROKER_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(1883),
            topic: env::var("MQTT_TOPIC").unwrap_or_else(|_| "frigate/events".into()),
            frigate_base: env::var("FRIGATE_BASE").unwrap_or_else(|_| "http://localhost:5000".into()),
        }
    }
}