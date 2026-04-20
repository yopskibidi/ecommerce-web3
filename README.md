# 🛒 E-Commerce Smart Contract (Soroban)

A simple e-commerce smart contract built using Stellar Soroban. This contract allows users to manage products and perform purchase transactions on-chain.

---

## 📌 Description

This project demonstrates a basic implementation of an e-commerce system using Soroban smart contracts on the Stellar network.

It provides core functionalities such as:
- Product management
- Purchase transactions
- On-chain data storage

This project is intended for learning and early-stage blockchain development.

---

## ✨ Features

- 📦 Add and manage products
- 📄 View product list
- 🛍 Purchase products
- 📦 Store order history
- 🔐 Wallet-based authentication

---

## 🧱 Data Structures

### Product

```rust
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: u64,
    pub stock: u64,
}

ID smart contract = CAQTWQZZFXVYHW46M7SKCNNAW4LK3DNCJ4KSAPGAQZKSGKEZSAX3H5ZW