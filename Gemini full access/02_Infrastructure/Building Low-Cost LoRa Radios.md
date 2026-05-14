# Building Low-Cost LoRa Radios

Engineering efficient, off-grid mesh nodes using the ESP32 and SX1262.

## Hardware Selection
- **Microcontroller**: ESP32 (specifically ESP32-S3) for its balance of power, processing, and connectivity.
- **Transceiver**: Semtech SX1262 for superior RF performance and lower power consumption compared to older modules like the RFM95.

## Wiring & Integration
- Interfaces via SPI bus.
- Crucial pins: BUSY (for command execution status), DIO1 (for interrupts), NRST (reset), DIO3 (TCXO control).

## Power Optimization
- Throttle CPU frequency (e.g., 80 MHz).
- Use ESP32 hardware deep sleep, waking only via DIO1 interrupts when a packet is received.
- This creates perpetual, low-power nodes.
