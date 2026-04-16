# ⭐ Stellar Tip Jar (Soroban dApp)

A decentralized tip jar smart contract built on the Stellar Soroban platform. This project enables users to send tips, store messages, and track all tipping activity directly on-chain without requiring a frontend.

---

## 🚀 Overview

Stellar Tip Jar is a backend-only decentralized application (dApp) that runs entirely on Soroban. It demonstrates how smart contracts can be used to manage transactions, store structured data, and provide analytics such as total tips, user contributions, and creator earnings.

This project is designed for:

* Learning Soroban smart contract development
* Demonstrating Web3 backend architecture
* Building a strong portfolio project

---

## ✨ Features

* 💸 Send tips between users (simulated on-chain)
* 🧾 Store tip messages (like “saweran”)
* 📊 Track total tips sent
* 🔢 Count number of transactions
* 👤 Track total tips per user
* 🎯 Track total earnings per creator
* 📜 Retrieve full tipping history
* ⏱️ Timestamp for every transaction
* 🔄 Reset contract state (for testing)

---

## 🧱 Tech Stack

* **Language:** Rust
* **Framework:** Soroban SDK
* **Blockchain:** Stellar (Soroban Testnet)
* **Environment:** Soroban Studio

---

## 📁 Project Structure

```
contract/
└── src/
    └── lib.rs   # Main smart contract
```

---

## ⚙️ Smart Contract Functions

### 🔹 `tip(from, to, amount, message)`

Send a tip from one address to another.

### 🔹 `get_total()`

Returns total amount of all tips.

### 🔹 `get_count()`

Returns total number of tip transactions.

### 🔹 `get_all_tips()`

Returns all tip history.

### 🔹 `get_tip(index)`

Get a specific tip by index.

### 🔹 `get_user_total(user)`

Get total tips sent by a specific user.

### 🔹 `get_creator_total(creator)`

Get total tips received by a creator.

### 🔹 `reset()`

Reset all stored data (for testing purposes).

---

## 🧪 How to Use (Soroban Studio)

1. Open Soroban Studio
2. Create a new project
3. Replace `lib.rs` with this contract code
4. Click **Deploy → Testnet**
5. Copy your Contract ID

### ▶️ Testing the Contract

Call the `tip` function with:

* `from`: your wallet address
* `to`: creator address
* `amount`: example `100`
* `message`: `"Great work!"`

Then test:

* `get_total()` → total tips
* `get_count()` → number of tips
* `get_all_tips()` → full history

---

## 📌 Example Output

```
{
  from: GABC...,
  to: GXYZ...,
  amount: 100,
  message: "Great work!",
  timestamp: 1712345678
}
```

---

## ⚠️ Limitations

* ❌ Does not transfer real XLM yet (simulation only)
* ❌ No frontend interface (interact via Soroban Studio)
* ❌ No wallet integration (Freighter not required)

---

## 🚀 Future Improvements

* 🔗 Integrate real XLM/token transfer
* 🌐 Add frontend (React / Web3 UI)
* 🔐 Wallet integration (Freighter)
* 📈 Advanced analytics dashboard
* 🧠 Smart tipping logic (fees, rewards, etc.)

---

## 📖 Learning Outcomes

This project demonstrates:

* Smart contract development using Soroban
* On-chain data storage (Vec, Map)
* Handling authentication (`require_auth`)
* Structuring a real-world dApp backend
* Designing scalable contract logic

---

## 📜 License

This project is open-source and available under the MIT License.

---

## 🙌 Acknowledgements

Built using the Stellar Soroban ecosystem. Special thanks to the Stellar developer community.
