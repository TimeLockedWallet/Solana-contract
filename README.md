# Time Lock Wallet ⏳💰  
Dự án ví khóa thời gian trên Solana. Chương trình được viết bằng **Anchor** và có frontend React để tương tác.  
Người dùng có thể khóa SOL hoặc token SPL cho đến một thời điểm xác định. Sau khi đến hạn, họ mới có thể rút tiền.  

---

## 🚀 Tính năng
- ✅ **initialize** — Khởi tạo cấu hình chương trình.  
- ✅ **initializeLock(amount, unlock_timestamp, isSol)** — Tạo ví khóa tiền với số lượng và thời gian mở khóa.  
- ✅ **withdraw()** — Rút tiền sau khi đến thời gian mở khóa.  
- ✅ **Frontend React** — Giao diện đơn giản kết nối với Phantom.  
- ✅ **Hỗ trợ trên Devnet**  

---

## ⚙️ Cài đặt môi trường

### 1. Cài Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
rustc --version
rustup default 1.83.0
```
### 2. Cài Solana CLI
```bash
sh -c "$(curl -sSfL https://release.anza.xyz/v1.18.21/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
solana --version
```
Tạo ví mới và chuyển sang Devnet:
```bash
solana-keygen new
solana config set -u https://api.devnet.solana.com
solana airdrop 5
```
### 3. Cài Anchor
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.30.1
avm use 0.30.1
anchor --version
```
## Smart Contract 
### Clone repo 
```bash
git clone https://github.com/TimeLockedWallet/Solana-contract.git
cd time_locked_wallet
```
### Build và Deploy
```bash
anchor build
anchor deploy
```
Program ID sẽ được sinh ra trong:
```bash
target/deploy/time_lock_wallet-keypair.json
```
Cập nhật Anchor.toml và lib.rs với Program ID này
