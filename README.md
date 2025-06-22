# 🧠 ICP CRUD DApp

A simple Decentralized Application (DApp) built using the Internet Computer Protocol (ICP) demonstrating **CRUD operations** via a backend canister written in **Rust** and a **React.js frontend**.

---

## 🚀 Features

- ✅ **Create**, **Read**, **Update**, and **Delete** items using canister functions.
- 🧠 Built on the Internet Computer using **Motoko SDK & Rust**.
- 🖥️ Frontend developed with **React + Vite**.
- 🔐 Secure communication via DFINITY’s `@dfinity/agent`.

---

## 🛠️ Tech Stack

| Layer     | Technology             |
|----------|------------------------|
| Backend  | Rust Canister (CDK)    |
| Frontend | React + Vite           |
| Agent    | @dfinity/agent         |
| Language | JavaScript, Rust       |
| Protocol | DFINITY SDK / IC       |

---

## 📂 Folder Structure

icp_dapp/
│
├── src/
│ ├── icp_dapp_backend/ # Rust backend canister
│ │ └── src/lib.rs
│ ├── icp_dapp_frontend/ # React frontend
│ │ └── src/App.jsx
│ └── declarations/ # Auto-generated bindings
│
├── dfx.json # DFINITY project config
├── package.json # Frontend dependencies
└── README.md # You're here!


---

## 💡 How to Run Locally

### 1️⃣ Prerequisites

- [Node.js](https://nodejs.org/)
- [DFINITY SDK](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [Rust](https://www.rust-lang.org/tools/install)

### 2️⃣ Install Dependencies

```bash
npm install
-
### 3️⃣ Start Local ICP Network
   dfx start --background
### 4️⃣ Deploy Canisters
    dfx deploy
###  5️⃣ Open the Frontend
Go to:
    http://localhost:4943/?canisterId=your_frontend_canister_id


-----
🧪 Backend Canister Functions
Function       	Description
create_item   	Add a new item to storage
read_item      	Retrieve item by ID
update_item   	Modify existing item
delete_item   	Remove item from storage

🏷️ Credits
Built by Bhawana Yadav as part of the ICP CRUD Weekend Challenge 🧠🚀

In collaboration with ICP Hub India and BlockseBlock
