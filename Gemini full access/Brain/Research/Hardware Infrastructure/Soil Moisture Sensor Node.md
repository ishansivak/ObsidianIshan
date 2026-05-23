# Soil Moisture Sensor Node

## Overview
This note covers the design and implementation of soil moisture sensor nodes used for monitoring environmental parameters in agricultural or experimental settings. These nodes act as the physical interface between the digital protocol layer and the biological substrate, providing real-time telemetry on soil health.

## Core Components
- **Sensor Mechanism**: High-precision capacitive soil moisture sensors are preferred over resistive models to prevent electrode corrosion and ensure long-term stability in diverse soil compositions.
- **Controller Architecture**: The ESP32 is utilized for its low-power deep-sleep modes, essential for long-term field deployment, and integrated Wi-Fi/Bluetooth capabilities for localized edge computing.
- **Power Management**: A solar-harvesting circuit paired with a LiPo battery management system (BMS) ensures autonomous, continuous operation, critical for remote sensor arrays.

## Data Integration & Protocol Mapping
The data captured by these nodes is ingested by the [[Protocol Operations]] framework. By mapping environmental moisture data against the activity patterns observed in [[Fungi as Biological Networks]], we can model the correlation between soil moisture levels and mycelial growth rates. This creates a feedback loop where physical infrastructure informs the biological network model, which is then validated by the protocol layer.

## Future Development
Future iterations will focus on increasing sensor density and integrating multi-parameter probes to measure soil temperature and electrical conductivity, providing a more comprehensive environmental dataset.

---
### Links
- Root Index: [[Infrastructure Index]]
- Biological Context: [[Biological Index]], [[Fungi as Biological Networks]]
- Protocol Context: [[Protocol Index]], [[Protocol Operations]]
