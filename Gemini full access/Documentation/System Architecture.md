---
tags: [protocol, upi-saito, settlement]
aliases: ["UPI-Saito Protocol", "Settlement Architecture"]
---
# UPI-Saito Settlement Protocol
Escrow-less fiat-to-digital settlement via UPI-verified Saito routing work.

## 1. QR URI Schema
`upi://pay?pa={vpa_or_phone}&am={amt}&tr={nonce}&spk={saito_pubkey}&sig={payer_sig}`
- **pa**: Payee UPI handle or phone-mapped VPA.
- **tr**: 32-byte nonce (replay protection).
- **spk**: Payee Saito Public Key.
- **sig**: Payer signature (links UTR to identity).

## 2. Settlement Lifecycle
1. **Payment**: Payer completes UPI transfer.
2. **Attestation**: Client broadcasts `PaymentProof` (UTR + sig) to gateway mesh.
3. **Verification**: Gateway Nodes verify UTR via NPCI/Bank APIs.
4. **Settlement**: Node broadcasts `RoutingProof` to Saito chain.
5. **Consensus**: Network validates proof; updates state; releases assets.

## 3. Economics & Resilience
- **Incentives**: Node staking required. Fraud slashed. Rewards dynamic (volume/performance/capacity).
- **Failure Recovery**: 
    - **Network Partitions**: Gossip-based proof propagation ensures eventual consistency.
    - **UTR Registry**: Prevents double-spend and ensures uniqueness across distributed nodes.
    - **Operational Continuity**: See [[Documentation/Monitoring|Monitoring]] for real-time health checks and manual intervention protocols.

---
## Related
- [[Documentation/Monitoring|Monitoring]]
