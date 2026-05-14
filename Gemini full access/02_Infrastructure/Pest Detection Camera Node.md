---
tags: [agritech, iot, computer-vision]
---
# Pest Detection Camera Node

An edge-computing camera module designed to identify agricultural pests and plant infections in real-time.

## Connectivity
- **Radio**: [[LoRa]] (integrated with [[Meshtastic]] or [[MeshCore]])
- **Communication**: Transmits alerts and low-resolution thumbnails over the [[Mesh Networking]] backbone.

## Features
- **Detection**: On-device image processing to detect common pests and leaf diseases.
- **Alerting**: Pushes notifications to a central management dashboard when threats are identified.
- **Power**: Solar panel with high-capacity battery for remote deployment.

## Integration
- Can be triggered to take photos by other sensors (e.g., when a moisture drop is detected).
- Part of the wider [[Mesh Networking]] agricultural monitoring ecosystem.

See also: [[Soil Moisture Sensor Node]]
