# Reticulum

Reticulum is a media-agnostic, cryptographic networking stack.

## Core Architecture
- **Routing**: Destination-based using cryptographic identifiers (16-byte hashes).
- **Topology**: Heterogeneous, connecting various physical layers (LoRa, Ethernet, Wi-Fi, I2P).
- **Security**: Mandatory encryption using X25519 and Ed25519; Perfect Forward Secrecy.
- **Use Cases**: Trustless, sovereign, planetary-scale networks.

## Key Features
- **Announce Mechanism**: Highly optimized path discovery with bandwidth caps (default 2%).
- **RNode**: Firmware that turns ESP32/SX1262 into flexible radio modems for the Reticulum stack.
