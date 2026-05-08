# Implementation: LLM Open Marketplace

## Overview
A decentralized marketplace for LLM inference where providers bid on performance-based service level agreements (SLAs), powered by the Saito network.

## Technical Stack
- **Inference Engine**: Leverage [[Ollama]] for local model serving.
- **Incentive/Routing Layer**: [[Saito Consensus]]. Instead of external smart contracts, inference requests are processed as transactions. The payment for the inference is routed through the network, rewarding the nodes that route the data and the inference provider.
- **Frontend**: **Next.js** dashboard for users to post requests (model, tokens/sec, latency).
- **Performance Verification**: Proof-of-Work/Proof-of-Routing. The network naturally incentivizes the node that provides the fastest results because it becomes the preferred route for that data.

## Key Challenges
- Mapping inference requests to Saito's routing model.
- Ensuring the inference provider is compensated directly via the transaction flow.

## Related Concept
- [[LLM Open Marketplace]]
