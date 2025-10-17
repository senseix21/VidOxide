```
[FFmpeg] --RTSP--> [MediaMTX :8554/demo] --RTSP--> [Frigate]
                                     |            ├─ MQTT events → broker
                                     |            └─ UI/API :5000
                                     └─ (optional) HLS/WebRTC for browsers
```
Rust CLI subscribes to `frigate/events` and prints notifications.