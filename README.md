# Time Lock Wallet ⏳💰  
A Solana smart contract (using Anchor) + Frontend app that allows you to lock SOL until a specific timestamp. Once the unlock time is reached, you can withdraw your funds back.  

---

## Features
- ✅ **initialize_lock(amount, unlock_timestamp)** — Create a time lock wallet with SOL and lock until `unlock_timestamp`.  
- ✅ **withdraw()** — Withdraw locked SOL after unlock time.  
- ✅ **Funds stored in PDA** — Program Derived Address securely holds locked SOL.  
- ✅ **On-chain enforced lock** — Withdrawal fails if attempted too early.  
- ✅ **Frontend** — Simple React app with Phantom/Backpack wallet integration.  
- ✅ **Devnet support** — Deploy and test using Solana Devnet.  

---

# Part One - Environment Setup ⚙️
Before writing or deploying smart contracts on Solana, we need to set up the development environment.

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
rustc --version
rustup default 1.83.0
