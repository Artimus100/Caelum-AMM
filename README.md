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

caelumx-project/
├── programs/
│   ├── bonding_curve/
│   │   ├── src/
│   │   │   ├── lib.rs             # Anchor program for bonding curve logic
│   │   │   ├── error.rs           # Custom errors for bonding curve
│   │   │   ├── instructions/      # Instruction handlers
│   │   │   │   ├── initialize.rs  # Initialize bonding curve state
│   │   │   │   ├── mint.rs        # Mint tokens using bonding curve
│   │   │   │   ├── update.rs      # Update bonding curve parameters
│   │   │   ├── state.rs           # Program accounts for bonding curve
│   │   │   ├── tests.rs           # Unit tests for bonding curve logic
│   │   └── Anchor.toml            # Anchor configuration for bonding curve
│   ├── nft_conversion/
│   │   ├── src/
│   │   │   ├── lib.rs             # Anchor program for NFT-to-token conversion
│   │   │   ├── error.rs           # Custom errors for NFT conversion
│   │   │   ├── instructions/      # Instruction handlers
│   │   │   │   ├── convert.rs     # Handle NFT-to-token conversion
│   │   │   │   ├── verify.rs      # NFT metadata verification
│   │   │   ├── state.rs           # Program accounts for NFT conversion
│   │   │   ├── tests.rs           # Unit tests for NFT conversion logic
│   │   └── Anchor.toml            # Anchor configuration for NFT conversion
│   └── shared/
│       ├── src/
│       │   ├── constants.rs       # Shared constants (e.g., curve parameters)
│       │   ├── utils.rs           # Common utilities (e.g., math functions)
│       └── Cargo.toml             # Shared library for reusable components
├── migrations/
│   ├── bonding_curve/
│   │   ├── deploy.ts              # Deployment script for bonding curve
│   │   ├── fund_reserve.ts        # Script to fund bonding curve reserve
│   ├── nft_conversion/
│   │   ├── deploy.ts              # Deployment script for NFT conversion
│   │   ├── mint_nfts.ts           # Script to mint demo NFTs
├── app/
│   ├── src/
│   │   ├── components/
│   │   │   ├── BondingCurveChart.tsx    # Chart for bonding curve visualization
│   │   │   ├── NFTConversionForm.tsx   # Form for NFT-to-token conversion
│   │   │   ├── TradeInterface.tsx      # Pump.fun-like trading interface
│   │   ├── pages/
│   │   │   ├── index.tsx               # Homepage
│   │   │   ├── trade.tsx               # Main trading page
│   │   │   ├── convert.tsx             # NFT-to-token conversion page
│   │   ├── utils/
│   │   │   ├── api.ts                  # API for smart contract interaction
│   │   │   ├── bondingCurve.ts         # Frontend logic for bonding curve calculations
│   │   ├── styles/                     # CSS or Tailwind styles
│   │   │   ├── globals.css
│   │   └── App.tsx                     # Main React application
│   └── package.json                    # Frontend dependencies
├── tests/
│   ├── integration/
│   │   ├── bonding_curve_test.ts       # Integration tests for bonding curve
│   │   ├── nft_conversion_test.ts      # Integration tests for NFT conversion
├── supabase/
│   ├── schemas/
│   │   ├── wallets.sql                 # Wallet schema
│   │   ├── trade_history.sql           # Trade history schema
│   │   ├── nft_metadata.sql            # NFT metadata schema
├── scripts/
│   ├── anchor_init.sh                  # Initialize Anchor project
│   ├── deploy_all.sh                   # Deploy all programs
│   ├── populate_demo_data.ts           # Populate demo NFTs and accounts
├── migrations/
│   ├── deploy.ts                       # Deployment for all modules
├── README.md                           # Project documentation
├── .env                                # Environment variables
├── .gitignore                          # Ignore unnecessary files
├── Anchor.toml                         # Root Anchor configuration
├── Cargo.toml                          # Root Cargo configuration
                # Anchor configuration  
