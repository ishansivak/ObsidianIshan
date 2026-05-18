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
- **Failure Recovery**: Gossip-based proof propagation; UTR registry prevents double-spend during partitions.
- **Monitoring**: Track usage in Google Cloud Console > APIs & Services > Billing.
