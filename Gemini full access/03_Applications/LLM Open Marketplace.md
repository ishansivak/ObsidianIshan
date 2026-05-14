---
tags:
  - product
  - llm
  - marketplace
  - saito
---
# LLM Open Marketplace

A decentralized marketplace for LLM inference where providers bid on performance-based service level agreements (SLAs), powered by the Saito network.

## Core Logic
### Supply Side (Providers)
- **Local Inference**: Users run models locally (e.g., via [[Ollama]]).
- **API Integration**: Providers connect via API keys (OpenAI, Anthropic, Google) to fulfill requests.
- **Performance Specs**: Providers set price, token speed, and model availability.

### Demand Side (Consumers)
- **Ad Monetization**: Users watch ads via [[AADS]] to earn Saito for tokens.
- **Direct Purchase**: Users acquire Saito directly to purchase tokens.

## Technical Stack
- **Inference Engine**: [[Ollama]].
- **Incentive/Routing Layer**: [[Saito Consensus]]. Requests are processed as transactions; payment is routed through the network, rewarding routing nodes and the provider.
- **Frontend**: Next.js dashboard for posting requests (model, tokens/sec, latency).
- **Performance Verification**: Network incentivizes nodes providing the fastest results by routing more data to them.

See also: [[Mesh Networks Overview]]
