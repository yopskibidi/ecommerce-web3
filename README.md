# 🛒 E-Commerce Smart Contract (Soroban)

Smart contract sederhana berbasis Stellar Soroban untuk mengelola sistem e-commerce seperti produk dan transaksi pembelian.

---

## 📌 Deskripsi

Project ini merupakan implementasi smart contract e-commerce menggunakan Soroban (Stellar). Contract ini memungkinkan pengguna untuk:

- Menambahkan produk
- Melihat daftar produk
- Melakukan pembelian
- Menyimpan data order

Contract ini masih dalam bentuk sederhana dan digunakan untuk pembelajaran serta pengembangan awal aplikasi berbasis blockchain.

---

## ✨ Fitur

- 📦 Product Management
- 🛍 Order / Checkout System
- 🔐 Wallet Authentication
- 📊 Storage berbasis blockchain

---

## 🧱 Struktur Data

### Product

```rust
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: u64,
    pub stock: u64,
}

ID smart contract = CAQTWQZZFXVYHW46M7SKCNNAW4LK3DNCJ4KSAPGAQZKSGKEZSAX3H5ZW