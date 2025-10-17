use rumqttc::{Client, Connection, Event, MqttOptions, Packet, QoS};
use std::time::Duration;

pub fn connect(host: &str, port: u16, client_id: &str) -> (Client, Connection) {
    let mut opts = MqttOptions::new(client_id, host, port);
    opts.set_keep_alive(Duration::from_secs(30));
    rumqttc::Client::new(opts, 10)
}

pub fn subscribe(client: &mut Client, topic: &str) {
    client.subscribe(topic, QoS::AtLeastOnce).expect("subscribe failed");
}

pub fn next_publish(conn: &mut Connection) -> Option<Vec<u8>> {
    match conn.iter().next()? {
        Ok(Event::Incoming(Packet::Publish(p))) => Some(p.payload.to_vec()),
        Ok(_) => None,
        Err(_) => None,
    }
}