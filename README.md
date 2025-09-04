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
Cập nhật Anchor.toml và lib.rs với Program ID này, sau đó chạy lệnh sau để test từng hàm :
```bash
npm run test
```
## Giải thích các hàm 
### 1. Khởi tạo cấu hình 
```bash
it("Is initialized!", async () => {
  const tx = await program.methods
    .initialize()
    .accountsPartial({
      config: configPda,
      authority: authority.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log("Transaction signature", tx);
});
```
### 2. Tạo ví khóa tiền
Ví dụ: Khóa 1 USDC trong 60 giây.
Tham số 
**unlockTimestamp (BN)** : Thời gian mở khoá token
**recipient (address)** : Địa chỉ của người nhận 
**amount (BN)** : Số tiền deposit 
**isSol(boolean)** : true nếu gửi Sol và false nếu gửi token SPL(như USDC)
```bash
it("Should successfully initialize the lock!", async () => {
    const unlockTimestamp = Math.floor(Date.now() / 1000) + 60;
    const amount = new BN(1_000_000); 
    const isSol = false; // nếu khóa SOL thì để true, SPL token thì false
    const tx = await program.methods.
      initializeLock(
        new BN(unlockTimestamp),
        authority.publicKey,
        amount,
        isSol
    ).accountsPartial({
        vault : vaultPda,
        bankVault : bankVaultPda,
        userTokenAta : userUsdcAta,
        bankVaultTokenAta : bankVaultUsdcAta,
        user : authority.publicKey,
        tokenMint : TOKENS.usdcMint,
        associatedTokenProgram : anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
    }).rpc();
    console.log("Your transaction signature", tx);
  });
```
### 3. Rút tiền
Chỉ rút được khi thời gian mở khóa đã đến.
```bash
it("Withdraws funds successfully", async () => {
  const tx = await program.methods
    .withdraw()
    .accountsPartial({
      vault: vaultPda,
      bankVault: bankVaultPda,
      bankVaultTokenAta: bankVaultUsdcAta,
      recipientTokenAta: userUsdcAta,
      owner: authority.publicKey,
      tokenMint: TOKENS.usdcMint,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .rpc();
  console.log("Transaction signature", tx);
});
```
