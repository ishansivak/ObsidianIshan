---
tags: [protocol]
---
# Saito Consensus and Operations

## Overview
Saito is a Layer 1 blockchain protocol incentivizing infrastructure funding and transaction scalability via a novel economic model.

### Core Mechanisms
- **Proof-of-Transaction**: Nodes are rewarded for facilitating transactions, aligning network security with real-world volume.
- **Economic Security**: Security is derived from the work performed in routing data, making disruption economically infeasible.
- **Scalability**: Decoupling transaction propagation from final validation maintains high throughput.

## UPI-Saito Settlement Layer
Architecture facilitating escrow-less, fiat-to-digital settlement using UPI-verified routing work.

### Settlement Lifecycle
1. **Payment**: Payer completes UPI transfer.
2. **Attestation**: Client broadcasts `PaymentProof`.
3. **Verification**: Gateway nodes verify UTR via APIs.
4. **Settlement**: Node broadcasts `RoutingProof`.
5. **Consensus**: Network validates, updates state, releases assets.

### Synergy
Functions as a bridge between physical infrastructure and biological signaling, mirroring nutrient distribution in [[Fungi and Mycelial Network Systems]].
