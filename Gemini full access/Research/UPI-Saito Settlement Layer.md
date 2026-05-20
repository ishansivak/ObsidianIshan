# UPI-Saito Settlement Layer
Design of the settlement layer connecting UPI to decentralized networks.

## Workflow
1. **Initiation**: User creates UPI transaction with encoded metadata.
2. **Attestation**: `PaymentProof` (UTR + sig) broadcast to gateway mesh.
3. **Verification**: Gateway nodes verify UTR via NPCI/Bank APIs.
4. **Settlement**: `RoutingProof` broadcast to [[Research/Saito Consensus|Saito Consensus]].

## Goal
Escrow-less fiat-to-digital settlement via UPI-verified Saito routing work.
