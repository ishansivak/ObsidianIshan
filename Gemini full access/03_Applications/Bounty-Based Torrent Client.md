---
tags:
  - product
  - p2p
  - torrent
  - saito
---
# Bounty-Based Torrent Client

A P2P torrent client that incentivizes the seeding of rare or specific files by allowing users to post bounties for seeders.

## Concept
Seeders are rewarded for keeping rare files available until bounty requirements are met.

## Technical Stack
- **Core Engine**: libtorrent (C++).
- **Incentive Layer**: [[Saito Consensus]]. The bounty is a recurring payment for the "routing" of the file data. Seeders earn by providing the data routed to the requester.
- **Verification**: Client verifies data chunks against the torrent info hash. Payment is triggered as chunks are received/routed.
- **Frontend**: Electron or browser-based.

## Key Challenges
- Integrating torrent chunking with Saito’s payment channel mechanism.
- Ensuring the seeder's contribution is correctly credited as routing work.

See also: [[Mesh Networks Overview]]
