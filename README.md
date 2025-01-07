# **CaelumX**  

## **Overview**  
CaelumX is a decentralized carbon credit marketplace built on the Solana blockchain. The platform integrates bonding curves for dynamic token pricing, enables NFT-to-token conversions, and provides a gamified trading experience inspired by Pump.fun.  

CaelumX aims to tokenize carbon credits, allowing users to trade, stake, and convert NFTs into tokens seamlessly, promoting sustainable actions through blockchain innovation.  

---

## **Features**  
- **Bonding Curve Mechanism**:  
  - Dynamic pricing of tokens based on supply and demand.  
  - Liquidity managed through reserve pooling.  

- **NFT-to-Token Conversion**:  
  - Users can convert their tokenized carbon credit NFTs into fractional tokens.  

- **Trading Platform**:  
  - Gamified interface for token staking and trading.  
  - Inspired by the Pump.fun platform for user engagement.  

- **Sustainability Focus**:  
  - Supports carbon offsetting by tokenizing carbon credits.  

---

## **Folder Structure**  

```plaintext  
caelumx-project/  
├── contracts/  
│   ├── bonding_curve/             # Bonding curve contract logic  
│   ├── nft_conversion/            # NFT-to-token conversion contract logic  
│   ├── shared/                    # Reusable utilities and constants  
├── frontend/  
│   ├── components/                # UI components for frontend  
│   ├── pages/                     # Frontend pages (e.g., trade, convert)  
│   ├── utils/                     # API and utility functions  
│   ├── styles/                    # Frontend styles  
├── migrations/  
│   ├── supabase/                  # Database schema for Supabase integration  
├── scripts/  
│   ├── deploy.ts                  # Deployment scripts for contracts  
│   ├── fund_reserve.ts            # Script to initialize bonding curve reserve  
│   ├── mint_nfts.ts               # Script to mint NFTs  
├── tests/  
│   ├── integration/               # Integration tests for contracts  
├── README.md                      # Project documentation  
├── .env                           # Environment variables  
├── Anchor.toml                    # Anchor configuration  
