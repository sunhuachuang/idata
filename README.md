# iData
Decentralized data/dataFi ecological. [Researching]

Inspired by [CympleTech's data protocol](https://github.com/CympleTech/data-protocol).

Purpose: Security & Privacy

## What it wants ?
- Let data **transfer** safely -> *Decentralized Transfer*
- Let data **storage**  safely -> *Decentralized Storage*
- Let data **exchange** safely -> *Decentralized Exchange*
- Let data **process**  safely -> *Decentralized Process*

### Design
- Must based on P2P network
- How to up and running it ( MUST NEED Incentives, [Why? TANSTAAFL!](https://en.wikipedia.org/wiki/There_ain%27t_no_such_thing_as_a_free_lunch) )
  - Proof of Services
    - Online for relay data quickly. (*proof of online*) **Basicly**
    - Compute for process data correctly. (*proof of compute*)
    - Storage for store data forever. (*proof of storage*)
  - Build blockchain and on-chain
    - Online for mining (generate)
    - Verifiable random functions & BFT for consensus
    - Zero-knowledge cuttable UTXO
    - Support multiple transaction types
      - anonymos token
      - transfer token
      - exchange data
      - storage data
      - process data

## Code structure
- Core Library (Self-balance / Self-transactions / Validate-consensus / Full-Miners-DHT)
- Full Node ( Transactions / Pools / Miner / Full-Consensus / Public Query )

## License

This project is licensed under

 * GNU GENERAL PUBLIC LICENSE, Version 3.0, [LICENSE](LICENSE)
 * Any question, please contact: contact@cympletech.com
