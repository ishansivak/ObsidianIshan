# Blackbox: Saito Consensus & Economic Mechanics

Saito is a next-generation Layer 1 blockchain that solves the economic "free-rider" problem of traditional networks (PoW/PoS) by aligning protocol incentives with the physical work of routing transactions and running user-facing infrastructure.

---

## 1. The Economic Market Failure
In Proof of Work and Proof of Stake, the protocol only pays for mining or staking. Nodes that run user-facing infrastructure, collect transactions, and propagate data receive no protocol-level compensation, leading to centralization (e.g., reliance on centralized APIs like Infura). Saito replaces these models with **Proof of Transactions (PoT)** driven by **Routing Work**.

---

## 2. Routing Work & The Decay Formula
Routing work is a cryptographic measure of a node's contribution to collecting and propagating transactions.
* **Cryptographic Routing Signatures:** When a user sends a transaction, they sign it and designate a first-hop node. As the transaction is relayed, each hop adds its signature, creating an unforgeable path from user to block producer.
* **The Decay Formula:** To incentivize nodes to connect directly to end-users (the fee source), routing work mathematically halves with each hop beyond the first.
    * *Example:* A transaction with a **10 SAITO** fee represents:
        * **1st-hop (User-facing):** 10 units of routing work.
        * **2nd-hop:** 5 units.
        * **3rd-hop:** 2.5 units, and so on.

---

## 3. Block Production & The Burn Fee
Block production difficulty is represented by a dynamic, token-denominated cost called the **Burn Fee**.
* **Logarithmic Decay:** Immediately after a block is produced, the Burn Fee is set extremely high to prevent instant spam blocks. Over time, it decays logarithmically.
* **Block Creation:** A node can propose a block only when the total accumulated routing work in its mempool (fees adjusted for hops) exceeds the current decaying Burn Fee.
* **The Fee Burn:** Once a block is produced, all transaction fees inside it are burned (locked away), preventing attackers from using one block's fees to instantly fund the next.

---

## 4. The Golden Ticket System (Payouts)
To prevent network deflation and distribute the burned fees, the protocol uses the **Golden Ticket** system:
1. **Hashing Puzzle:** Miners solve a hashing puzzle based on the previous block.
2. **The 50/50 Split:** The solution (Golden Ticket) is included in the next block, unlocking the burned fees:
    * **50% to the Miner** who solved the puzzle.
    * **50% to a Routing Node** from the previous block.
3. **Weighted Lottery:** The routing node is chosen randomly but is heavily weighted by the routing work performed. The protocol selects a transaction (weighted by fee size) and then a hop on its path (weighted by routing work contributed).

---

## 5. Automatic Transaction Rebroadcasting (ATR)
To prevent ledger bloat and support high-throughput data processing:
* Saito operates on a **circular ledger** divided into epochs. Data falling off the end of an epoch is pruned.
* If a user wants to keep a UTXO or data alive, the protocol automatically rebroadcasts it into the current epoch, charging an **ATR rebroadcasting fee**.
* These fees flow back into the mempool as fresh routing work, creating a self-sustaining loop where users pay only for the storage they actively consume.
